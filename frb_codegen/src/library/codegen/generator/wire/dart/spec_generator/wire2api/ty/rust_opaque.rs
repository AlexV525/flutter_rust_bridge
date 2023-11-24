use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::wire2api::ty::WireDartGeneratorWire2apiTrait;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> WireDartGeneratorWire2apiTrait for RustOpaqueRefWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        format!(
            "return {0}.fromRaw(raw[0], raw[1]);",
            ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                .dart_api_type()
        )
    }
}
