use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_test_method__method__SharedStructInBlock1And2(
    port_: i64,
    that: *mut wire_SharedStructInBlock1And2,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__SharedStructInBlock1And2_impl(port_, that, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__SharedStructInBlock1And2(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__SharedStructInBlock1And2_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__SharedStructInAllBlocks(
    port_: i64,
    that: *mut wire_SharedStructInAllBlocks,
    message: *mut wire_uint_8_list,
    num: u32,
) {
    wire_test_method__method__SharedStructInAllBlocks_impl(port_, that, message, num)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__SharedStructInAllBlocks(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__SharedStructInAllBlocks_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__CrossSharedStructInBlock1And2(
    port_: i64,
    that: *mut wire_CrossSharedStructInBlock1And2,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__CrossSharedStructInBlock1And2_impl(port_, that, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__CrossSharedStructInBlock1And2(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__CrossSharedStructInBlock1And2_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__SharedStructOnlyForSyncTest(
    port_: i64,
    that: *mut wire_SharedStructOnlyForSyncTest,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__SharedStructOnlyForSyncTest_impl(port_, that, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__SharedStructOnlyForSyncTest(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__SharedStructOnlyForSyncTest_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__CrossSharedStructInBlock2And3(
    port_: i64,
    that: *mut wire_CrossSharedStructInBlock2And3,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__CrossSharedStructInBlock2And3_impl(port_, that, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__CrossSharedStructInBlock2And3(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__CrossSharedStructInBlock2And3_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__SharedStructInBlock2And3(
    port_: i64,
    that: *mut wire_SharedStructInBlock2And3,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__SharedStructInBlock2And3_impl(port_, that, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__SharedStructInBlock2And3(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__SharedStructInBlock2And3_impl(port_, message)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_cross_shared_struct_in_block_1_and_2(
) -> *mut wire_CrossSharedStructInBlock1And2 {
    support::new_leak_box_ptr(wire_CrossSharedStructInBlock1And2::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_cross_shared_struct_in_block_2_and_3(
) -> *mut wire_CrossSharedStructInBlock2And3 {
    support::new_leak_box_ptr(wire_CrossSharedStructInBlock2And3::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f64(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i32(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shared_struct_in_all_blocks() -> *mut wire_SharedStructInAllBlocks
{
    support::new_leak_box_ptr(wire_SharedStructInAllBlocks::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shared_struct_in_block_1_and_2(
) -> *mut wire_SharedStructInBlock1And2 {
    support::new_leak_box_ptr(wire_SharedStructInBlock1And2::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shared_struct_in_block_2_and_3(
) -> *mut wire_SharedStructInBlock2And3 {
    support::new_leak_box_ptr(wire_SharedStructInBlock2And3::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shared_struct_only_for_sync_test(
) -> *mut wire_SharedStructOnlyForSyncTest {
    support::new_leak_box_ptr(wire_SharedStructOnlyForSyncTest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_enum_type() -> *mut wire_EnumType {
    support::new_leak_box_ptr(wire_EnumType::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_float_32_list(len: i32) -> *mut wire_float_32_list {
    let ans = wire_float_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_enum_type(len: i32) -> *mut wire_list_enum_type {
    let wrap = wire_list_enum_type {
        ptr: support::new_leak_vec_ptr(<wire_EnumType>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<ZeroCopyBuffer<Vec<f32>>> for *mut wire_float_32_list {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<f32>> {
        ZeroCopyBuffer(self.wire2api())
    }
}

impl Wire2Api<CrossSharedStructInBlock1And2> for *mut wire_CrossSharedStructInBlock1And2 {
    fn wire2api(self) -> CrossSharedStructInBlock1And2 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CrossSharedStructInBlock1And2>::wire2api(*wrap).into()
    }
}
impl Wire2Api<CrossSharedStructInBlock2And3> for *mut wire_CrossSharedStructInBlock2And3 {
    fn wire2api(self) -> CrossSharedStructInBlock2And3 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CrossSharedStructInBlock2And3>::wire2api(*wrap).into()
    }
}
impl Wire2Api<f64> for *mut f64 {
    fn wire2api(self) -> f64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<SharedStructInAllBlocks> for *mut wire_SharedStructInAllBlocks {
    fn wire2api(self) -> SharedStructInAllBlocks {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SharedStructInAllBlocks>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SharedStructInBlock1And2> for *mut wire_SharedStructInBlock1And2 {
    fn wire2api(self) -> SharedStructInBlock1And2 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SharedStructInBlock1And2>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SharedStructInBlock2And3> for *mut wire_SharedStructInBlock2And3 {
    fn wire2api(self) -> SharedStructInBlock2And3 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SharedStructInBlock2And3>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SharedStructOnlyForSyncTest> for *mut wire_SharedStructOnlyForSyncTest {
    fn wire2api(self) -> SharedStructOnlyForSyncTest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SharedStructOnlyForSyncTest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<EnumType>> for *mut wire_EnumType {
    fn wire2api(self) -> Box<EnumType> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumType>::wire2api(*wrap).into()
    }
}
impl Wire2Api<CrossSharedStructInBlock1And2> for wire_CrossSharedStructInBlock1And2 {
    fn wire2api(self) -> CrossSharedStructInBlock1And2 {
        CrossSharedStructInBlock1And2 {
            name: self.name.wire2api(),
        }
    }
}
impl Wire2Api<CrossSharedStructInBlock2And3> for wire_CrossSharedStructInBlock2And3 {
    fn wire2api(self) -> CrossSharedStructInBlock2And3 {
        CrossSharedStructInBlock2And3 {
            name: self.name.wire2api(),
        }
    }
}
impl Wire2Api<EnumType> for wire_EnumType {
    fn wire2api(self) -> EnumType {
        match self.tag {
            0 => EnumType::Empty,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Primitives);
                EnumType::Primitives {
                    int32: ans.int32.wire2api(),
                    float64: ans.float64.wire2api(),
                    boolean: ans.boolean.wire2api(),
                }
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Nested);
                EnumType::Nested(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Optional);
                EnumType::Optional(ans.field0.wire2api(), ans.field1.wire2api())
            },
            4 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Buffer);
                EnumType::Buffer(ans.field0.wire2api())
            },
            5 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Enums);
                EnumType::Enums(ans.field0.wire2api())
            },
            6 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.BytesArray);
                EnumType::BytesArray(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Vec<f32>> for *mut wire_float_32_list {
    fn wire2api(self) -> Vec<f32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<Vec<EnumType>> for *mut wire_list_enum_type {
    fn wire2api(self) -> Vec<EnumType> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<SharedStructInAllBlocks> for wire_SharedStructInAllBlocks {
    fn wire2api(self) -> SharedStructInAllBlocks {
        SharedStructInAllBlocks {
            id: self.id.wire2api(),
            num: self.num.wire2api(),
            name: self.name.wire2api(),
            enum_list: self.enum_list.wire2api(),
        }
    }
}
impl Wire2Api<SharedStructInBlock1And2> for wire_SharedStructInBlock1And2 {
    fn wire2api(self) -> SharedStructInBlock1And2 {
        SharedStructInBlock1And2 {
            id: self.id.wire2api(),
            num: self.num.wire2api(),
            name: self.name.wire2api(),
        }
    }
}
impl Wire2Api<SharedStructInBlock2And3> for wire_SharedStructInBlock2And3 {
    fn wire2api(self) -> SharedStructInBlock2And3 {
        SharedStructInBlock2And3 {
            id: self.id.wire2api(),
            num: self.num.wire2api(),
            name: self.name.wire2api(),
        }
    }
}
impl Wire2Api<SharedStructOnlyForSyncTest> for wire_SharedStructOnlyForSyncTest {
    fn wire2api(self) -> SharedStructOnlyForSyncTest {
        SharedStructOnlyForSyncTest {
            name: self.name.wire2api(),
            score: self.score.wire2api(),
        }
    }
}

impl Wire2Api<[u8; 3]> for *mut wire_uint_8_list {
    fn wire2api(self) -> [u8; 3] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_CrossSharedStructInBlock1And2 {
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CrossSharedStructInBlock2And3 {
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_32_list {
    ptr: *mut f32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_enum_type {
    ptr: *mut wire_EnumType,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SharedStructInAllBlocks {
    id: i32,
    num: f64,
    name: *mut wire_uint_8_list,
    enum_list: *mut wire_list_enum_type,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SharedStructInBlock1And2 {
    id: i32,
    num: f64,
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SharedStructInBlock2And3 {
    id: i32,
    num: f64,
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SharedStructOnlyForSyncTest {
    name: *mut wire_uint_8_list,
    score: f64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumType {
    tag: i32,
    kind: *mut EnumTypeKind,
}

#[repr(C)]
pub union EnumTypeKind {
    Empty: *mut wire_EnumType_Empty,
    Primitives: *mut wire_EnumType_Primitives,
    Nested: *mut wire_EnumType_Nested,
    Optional: *mut wire_EnumType_Optional,
    Buffer: *mut wire_EnumType_Buffer,
    Enums: *mut wire_EnumType_Enums,
    BytesArray: *mut wire_EnumType_BytesArray,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumType_Empty {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumType_Primitives {
    int32: i32,
    float64: f64,
    boolean: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumType_Nested {
    field0: *mut wire_EnumType,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumType_Optional {
    field0: *mut i32,
    field1: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumType_Buffer {
    field0: *mut wire_float_32_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumType_Enums {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumType_BytesArray {
    field0: *mut wire_uint_8_list,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_CrossSharedStructInBlock1And2 {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_CrossSharedStructInBlock1And2 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_CrossSharedStructInBlock2And3 {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_CrossSharedStructInBlock2And3 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl Default for wire_EnumType {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_EnumType {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_EnumType_Primitives() -> *mut EnumTypeKind {
    support::new_leak_box_ptr(EnumTypeKind {
        Primitives: support::new_leak_box_ptr(wire_EnumType_Primitives {
            int32: Default::default(),
            float64: Default::default(),
            boolean: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumType_Nested() -> *mut EnumTypeKind {
    support::new_leak_box_ptr(EnumTypeKind {
        Nested: support::new_leak_box_ptr(wire_EnumType_Nested {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumType_Optional() -> *mut EnumTypeKind {
    support::new_leak_box_ptr(EnumTypeKind {
        Optional: support::new_leak_box_ptr(wire_EnumType_Optional {
            field0: core::ptr::null_mut(),
            field1: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumType_Buffer() -> *mut EnumTypeKind {
    support::new_leak_box_ptr(EnumTypeKind {
        Buffer: support::new_leak_box_ptr(wire_EnumType_Buffer {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumType_Enums() -> *mut EnumTypeKind {
    support::new_leak_box_ptr(EnumTypeKind {
        Enums: support::new_leak_box_ptr(wire_EnumType_Enums {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumType_BytesArray() -> *mut EnumTypeKind {
    support::new_leak_box_ptr(EnumTypeKind {
        BytesArray: support::new_leak_box_ptr(wire_EnumType_BytesArray {
            field0: core::ptr::null_mut(),
        }),
    })
}

impl NewWithNullPtr for wire_SharedStructInAllBlocks {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
            enum_list: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SharedStructInAllBlocks {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SharedStructInBlock1And2 {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SharedStructInBlock1And2 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SharedStructInBlock2And3 {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SharedStructInBlock2And3 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SharedStructOnlyForSyncTest {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            score: Default::default(),
        }
    }
}

impl Default for wire_SharedStructOnlyForSyncTest {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
