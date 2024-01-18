use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::lang::rust::RustLang;
use crate::codegen::generator::codec::sse::lang::Lang;
use crate::codegen::generator::codec::sse::misc::with_sse_extra_types;
use crate::codegen::generator::codec::sse::ty::{CodecSseTy, CodecSseTyContext};
use crate::codegen::generator::codec::structs::{CodecMode, EncodeOrDecode};
use crate::codegen::generator::misc::comments::generate_codec_comments;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecOutputSpec;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::base::WireRustCodecSseGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::codec::sse::ty::CodecSseTyTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

pub(super) fn generate_encode_or_decode(
    context: WireRustCodecSseGeneratorContext,
    types: &[IrType],
    mode: EncodeOrDecode,
) -> WireRustCodecOutputSpec {
    let types = with_sse_extra_types(types);

    let mut inner = Default::default();
    inner += (types.iter())
        .flat_map(|ty| {
            vec![
                generate_encode_or_decode_for_type(ty, context, mode),
                create_codec_sse_ty(ty, context).generate_extra(&Lang::RustLang(RustLang)),
            ]
        })
        .collect();
    WireRustCodecOutputSpec { inner }
}

fn generate_encode_or_decode_for_type(
    ty: &IrType,
    context: WireRustCodecSseGeneratorContext,
    mode: EncodeOrDecode,
) -> Acc<WireRustOutputCode> {
    let rust_api_type = ty.rust_api_type();
    let body = create_codec_sse_ty(ty, context).generate(&Lang::RustLang(RustLang), mode);

    if let Some(body) = body {
        Acc::new_common(generate_sse_encode_or_decode_impl(&rust_api_type, body.trim()).into())
    } else {
        Acc::default()
    }
}

fn create_codec_sse_ty(ty: &IrType, context: WireRustCodecSseGeneratorContext) -> CodecSseTy {
    CodecSseTy::new(
        ty.clone(),
        CodecSseTyContext::new(context.ir_pack, context.api_dart_config),
    )
}

pub(crate) fn generate_sse_encode_or_decode_impl(
    rust_api_type: &str,
    body: &str,
    mode: EncodeOrDecode,
) -> String {
    let codec_comments = generate_codec_comments(CodecMode::Sse);
    match mode {
        EncodeOrDecode::Encode => format!(
            "
            impl SseEncode for {rust_api_type} {{
                {codec_comments}
                fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {{{body}}}
            }}
            "
        ),
        EncodeOrDecode::Decode => format!(
            "
            impl SseDecode for {rust_api_type} {{
                {codec_comments}
                fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {{{body}}}
            }}
            "
        ),
    }
}
