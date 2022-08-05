#![feature(core_ffi_c)]

use core::ffi::c_char;
use std::ffi::CString;

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ObjInfo {
    pub name: CString,
    pub age: f32,
    pub desc: CString,
}

/***************************start*****************************/
#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}
#[no_mangle]
pub extern "C" fn gen_quaternion(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
    Quaternion { x, y, z, w }
}

#[no_mangle]
pub extern "C" fn gen_obj_info(name: CString, age: f32, desc: CString) -> ObjInfo {
    ObjInfo { name, age, desc }
}
// #[no_mangle]
// pub extern "C" fn gen_obj_info(name: CString, age: f32, desc: CString) -> *mut ObjInfo {
//     let obj=ObjInfo { name, age, desc };
//     Box::into_raw(Box::new(ob))
// }
// #[no_mangle]
// pub extern "C" fn gen_obj_info_free(ptr: *mut ObjInfo) {
//     if ptr.is_null() {
//         return;
//     }
//     unsafe {
//         Box::from_raw(ptr);
//     }
// }

#[no_mangle]
pub extern "C" fn gen_obj_info_str(name: CString, age: f32, desc: CString) -> CString {
    let ob = ObjInfo { name, age, desc };
    let str_json = serde_json::to_string(&ob).expect("json::to_string failed");
    CString::new(str_json).unwrap()
}

// #[no_mangle]
// pub extern "C" fn gen_obj_info_str(name: CString, age: f32, desc: CString) -> *mut c_char {
//     let ob = ObjInfo { name, age, desc };
//     let str_json = serde_json::to_string(&ob).expect("json::to_string failed");
//     let c_str = CString::new(str_json).unwrap();
//     c_str.into_raw()
// }
//
// #[no_mangle]
// pub extern "C" fn gen_obj_info_str_free(s: *mut c_char) {
//     destroy_c_char(s: *mut c_char)
// }

/***************************end*****************************/
pub fn destroy_c_char(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
