use jni::{
    objects::JClass,
    sys::jobject,
    sys::{jfloat, jint, jstring},
    JNIEnv,
};
use serde::{Serialize, Deserialize};

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/*
 * Class:     com_example_rust_cross_jni_RustCrossJni
 * Method:    add
 * Signature: (II)I
 */
#[no_mangle]
pub extern "C" fn Java_com_example_rust_1cross_1jni_RustCrossJni_add(
    env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    a + b
}

/*
 * Class:     com_example_rust_cross_jni_RustCrossJni
 * Method:    gen_quaternion_str
 * Signature: (FFFF)Ljava/lang/String;
 */
#[no_mangle]
pub extern "C" fn Java_com_example_rust_1cross_1jni_RustCrossJni_gen_1quaternion_1str(
    env: JNIEnv,
    jclass: JClass,
    x: jfloat,
    y: jfloat,
    z: jfloat,
    w: jfloat,
) -> jstring {
    let eq = Quaternion { x, y, z, w };
    let str_eq = serde_json::to_string(&eq);
    let output = env
        .new_string(str_eq.unwrap())
        .expect("Couldn't create java string!");
    // Finally, extract the raw pointer to return.
    output.into_inner()
}
