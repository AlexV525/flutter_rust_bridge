// Name "enumeration" not "enum", since the latter is a keyword

use crate::codegen::mir::comment::MirComment;
use crate::codegen::mir::field::MirField;
use crate::codegen::mir::ident::MirIdent;
use crate::codegen::mir::namespace::{Namespace, NamespacedName};
use crate::codegen::mir::ty::structure::MirStruct;
use crate::codegen::mir::ty::{MirContext, MirType, MirTypeTrait};
use convert_case::{Case, Casing};

crate::mir! {
pub struct MirTypeEnumRef {
    pub ident: MirEnumIdent,
    pub is_exception: bool,
}

pub struct MirEnumIdent(pub NamespacedName);

pub struct MirEnum {
    pub name: NamespacedName,
    pub wrapper_name: Option<String>,
    pub comments: Vec<MirComment>,
    pub variants: Vec<MirVariant>,
    pub mode: MirEnumMode,
}

#[derive(Copy)]
pub enum MirEnumMode {
    Simple,
    Complex,
}

pub struct MirVariant {
    pub name: MirIdent,
    pub wrapper_name: MirIdent,
    pub comments: Vec<MirComment>,
    pub kind: MirVariantKind,
}

pub enum MirVariantKind {
    Value,
    Struct(MirStruct),
}
}

impl MirTypeEnumRef {
    #[inline]
    pub fn get<'a>(&self, file: &'a impl MirContext) -> &'a MirEnum {
        &file.enum_pool()[&self.ident]
    }
}

impl MirTypeTrait for MirTypeEnumRef {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl MirContext,
    ) {
        let enu = self.get(ir_context);
        for variant in enu.variants() {
            if let MirVariantKind::Struct(st) = &variant.kind {
                st.fields
                    .iter()
                    .for_each(|field| field.ty.visit_types(f, ir_context));
            }
        }
    }

    fn safe_ident(&self) -> String {
        self.ident.0.name.to_case(Case::Snake)
    }

    fn rust_api_type(&self) -> String {
        self.ident.0.rust_style()
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.ident.0.namespace.clone())
    }
}

impl MirEnum {
    pub fn variants(&self) -> &[MirVariant] {
        &self.variants
    }
}

impl MirVariantKind {
    pub(crate) fn fields(&self) -> Vec<MirField> {
        match self {
            MirVariantKind::Value => vec![],
            MirVariantKind::Struct(st) => st.fields.clone(),
        }
    }
}

impl From<NamespacedName> for MirEnumIdent {
    fn from(value: NamespacedName) -> Self {
        Self(value)
    }
}
