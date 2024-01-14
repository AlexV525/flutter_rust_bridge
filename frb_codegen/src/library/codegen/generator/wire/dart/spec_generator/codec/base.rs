use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::EncodeOrDecode;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, CodecMode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::entrypoint::CstWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::entrypoint::DcoWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::codec::pde::entrypoint::PdeWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::entrypoint::SseWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::spec_generator::WireDartOutputSpec;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen_codec_structs;
use serde::Serialize;
use strum::IntoEnumIterator;

codegen_codec_structs!(WireDart);

pub(crate) trait WireDartCodecEntrypointTrait<'a>:
    BaseCodecEntrypointTrait<WireDartGeneratorContext<'a>, WireDartCodecOutputSpec>
{
    fn generate_dart2rust_inner_func_stmt(&self, func: &IrFunc, wire_func_name: &str) -> String;
}
