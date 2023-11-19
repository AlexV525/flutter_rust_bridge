use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorApi2wireTrait for PrimitiveListWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
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
                "final ans = inner.new_{}(raw.length);
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
                format!("ffi.Pointer<wire_{}>", self.ir.safe_ident())
            }
            Target::Wasm => match self.ir.primitive {
                IrTypePrimitive::I64 | IrTypePrimitive::U64 => {
                    "Object /* BigInt64Array */".to_owned()
                }
                _ => self.ir.dart_api_type(),
            },
        }
    }
}
