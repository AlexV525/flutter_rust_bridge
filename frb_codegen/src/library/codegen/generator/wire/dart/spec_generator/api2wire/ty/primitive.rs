use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::base::ApiDartGenerator;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::library::codegen::generator::api_dart::info::ApiDartGeneratorInfoTrait;

impl<'a> WireDartGeneratorApi2wireTrait for PrimitiveWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        match self.ir {
            IrTypePrimitive::I64 | IrTypePrimitive::U64 => Acc {
                io: Some("return raw;".into()),
                wasm: Some("return castNativeBigInt(raw);".into()),
                ..Default::default()
            },
            _ => "return raw;".into(),
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match &self.ir {
            IrTypePrimitive::I64 | IrTypePrimitive::U64 if target == Target::Wasm => {
                "Object".into()
            }
            _ => ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                .dart_api_type(),
        }
    }
}

/// Representations of primitives within Dart's pointers, e.g. `ffi.Pointer<ffi.Uint8>`.
/// This is enforced on Dart's side, and should be used instead of `dart_wire_type`
/// whenever primitives are put behind a pointer.
pub fn dart_native_type_of_primitive(prim: &IrTypePrimitive) -> &'static str {
    match prim {
        IrTypePrimitive::U8 => "ffi.Uint8",
        IrTypePrimitive::I8 => "ffi.Int8",
        IrTypePrimitive::U16 => "ffi.Uint16",
        IrTypePrimitive::I16 => "ffi.Int16",
        IrTypePrimitive::U32 => "ffi.Uint32",
        IrTypePrimitive::I32 => "ffi.Int32",
        IrTypePrimitive::U64 => "ffi.Uint64",
        IrTypePrimitive::I64 => "ffi.Int64",
        IrTypePrimitive::Usize => "ffi.UintPtr",
        IrTypePrimitive::Isize => "ffi.IntPtr",
        IrTypePrimitive::F32 => "ffi.Float",
        IrTypePrimitive::F64 => "ffi.Double",
        IrTypePrimitive::Bool => "ffi.Bool",
        IrTypePrimitive::Unit => "ffi.Void",
    }
}
