use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

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
    pub name: String,
    pub age: f32,
    pub desc: String,
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
pub extern "C" fn gen_quaternion_str(x: f32, y: f32, z: f32, w: f32) -> *mut c_char {
    let q = Quaternion { x, y, z, w };
    trans_obj_to_char(&q)
}
#[no_mangle]
pub extern "C" fn gen_quaternion_str_free(s: *mut c_char) {
    destroy_c_char(s)
}


#[no_mangle]
pub extern "C" fn gen_obj_info(name: *const c_char, age: f32, desc: *const c_char) -> *mut c_char {
    let string_name = trans_char_to_string(name);
    let string_desc = trans_char_to_string(desc);

    let obj = ObjInfo {
        name: string_name,
        age,
        desc: string_desc,
    };
    trans_obj_to_char(&obj)
}

#[no_mangle]
pub extern "C" fn gen_obj_info_free(s: *mut c_char) {
    destroy_c_char(s)
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

// #[no_mangle]
// pub extern "C" fn gen_obj_info_str(name: CString, age: f32, desc: CString) -> CString {
//     let ob = ObjInfo { name, age, desc };
//     let str_json = serde_json::to_string(&ob).expect("json::to_string failed");
//     CString::new(str_json).unwrap()
// }

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
