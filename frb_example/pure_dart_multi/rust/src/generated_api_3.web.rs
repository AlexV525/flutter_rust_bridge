use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_test_inbuilt_type_in_block_3(port_: MessagePort, a: i32, b: f32) {
    wire_test_inbuilt_type_in_block_3_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_test_string_in_block_3(port_: MessagePort, s: String, i: u64) {
    wire_test_string_in_block_3_impl(port_, s, i)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3(
    port_: MessagePort,
    score: f64,
) {
    wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3_impl(port_, score)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_only_for_sync_as_input_with_no_sync_return_in_block_3(
    port_: MessagePort,
    obj: JsValue,
    default_score: f64,
) {
    wire_test_shared_struct_only_for_sync_as_input_with_no_sync_return_in_block_3_impl(
        port_,
        obj,
        default_score,
    )
}

#[wasm_bindgen]
pub fn wire_test_all_shared_struct_in_block_3(
    port_: MessagePort,
    custom: JsValue,
    s: String,
    i: i32,
) {
    wire_test_all_shared_struct_in_block_3_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_in_block_3_for_2_and_3(
    port_: MessagePort,
    custom: JsValue,
    s: String,
    i: i32,
) {
    wire_test_shared_struct_in_block_3_for_2_and_3_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_cross_shared_struct_in_block_3_for_2_and_3(port_: MessagePort, name: String) {
    wire_test_cross_shared_struct_in_block_3_for_2_and_3_impl(port_, name)
}

#[wasm_bindgen]
pub fn wire_test_cross_shared_struct_in_sync_in_block_3_for_2_and_3(
    name: String,
) -> support::WireSyncReturn {
    wire_test_cross_shared_struct_in_sync_in_block_3_for_2_and_3_impl(name)
}

#[wasm_bindgen]
pub fn wire_test_unique_struct_3(port_: MessagePort, custom: JsValue, s: String, i: i64) {
    wire_test_unique_struct_3_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_struct_defined_in_block_3(port_: MessagePort, custom: JsValue) {
    wire_test_struct_defined_in_block_3_impl(port_, custom)
}

#[wasm_bindgen]
pub fn wire_test_method__method__StructDefinedInBlock3(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__StructDefinedInBlock3_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__StructDefinedInBlock3(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__StructDefinedInBlock3_impl(port_, message)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<StructDefinedInBlock3> for JsValue {
    fn wire2api(self) -> StructDefinedInBlock3 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructDefinedInBlock3 {
            name: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<StructOnlyForBlock3> for JsValue {
    fn wire2api(self) -> StructOnlyForBlock3 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        StructOnlyForBlock3 {
            id: self_.get(0).wire2api(),
            num: self_.get(1).wire2api(),
            name: self_.get(2).wire2api(),
        }
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> i64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
