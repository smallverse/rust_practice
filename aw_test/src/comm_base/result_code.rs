use std::sync::atomic::{AtomicI32, AtomicPtr};

const CODE_SUCCESS: AtomicI32 = AtomicI32::from(1000);
const CODE_FAILED: AtomicI32 = AtomicI32::from(1001);
const CODE_VALIDATE_FAILED: AtomicI32 = AtomicI32::from(1002);
const CODE_ERROR: AtomicI32 = AtomicI32::from(500);

const MSG_SUCCESS: AtomicPtr<&str> =  AtomicPtr::from("success");
const MSG_FAILED: AtomicI32 = AtomicI32::from(1001);
const MSG_VALIDATE_FAILED: AtomicI32 = AtomicI32::from(1002);
const MSG_ERROR: AtomicI32 = AtomicI32::from(500);
#[allow(non_camel_case_types)]
pub enum ResultCodeEnum {
    Papery { CODE:},
    // /** SUCCESS */
    // SUCCESS(1000, "success"),
    // /** FAILED */
    // FAILED(1001, "failed"),
    // /** VALIDATE_FAILED */
    // VALIDATE_FAILED(1002, "validate_failed"),
    // /** ERROR */
    // ERROR(5000, "error"),
}

enum Option<T> {
    Some(T),
    None,
}

