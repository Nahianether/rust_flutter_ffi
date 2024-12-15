use std::{ffi::c_char, time::Duration};

use ffi::{CstrToRust, RustToCstr};

pub mod ffi;

#[no_mangle]
pub extern "C" fn sum(a: usize, b: usize) -> usize {
    a + b
}

#[no_mangle]
pub extern "C" fn sum_async(a: usize, b: usize) -> usize {
    std::thread::sleep(Duration::from_secs(1));
    a + b
}

#[no_mangle]
pub extern "C" fn sum_str(a: *const c_char, b: *const c_char) -> *mut c_char {
    let av = a.to_native().parse::<usize>().unwrap();
    let bv = b.to_native().parse::<usize>().unwrap();
    let result = (av + bv).to_string();
    result.to_cstr()
}
