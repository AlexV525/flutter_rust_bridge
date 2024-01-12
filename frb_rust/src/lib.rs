//! Main documentation is in <https://github.com/fzyzcjy/flutter_rust_bridge>

mod generalized_isolate;
pub(crate) mod handler;
mod misc;
mod platform_types;
mod rust2dart;
pub(crate) mod third_party;
pub(crate) mod thread_pool;

pub(crate) mod codec;
pub(crate) mod dart_fn;
pub(crate) mod dart_opaque;
pub(crate) mod ffi_binding;
#[doc(hidden)]
pub mod for_generated;
#[doc(hidden)] // only to be used as `for_generated::...`
pub mod generalized_arc;
#[doc(hidden)] // only to be used as `for_generated::rust_async`
pub mod rust_async;
pub(crate) mod rust_opaque;
pub(crate) mod web_transfer;

pub use crate::codec::sse::SseCodec;
pub use crate::dart_fn::DartFnFuture;
pub use crate::dart_opaque::DartOpaque;
pub use crate::generalized_isolate::{IntoDart, ZeroCopyBuffer};
pub use crate::handler::handler::Handler;
pub use crate::handler::implementation::handler::DefaultHandler;
pub use crate::misc::dart_dynamic::DartDynamic;
pub use crate::misc::into_into_dart::IntoIntoDart;
#[cfg(feature = "user-utils")]
pub use crate::misc::user_utils::setup_default_user_utils;
pub use crate::rust_async::{spawn, spawn_blocking_with, spawn_local, JoinHandle};
#[allow(deprecated)]
pub use crate::rust_opaque::{DartSafe, RustOpaque, RustOpaqueNom};
pub use flutter_rust_bridge_macros::frb;
