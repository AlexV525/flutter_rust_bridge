use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for PrimitiveListWireDartCodecCstGenerator<'a> {
    fn encode_func_body(&self) -> Acc<Option<String>> {
        Acc {
            // NOTE Dart code *only* allocates memory. It never *release* memory by itself.
            // Instead, Rust receives that pointer and now it is in control of Rust.
            // Therefore, *never* continue to use this pointer after you have passed the pointer
            // to Rust.
            // NOTE WARN: Never use the [calloc] provided by Dart FFI to allocate any memory.
            // Instead, ask Rust to allocate some memory and return raw pointers. Otherwise,
            // memory will be allocated in one dylib (e.g. libflutter.so), and then be released
            // by another dylib (e.g. my_rust_code.so), especially in Android platform. It can be
            // undefined behavior.
            io: Some(format!(
                "final ans = wire.cst_new_{}(raw.length);
                ans.ref.ptr.asTypedList(raw.length).setAll(0, {});
                return ans;",
                self.ir.safe_ident(),
                match self.ir.primitive {
                    IrTypePrimitive::I64 | IrTypePrimitive::U64 => "raw.inner",
                    _ => "raw",
                }
            )),
            wasm: Some(
                match self.ir.primitive {
                    IrTypePrimitive::I64 | IrTypePrimitive::U64 => "return raw.inner;",
                    _ => "return raw;",
                }
                .into(),
            ),
            ..Default::default()
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Io => {
                format!("ffi.Pointer<wire_cst_{}>", self.ir.safe_ident())
            }
            Target::Wasm => match self.ir.primitive {
                IrTypePrimitive::I64 | IrTypePrimitive::U64 => {
                    "Object /* BigInt64Array */".to_owned()
                }
                _ => ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                    .dart_api_type(),
            },
        }
    }
}
