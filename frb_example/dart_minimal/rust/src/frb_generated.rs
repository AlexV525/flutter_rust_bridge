// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 1.82.4.

#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]

// Section: imports

use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate!();

// Section: executor

flutter_rust_bridge::frb_generated_default_handler!();

// Section: wire_funcs

fn wire_hi_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "hi",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_a = <DART_FN_RUST_API_TYPE_NOT_USED>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse((move || {
                    Result::<_, ()>::Ok(crate::api::minimal::hi(api_a))
                })())
            }
        },
    )
}
fn wire_minimal_adder_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    a: impl CstDecode<i32> + core::panic::UnwindSafe,
    b: impl CstDecode<i32> + core::panic::UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "minimal_adder",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_a = a.cst_decode();
            let api_b = b.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    Result::<_, ()>::Ok(crate::api::minimal::minimal_adder(api_a, api_b))
                })())
            }
        },
    )
}

// Section: related_funcs

fn decode_DartFn_Inputs_String_String_Output_String(
    dart_opaque: flutter_rust_bridge::DartOpaque,
) -> impl Fn(String, String) -> flutter_rust_bridge::DartFnFuture<String> {
    use flutter_rust_bridge::IntoDart;

    async fn body(
        dart_opaque: flutter_rust_bridge::DartOpaque,
        arg0: String,
        arg1: String,
    ) -> String {
        let args = vec![
            arg0.into_into_dart().into_dart(),
            arg1.into_into_dart().into_dart(),
        ];
        let message = FLUTTER_RUST_BRIDGE_HANDLER
            .dart_fn_invoke(dart_opaque, args)
            .await;
        <String>::sse_decode_single(message)
    }

    move |arg0: String, arg1: String| {
        flutter_rust_bridge::for_generated::convert_into_dart_fn_future(body(
            dart_opaque.clone(),
            arg0,
            arg1,
        ))
    }
}

// Section: dart2rust

impl CstDecode<i32> for i32 {
    fn cst_decode(self) -> i32 {
        self
    }
}
impl CstDecode<u8> for u8 {
    fn cst_decode(self) -> u8 {
        self
    }
}
impl CstDecode<usize> for usize {
    fn cst_decode(self) -> usize {
        self
    }
}
impl SseDecode for flutter_rust_bridge::DartOpaque {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <usize>::sse_decode(deserializer);
        return unsafe { flutter_rust_bridge::for_generated::sse_decode_dart_opaque(inner) };
    }
}

impl SseDecode for String {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<u8>>::sse_decode(deserializer);
        return String::from_utf8(inner).unwrap();
    }
}

impl SseDecode for i32 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for Vec<u8> {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<u8>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for u8 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap()
    }
}

impl SseDecode for () {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {}
}

impl SseDecode for usize {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u64::<NativeEndian>().unwrap() as _
    }
}

// Section: rust2dart

impl SseEncode for flutter_rust_bridge::DartOpaque {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <usize>::sse_encode(self.encode(), serializer);
    }
}

impl SseEncode for String {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.into_bytes(), serializer);
    }
}

impl SseEncode for i32 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for Vec<u8> {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <u8>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for u8 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self).unwrap();
    }
}

impl SseEncode for () {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {}
}

impl SseEncode for usize {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer
            .cursor
            .write_u64::<NativeEndian>(self as _)
            .unwrap();
    }
}

#[cfg(not(target_family = "wasm"))]
#[path = "frb_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "frb_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;
