mod utils;

use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
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

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
struct PubObjInfo {
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
pub extern "C" fn gen_quaternion_str(x: f32, y: f32, z: f32, w: f32) -> String {
    let q = PubQuaternion { x, y, z, w };
    trans_obj_to_string(&q)
}

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn gen_obj_info_str(name: String, age: f32, desc: String) -> String {
    let obj = PubObjInfo { name, age, desc };
    trans_obj_to_string(&obj)
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

fn trans_obj_to_string<T>(value: &T) -> String
where
    T: ?Sized + Serialize,
{
    let str_json = serde_json::to_string(value).unwrap();
    str_json
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
