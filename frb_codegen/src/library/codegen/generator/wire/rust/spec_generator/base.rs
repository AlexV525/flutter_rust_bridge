use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::WireRustCodecCstGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::WireRustCodecDcoGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::base::WireRustCodecSseGeneratorContext;
use crate::codegen::mir::ty::MirType::*;
use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireRustGeneratorImplTrait)]
    #[enum_dispatch(WireRustGeneratorRust2DartTrait)]
    #[enum_dispatch(WireRustGeneratorDart2RustTrait)]
    #[enum_dispatch(WireRustGeneratorMiscTrait)]
    WireRustGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireRustGeneratorContext<'a> {
    pub(crate) ir_pack: &'a MirPack,
    pub(crate) config: &'a GeneratorWireRustInternalConfig,
    pub(crate) wire_dart_config: &'a GeneratorWireDartInternalConfig,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

impl WireRustGeneratorContext<'_> {
    pub(crate) fn as_wire_rust_codec_cst_context(&self) -> WireRustCodecCstGeneratorContext {
        WireRustCodecCstGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.config,
            wire_dart_config: self.wire_dart_config,
            api_dart_config: self.api_dart_config,
        }
    }

    pub(crate) fn as_wire_rust_codec_dco_context(&self) -> WireRustCodecDcoGeneratorContext {
        WireRustCodecDcoGeneratorContext {
            ir_pack: self.ir_pack,
        }
    }

    pub(crate) fn as_wire_rust_codec_sse_context(&self) -> WireRustCodecSseGeneratorContext {
        WireRustCodecSseGeneratorContext {
            ir_pack: self.ir_pack,
            api_dart_config: self.api_dart_config,
        }
    }
}
