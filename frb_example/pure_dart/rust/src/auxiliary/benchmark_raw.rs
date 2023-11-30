#![allow(non_camel_case_types)]

use crate::api::benchmark_api::{
    benchmark_input_bytes_twin_normal, benchmark_output_bytes_twin_normal,
};
use flutter_rust_bridge::support::{new_leak_vec_ptr, vec_from_leak_ptr};
use flutter_rust_bridge::{Channel, IntoDart, MessagePort, ZeroCopyBuffer};

#[no_mangle]
pub extern "C" fn benchmark_raw_void_sync() {}

#[repr(C)]
#[derive(Clone)]
pub struct benchmark_raw_list_prim_u_8 {
    ptr: *mut u8,
    len: i32,
}

#[no_mangle]
pub extern "C" fn benchmark_raw_new_list_prim_u_8(len: i32) -> benchmark_raw_list_prim_u_8 {
    benchmark_raw_list_prim_u_8 {
        ptr: new_leak_vec_ptr(Default::default(), len),
        len,
    }
}

// NOTE: `Vec::from_raw_parts` says:
// "it is normally not safe to build a Vec<u8> from a pointer to a C char array with length size_t,
// doing so is only safe if the array was initially allocated by a Vec or String"
// Thus, the input vec *should* be allocated by our `new_leak_vec_ptr` only.
#[no_mangle]
pub unsafe extern "C" fn benchmark_raw_input_bytes(bytes: benchmark_raw_list_prim_u_8) -> i32 {
    let vec = vec_from_leak_ptr(bytes.ptr, bytes.len);
    benchmark_input_bytes_twin_normal(vec)
}

#[no_mangle]
pub extern "C" fn benchmark_raw_output_bytes(port: MessagePort, size: i32) {
    let vec = benchmark_output_bytes_twin_normal(size);
    Channel::new(port).post(ZeroCopyBuffer(vec).into_dart());
}
