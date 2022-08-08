mod utils;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust_practice!");
}

#[wasm_bindgen]
#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PubQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[wasm_bindgen]
#[repr(C)]
pub struct PubObjInfo {
    pub name: *const c_char,
    pub age: f32,
    pub desc: *const c_char,
}

#[derive(Debug, Serialize, Deserialize)]
struct ObjInfo {
    pub name: String,
    pub age: f32,
    pub desc: String,
}

/***************************start*****************************/
#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn gen_quaternion(x: f32, y: f32, z: f32, w: f32) -> PubQuaternion {
    PubQuaternion { x, y, z, w }
}

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn gen_quaternion_str(x: f32, y: f32, z: f32, w: f32) -> *mut c_char {
    let q = PubQuaternion { x, y, z, w };
    trans_obj_to_char(&q)
}

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn gen_quaternion_str_free(s: *mut c_char) {
    destroy_c_char(s)
}

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn gen_obj_info_str(
    name: *const c_char,
    age: f32,
    desc: *const c_char,
) -> *mut c_char {
    let string_name = trans_char_to_string(name);
    let string_desc = trans_char_to_string(desc);

    let obj = ObjInfo {
        name: string_name,
        age,
        desc: string_desc,
    };
    trans_obj_to_char(&obj)
}

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn gen_obj_info_str_free(s: *mut c_char) {
    destroy_c_char(s)
}

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn gen_obj_info(
    name: *const c_char,
    age: f32,
    desc: *const c_char,
) -> *mut PubObjInfo {
    let obj = PubObjInfo { name, age, desc };
    Box::into_raw(Box::new(obj))
}

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn gen_obj_info_free(ptr: *mut PubObjInfo) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

/***************************end*****************************/
fn destroy_c_char(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

fn trans_obj_to_char<T>(value: &T) -> *mut c_char
    where
        T: ?Sized + Serialize,
{
    let str_json = serde_json::to_string(value).unwrap();
    let c_str = CString::new(str_json).unwrap();
    c_str.into_raw()
}

fn trans_char_to_string(char: *const c_char) -> String {
    let c_str = unsafe {
        assert!(!char.is_null());

        CStr::from_ptr(char)
    };
    let r_str = c_str.to_str().unwrap();
    let string_re = String::from(r_str);
    string_re
}
