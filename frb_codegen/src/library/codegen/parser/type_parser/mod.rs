use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::hir::hierarchical::struct_or_enum::{HirEnum, HirStruct};
use crate::codegen::mir::func::MirFuncOwnerInfo;
use crate::codegen::mir::namespace::Namespace;
use crate::codegen::mir::pack::{MirEnumPool, MirStructPool};
use crate::codegen::mir::ty::enumeration::{MirEnum, MirEnumIdent};
use crate::codegen::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::mir::ty::structure::{MirStruct, MirStructIdent};
use crate::codegen::mir::ty::MirContext;
use crate::codegen::mir::ty::MirType;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::type_parser::array::ArrayParserInfo;
use crate::codegen::parser::type_parser::enum_or_struct::EnumOrStructParserInfo;
use crate::codegen::parser::type_parser::rust_auto_opaque_implicit::RustAutoOpaqueParserInfo;
use crate::codegen::parser::type_parser::rust_opaque::RustOpaqueParserInfo;
use std::collections::HashMap;
use syn::Type;

pub(crate) mod array;
pub(crate) mod concrete;
mod dart_fn;
mod enum_or_struct;
pub(crate) mod enumeration;
pub(crate) mod external_impl;
pub(crate) mod misc;
pub(crate) mod optional;
pub(crate) mod path;
pub(crate) mod path_data;
pub(crate) mod primitive;
pub(crate) mod result;
pub(crate) mod rust_auto_opaque_explicit;
pub(crate) mod rust_auto_opaque_implicit;
mod rust_opaque;
pub(crate) mod structure;
pub(crate) mod tuple;
pub(crate) mod ty;
pub(crate) mod unencodable;

pub(crate) struct TypeParser<'a> {
    src_structs: HashMap<String, &'a HirStruct>,
    src_enums: HashMap<String, &'a HirEnum>,
    src_types: HashMap<String, Type>,
    dart_code_of_type: HashMap<String, String>,
    struct_parser_info: EnumOrStructParserInfo<MirStructIdent, MirStruct>,
    enum_parser_info: EnumOrStructParserInfo<MirEnumIdent, MirEnum>,
    rust_opaque_parser_info: RustOpaqueParserInfo,
    rust_auto_opaque_parser_info: RustAutoOpaqueParserInfo,
    array_parser_info: ArrayParserInfo,
}

impl<'a> TypeParser<'a> {
    pub(crate) fn new(
        src_structs: HashMap<String, &'a HirStruct>,
        src_enums: HashMap<String, &'a HirEnum>,
        src_types: HashMap<String, Type>,
    ) -> Self {
        TypeParser {
            src_structs,
            src_enums,
            src_types,
            dart_code_of_type: HashMap::new(),
            struct_parser_info: EnumOrStructParserInfo::new(),
            enum_parser_info: EnumOrStructParserInfo::new(),
            rust_opaque_parser_info: RustOpaqueParserInfo::default(),
            rust_auto_opaque_parser_info: RustAutoOpaqueParserInfo::default(),
            array_parser_info: Default::default(),
        }
    }

    pub(crate) fn consume(self) -> (MirStructPool, MirEnumPool, HashMap<String, String>) {
        (
            self.struct_parser_info.object_pool,
            self.enum_parser_info.object_pool,
            self.dart_code_of_type,
        )
    }

    pub(crate) fn parse_type(
        &mut self,
        ty: &Type,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<MirType> {
        TypeParserWithContext::new(self, context).parse_type(ty)
    }

    pub(crate) fn transform_rust_auto_opaque(
        &mut self,
        ty_raw: &MirTypeRustAutoOpaqueImplicit,
        transform: impl FnOnce(&str) -> String,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<MirType> {
        TypeParserWithContext::new(self, context).transform_rust_auto_opaque(ty_raw, transform)
    }
}

pub(crate) struct TypeParserWithContext<'a, 'b, 'c> {
    pub inner: &'b mut TypeParser<'a>,
    pub context: &'c TypeParserParsingContext,
}

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub fn new(inner: &'b mut TypeParser<'a>, context: &'c TypeParserParsingContext) -> Self {
        Self { inner, context }
    }
}

pub(crate) struct TypeParserParsingContext {
    pub(crate) initiated_namespace: Namespace,
    pub(crate) func_attributes: FrbAttributes,
    pub(crate) default_stream_sink_codec: CodecMode,
    pub(crate) default_rust_opaque_codec: RustOpaqueCodecMode,
    pub(crate) owner: Option<MirFuncOwnerInfo>,
}

impl MirContext for TypeParser<'_> {
    fn struct_pool(&self) -> &MirStructPool {
        &self.struct_parser_info.object_pool
    }

    fn enum_pool(&self) -> &MirEnumPool {
        &self.enum_parser_info.object_pool
    }
}
