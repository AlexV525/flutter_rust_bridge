use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::record::IrTypeRecord;
use crate::codegen::ir::ty::structure::{IrStruct, IrStructIdent, IrTypeStructRef};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::library::codegen::ir::ty::IrTypeTrait;
use anyhow::Result;
use itertools::Itertools;
use syn::TypeTuple;

impl<'a> TypeParserWithContext<'a> {
    pub(crate) fn parse_type_tuple(&mut self, type_tuple: &TypeTuple) -> anyhow::Result<IrType> {
        if type_tuple.elems.is_empty() {
            return Ok(Primitive(IrTypePrimitive::Unit));
        }

        let namespace = self.context.initiated_namespace.clone();

        let values = type_tuple
            .elems
            .iter()
            .map(|elem| self.parse_type(elem))
            .collect::<Result<Vec<_>>>()?;
        let safe_ident = values
            .iter()
            .map(IrType::safe_ident)
            .collect_vec()
            .join("_");
        let safe_ident = format!("__record__{safe_ident}");
        self.inner.struct_parser_info.object_pool.insert(
            IrStructIdent(NamespacedName::new(namespace.clone(), safe_ident.clone())),
            IrStruct {
                name: NamespacedName::new(namespace.clone(), safe_ident.clone()),
                wrapper_name: None,
                path: None,
                is_fields_named: true,
                dart_metadata: vec![],
                comments: vec![],
                fields: values
                    .iter()
                    .enumerate()
                    .map(|(idx, ty)| IrField {
                        ty: ty.clone(),
                        name: IrIdent::new(format!("field{idx}")),
                        is_final: true,
                        comments: vec![],
                        default: None,
                        settings: Default::default(),
                    })
                    .collect(),
            },
        );
        Ok(IrType::Record(IrTypeRecord {
            inner: IrTypeStructRef {
                // name: safe_ident,
                // freezed: false,
                // empty: false,
                ident: IrStructIdent(NamespacedName::new(namespace.clone(), safe_ident)),
                is_exception: false,
            },
            values: values.into_boxed_slice(),
        }))
    }
}
