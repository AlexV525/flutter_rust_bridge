use crate::codegen::ir::mir::namespace::Namespace;
use crate::codegen::ir::mir::ty::rust_opaque::{
    MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::RustOpaque;
use crate::codegen::mir_parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::mir_parser::type_parser::TypeParserWithContext;
use quote::ToTokens;
use std::collections::HashMap;
use std::fmt::Debug;
use syn::Type;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_rust_opaque(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        Ok(Some(match last_segment {
            ("RustOpaque", [ty]) => self.parse_rust_opaque(ty, None),
            ("RustOpaqueNom", [ty]) => self.parse_rust_opaque(ty, Some(RustOpaqueCodecMode::Nom)),
            ("RustOpaqueMoi", [ty]) => self.parse_rust_opaque(ty, Some(RustOpaqueCodecMode::Moi)),

            _ => return Ok(None),
        }))
    }

    fn parse_rust_opaque(&mut self, ty: &Type, codec: Option<RustOpaqueCodecMode>) -> MirType {
        let ty_str = ty.to_token_stream().to_string();
        let info = self.inner.rust_opaque_parser_info.get_or_insert(
            ty_str.clone(),
            RustOpaqueParserTypeInfo::new(
                self.context.initiated_namespace.clone(),
                codec
                    .or(self.context.func_attributes.rust_opaque_codec())
                    .unwrap_or(self.context.default_rust_opaque_codec),
            ),
        );
        RustOpaque(MirTypeRustOpaque::new(
            info.namespace,
            MirRustOpaqueInner(ty_str),
            info.codec,
            false,
        ))
    }
}

pub(super) type RustOpaqueParserInfo = GeneralizedRustOpaqueParserInfo;

#[derive(Clone, Debug)]
pub(super) struct RustOpaqueParserTypeInfo {
    pub namespace: Namespace,
    pub codec: RustOpaqueCodecMode,
}

impl RustOpaqueParserTypeInfo {
    pub fn new(namespace: Namespace, codec: RustOpaqueCodecMode) -> Self {
        Self { namespace, codec }
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct GeneralizedRustOpaqueParserInfo(HashMap<String, RustOpaqueParserTypeInfo>);

impl GeneralizedRustOpaqueParserInfo {
    pub fn get_or_insert(
        &mut self,
        type_safe_ident: String,
        insert_value: RustOpaqueParserTypeInfo,
    ) -> RustOpaqueParserTypeInfo {
        (self.0.entry(type_safe_ident).or_insert(insert_value)).clone()
    }
}
