use crate::codegen::mir::comment::MirComment;
use crate::codegen::mir::default::MirDefaultValue;
use crate::codegen::mir::ident::MirIdent;
use crate::codegen::mir::ty::MirType;
use serde::Deserialize;

crate::ir! {
pub struct MirField {
    pub ty: MirType,
    pub name: MirIdent,
    pub is_final: bool,
    pub is_rust_public: Option<bool>,
    pub comments: Vec<MirComment>,
    pub default: Option<MirDefaultValue>,
    pub settings: MirFieldSettings,
}

#[derive(Deserialize, Default)]
pub struct MirFieldSettings {
    pub is_in_mirrored_enum: bool,
}
}

impl MirField {
    #[inline]
    pub fn is_optional(&self) -> bool {
        matches!(&self.ty, MirType::Optional(_)) || self.default.is_some()
    }
}
