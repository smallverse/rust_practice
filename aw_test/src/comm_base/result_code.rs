use std::sync::atomic::{AtomicI32, AtomicPtr};

pub const CODE_SUCCESS: AtomicI32 = AtomicI32::from(1000);
pub const CODE_FAILED: AtomicI32 = AtomicI32::from(1001);
pub const CODE_VALIDATE_FAILED: AtomicI32 = AtomicI32::from(1002);
pub const CODE_ERROR: AtomicI32 = AtomicI32::from(500);

pub const MSG_SUCCESS: AtomicPtr<String> = AtomicPtr::from(&mut String::from("success"));
pub const MSG_FAILED: AtomicPtr<String> = AtomicPtr::from(&mut String::from("failed"));
pub const MSG_VALIDATE_FAILED: AtomicPtr<String> =
    AtomicPtr::from(&mut String::from("validate_failed"));
pub const MSG_ERROR: AtomicPtr<String> = AtomicPtr::from(&mut String::from("error"));
