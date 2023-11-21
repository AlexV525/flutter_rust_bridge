use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::pack::IrStructPool;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::source_graph::modules::StructOrEnumWrapper;
use crate::codegen::parser::type_parser::unencodable::{
    parse_path_type_to_unencodable, SplayedSegment,
};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use syn::{Ident, TypePath};

pub(super) trait EnumOrStructParser<Id, Obj, SrcObj, Item>
where
    Id: From<NamespacedName> + Clone + PartialEq + Eq + Hash,
    SrcObj: StructOrEnumWrapper<Item> + Clone,
{
    fn parse(
        &mut self,
        type_path: &TypePath,
        splayed_segments: &[SplayedSegment],
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        if let (name, _) = last_segment {
            if let Some(src_object) = self.src_objects().get(*name) {
                let src_object = (*src_object).clone();

                let namespace = Namespace::new(src_object.inner().path.clone());
                let namespaced_name = NamespacedName::new(namespace, name.to_string());
                let ident: Id = namespaced_name.clone().into();

                if (self.parser_info().parsing_or_parsed_objects).insert(namespaced_name.clone()) {
                    let (name, wrapper_name) = compute_name_and_wrapper_name(
                        &namespaced_name.namespace,
                        &src_object.inner().ident,
                        src_object.inner().mirror,
                    );

                    match self.parse_inner(&src_object, name, wrapper_name)? {
                        Some(parsed_object) => {
                            (self.parser_info().object_pool).insert(ident.clone(), parsed_object)
                        }
                        None => {
                            return Ok(Some(parse_path_type_to_unencodable(
                                type_path,
                                splayed_segments,
                            )))
                        }
                    };
                }

                return Ok(Some(self.construct_output(ident)?));
            }
        }

        Ok(None)
    }

    fn parse_inner(
        &mut self,
        src_object: &SrcObj,
        name: NamespacedName,
        wrapper_name: Option<NamespacedName>,
    ) -> anyhow::Result<Option<Obj>>;

    fn construct_output(&self, ident: Id) -> anyhow::Result<IrType>;

    fn src_objects(&self) -> &HashMap<String, &SrcObj>;

    fn parser_info(&mut self) -> &mut EnumOrStructParserInfo<Id, Obj>;
}

#[derive(Clone, Debug, Default)]
pub(super) struct EnumOrStructParserInfo<Id, Obj> {
    parsing_or_parsed_objects: HashSet<NamespacedName>,
    pub(super) object_pool: HashMap<Id, Obj>,
}

impl<Id, Obj> EnumOrStructParserInfo<Id, Obj> {
    pub fn new() -> Self {
        Self {
            parsing_or_parsed_objects: HashSet::new(),
            object_pool: HashMap::new(),
        }
    }
}

fn compute_name_and_wrapper_name(
    namespace: &Namespace,
    ident: &Ident,
    mirror: bool,
) -> (NamespacedName, Option<NamespacedName>) {
    let name = ident.to_string();
    let wrapper_name = if mirror {
        Some(format!("mirror_{name}"))
    } else {
        None
    };
    (
        NamespacedName::new(namespace.clone(), name),
        wrapper_name.map(|x| NamespacedName::new(namespace.clone(), x)),
    )
}
