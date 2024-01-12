// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

pub use crate::auxiliary::sample_types::{
    FrbOpaqueReturn, HideData, NonCloneData, NonSendHideData,
};
use anyhow::Result;
use flutter_rust_bridge::{opaque_dyn, RustOpaque};
use std::fmt::Debug;
use std::ops::Deref;
pub use std::sync::{Mutex, RwLock};

/// Opaque types
pub trait DartDebugTwinSse: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinSse for T {}

pub enum EnumOpaqueTwinSse {
    Struct(RustOpaqueMoi<HideData>),
    Primitive(RustOpaqueMoi<i32>),
    TraitObj(RustOpaqueMoi<Box<dyn DartDebugTwinSse>>),
    Mutex(RustOpaqueMoi<Mutex<HideData>>),
    RwLock(RustOpaqueMoi<RwLock<HideData>>),
}

/// [`HideData`] has private fields.
pub struct OpaqueNestedTwinSse {
    pub first: RustOpaqueMoi<HideData>,
    pub second: RustOpaqueMoi<HideData>,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_opaque_twin_sse() -> RustOpaqueMoi<HideData> {
    RustOpaque::new(HideData::new())
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_option_opaque_twin_sse(
    opaque: Option<RustOpaqueMoi<HideData>>,
) -> Option<RustOpaqueMoi<HideData>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_create_opaque_twin_sse() -> SyncReturn<RustOpaqueMoi<HideData>> {
//     SyncReturn(RustOpaque::new(HideData::new()))
// }

#[flutter_rust_bridge::frb(serialize)]
pub fn create_array_opaque_enum_twin_sse() -> [EnumOpaqueTwinSse; 5] {
    [
        EnumOpaqueTwinSse::Struct(RustOpaque::new(HideData::new())),
        EnumOpaqueTwinSse::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinSse::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinSse::Mutex(RustOpaque::new(Mutex::new(HideData::new()))),
        EnumOpaqueTwinSse::RwLock(RustOpaque::new(RwLock::new(HideData::new()))),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_enum_opaque_twin_sse(opaque: EnumOpaqueTwinSse) -> String {
    match opaque {
        EnumOpaqueTwinSse::Struct(s) => s.hide_data(),
        EnumOpaqueTwinSse::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinSse::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinSse::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaqueTwinSse::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_opaque_twin_sse(opaque: RustOpaqueMoi<HideData>) -> String {
    opaque.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_opaque_with_delay_twin_sse(opaque: RustOpaqueMoi<HideData>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_array_twin_sse() -> [RustOpaqueMoi<HideData>; 2] {
    [
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_create_non_clone_twin_sse() -> SyncReturn<RustOpaqueMoi<NonCloneData>> {
//     SyncReturn(RustOpaque::new(NonCloneData::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(serialize)]
pub fn run_non_clone_twin_sse(clone: RustOpaqueMoi<NonCloneData>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_sync_opaque_twin_sse() -> RustOpaqueMoi<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_create_sync_opaque_twin_sse() -> SyncReturn<RustOpaqueMoi<NonSendHideData>> {
//     SyncReturn(RustOpaque::new(NonSendHideData::new()))
// }

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_array_run_twin_sse(data: [RustOpaqueMoi<HideData>; 2]) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_vec_twin_sse() -> Vec<RustOpaqueMoi<HideData>> {
    vec![
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_vec_run_twin_sse(data: Vec<RustOpaqueMoi<HideData>>) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_nested_opaque_twin_sse() -> OpaqueNestedTwinSse {
    OpaqueNestedTwinSse {
        first: RustOpaque::new(HideData::new()),
        second: RustOpaque::new(HideData::new()),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_nested_opaque_twin_sse(opaque: OpaqueNestedTwinSse) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

#[flutter_rust_bridge::frb(serialize)]
pub fn unwrap_rust_opaque_twin_sse(opaque: RustOpaqueMoi<HideData>) -> Result<String> {
    let data: HideData = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(serialize)]
pub fn frb_generator_test_twin_sse() -> RustOpaqueMoi<FrbOpaqueReturn> {
    panic!("dummy code");
}
