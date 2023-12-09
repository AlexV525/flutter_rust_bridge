use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for DartOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(simple_delegate_encode(
            lang,
            &self.ir.get_delegate(),
            match lang {
                Lang::DartLang(_) => {
                    "PlatformPointerUtil.ptrToInt(wire.dart_opaque_dart2rust_encode(self))"
                }
                Lang::RustLang(_) => "self.encode()",
            },
        ))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(simple_delegate_decode(
            lang,
            &self.ir.get_delegate(),
            "unsafe { flutter_rust_bridge::for_generated::sse_decode_dart_opaque(inner) }",
        ))
    }
}
