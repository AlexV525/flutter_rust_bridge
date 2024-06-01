mod field;

use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::hir::hierarchical::struct_or_enum::HirStruct;
use crate::codegen::mir::func::{MirFunc, MirFuncAccessorMode};
use crate::codegen::mir::namespace::NamespacedName;
use crate::codegen::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::mir::ty::{MirContext, MirType};
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::misc::extract_src_types_in_paths;
use crate::codegen::parser::sanity_checker::auto_accessor_checker;
use crate::codegen::parser::type_parser::{
    TypeParser, TypeParserParsingContext, TypeParserWithContext,
};
use field::parse_auto_accessor_of_field;
use itertools::Itertools;
use std::collections::HashMap;

pub(crate) fn parse_auto_accessors(
    config: &ParserInternalConfig,
    src_structs: &HashMap<String, &HirStruct>,
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<MirFunc>> {
    let src_structs_in_paths =
        extract_src_types_in_paths(src_structs, &config.rust_input_namespace_pack)?;

    let infos = src_structs_in_paths
        .iter()
        .map(|struct_name| parse_auto_accessors_of_struct(config, struct_name, type_parser))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec();

    auto_accessor_checker::report(
        &infos
            .iter()
            .flat_map(|x| x.sanity_check_hint.clone())
            .collect_vec(),
    );

    Ok(infos.into_iter().map(|x| x.ir_func).collect_vec())
}

fn parse_auto_accessors_of_struct(
    config: &ParserInternalConfig,
    struct_name: &NamespacedName,
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<MirFuncAndSanityCheckInfo>> {
    let context = create_parsing_context(
        struct_name,
        config.default_stream_sink_codec,
        config.default_rust_opaque_codec,
    )?;

    let ty_direct_parse =
        match type_parser.parse_type(&syn::parse_str(&struct_name.name)?, &context) {
            Ok(value) => value,
            // We do not care about parsing errors here (e.g. some type that we do not support)
            Err(_) => return Ok(vec![]),
        };
    if !matches!(ty_direct_parse, MirType::RustAutoOpaqueImplicit(_)) {
        return Ok(vec![]);
    }

    let ty_struct_ref = TypeParserWithContext::new(type_parser, &context)
        .parse_type_path_data_struct(&(&struct_name.name, &[]), Some(false));
    let ty_struct_ident = if let Ok(Some(MirType::StructRef(ir))) = ty_struct_ref {
        ir.ident
    } else {
        return Ok(vec![]);
    };
    let ty_struct = &type_parser.struct_pool()[&ty_struct_ident].to_owned();

    (ty_struct.fields.iter())
        .filter(|field| field.is_rust_public.unwrap())
        .flat_map(|field| {
            [MirFuncAccessorMode::Getter, MirFuncAccessorMode::Setter]
                .into_iter()
                .map(|accessor_mode| {
                    parse_auto_accessor_of_field(
                        config,
                        struct_name,
                        field,
                        accessor_mode,
                        &ty_direct_parse,
                        type_parser,
                        &context,
                    )
                })
                .collect_vec()
        })
        .collect()
}

fn create_parsing_context(
    struct_name: &NamespacedName,
    default_stream_sink_codec: CodecMode,
    default_rust_opaque_codec: RustOpaqueCodecMode,
) -> anyhow::Result<TypeParserParsingContext> {
    Ok(TypeParserParsingContext {
        initiated_namespace: struct_name.namespace.to_owned(),
        func_attributes: FrbAttributes::parse(&[])?,
        default_stream_sink_codec,
        default_rust_opaque_codec,
        owner: None,
    })
}

struct MirFuncAndSanityCheckInfo {
    mir_func: MirFunc,
    sanity_check_hint: Option<auto_accessor_checker::SanityCheckHint>,
}
