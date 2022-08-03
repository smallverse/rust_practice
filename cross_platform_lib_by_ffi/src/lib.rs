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
    pub name: CString,
    pub age: f32,
    pub desc: CString,
}
/***************************start*****************************/
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn gen_quaternion(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
    Quaternion { x, y, z, w }
}

pub fn gen_obj_info(name: CString, age: f32, desc: CString) -> ObjInfo {
    ObjInfo { name, age, desc }
}

pub fn gen_obj_info_str(name: CString, age: f32, desc: CString) -> CString {
    let ob = ObjInfo { name, age, desc };
    let str_json = serde_json::to_string(&ob).unwrap();
    let c_str = CString::new(str_json).expect("CString::new failed");
    c_str
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
