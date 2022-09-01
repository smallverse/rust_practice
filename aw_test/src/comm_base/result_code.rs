use std::sync::atomic::{AtomicI32, AtomicPtr};

const CODE_SUCCESS: AtomicI32 = AtomicI32::from(1000);
const CODE_FAILED: AtomicI32 = AtomicI32::from(1001);
const CODE_VALIDATE_FAILED: AtomicI32 = AtomicI32::from(1002);
const CODE_ERROR: AtomicI32 = AtomicI32::from(500);

const MSG_SUCCESS: AtomicPtr<&str> = AtomicPtr::from(&mut "success");
const MSG_FAILED: AtomicPtr<&str> = AtomicPtr::from(&mut "failed");
const MSG_VALIDATE_FAILED: AtomicPtr<&str> = AtomicPtr::from(&mut "validate_failed");
const MSG_ERROR: AtomicPtr<&str> = AtomicPtr::from(&mut "error");
