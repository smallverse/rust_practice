use crate::ffi::{ObjInfo, Quaternion};
use std::ffi::{CStr, CString};
use std::iter;
use std::os::raw::c_char;

#[cxx::bridge(namespace = "com::cross_platform_lib")]
mod ffi {
    extern "Rust" {
        fn add(left: usize, right: usize) -> usize;
        fn gen_quaternion(x: f32, y: f32, z: f32, w: f32) -> Quaternion;
        fn gen_obj_info(name: String, age: f32, desc: String) -> ObjInfo;
        fn gen_obj_info_str(name: String, age: f32, desc: String) -> String;
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Quaternion {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ObjInfo {
        pub name: String,
        pub age: f32,
        pub desc: String,
    }
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
pub extern "C" fn gen_obj_info(name: String, age: f32, desc: String) -> ObjInfo {
    ObjInfo { name, age, desc }
}

#[no_mangle]
pub extern "C" fn gen_obj_info_str(name: String, age: f32, desc: String) -> String {
    let ob = ObjInfo { name, age, desc };
    serde_json::to_string(&ob).unwrap()
}

/***************************end*****************************/
fn c_char_to_string(name: *const c_char) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned(); // if necessary
    str_buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
