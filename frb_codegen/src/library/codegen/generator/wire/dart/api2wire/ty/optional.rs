use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::base::*;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorApi2wireTrait for OptionalWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            TargetOrCommon::Io | TargetOrCommon::Wasm => Some(format!(
                "return raw == null ? {} : api2wire_{}(raw);",
                if target == TargetOrCommon::Wasm {
                    "null"
                } else {
                    "ffi.nullptr"
                },
                self.ir.inner.safe_ident()
            )),
            _ => None,
        })
    }
}
