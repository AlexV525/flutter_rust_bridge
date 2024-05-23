use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for PrimitiveListWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        match &self.ir.primitive {
            IrTypePrimitive::I64 | IrTypePrimitive::Isize => "return Int64List.from(raw);".into(),
            IrTypePrimitive::U64 | IrTypePrimitive::Usize => "return Uint64List.from(raw);".into(),
            _ => gen_decode_simple_type_cast(self.ir.clone().into(), self.context),
        }
    }
}
