use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;

impl<'a> WireRustGeneratorWire2apiTrait for RecordWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        self.as_struct_generator().generate_wire2api_class()
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        let ir = self.ir.inner.get(self.context.ir_pack);
        let len = ir.fields.len();
        let values: Acc<Vec<_>> = ir
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| Acc {
                wasm: format!("self_.get({idx}).wire2api()"),
                io: format!("self.{}.wire2api()", field.name.rust_style()),
                ..Default::default()
            })
            .collect();
        Acc {
            io: Some(format!("({},)", values.io.join(","))),
            wasm: Some(format!(
                "let self_ = self.dyn_into::<JsArray>().unwrap();
                assert_eq!(self_.length(), {len}, \"Expected {len} elements, got {{}}\", self_.length());
                ({},)",
                values.wasm.join(",")
            )),
            ..Default::default()
        }
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        self.as_struct_generator().generate_impl_new_with_nullptr()
    }
}
