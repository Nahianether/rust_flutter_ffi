use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(s));
    }
}

fn free_dart_string(s: *const c_char) {
    if s.is_null() {
        return;
    }
    // let ss = unsafe { CStr::from_ptr(s).to_string_lossy().into_owned() };
    // println!("Before free: {:?}", ss);
    unsafe {
        libc::free(s as *mut libc::c_void);
    }
    // let ss = unsafe { CStr::from_ptr(s).to_string_lossy().into_owned() };
    // println!("After free: {:?}", ss);
}

pub trait CstrToRust {
    fn to_native(&self) -> String;
}

impl CstrToRust for *const c_char {
    fn to_native(&self) -> String {
        if self.is_null() {
            return String::new();
        }
        let v = unsafe { CStr::from_ptr(*self).to_string_lossy().into_owned() };
        free_dart_string(*self);
        v
    }
}

pub trait RustToCstr {
    fn to_cstr(&self) -> *mut c_char;
}

impl RustToCstr for String {
    fn to_cstr(&self) -> *mut c_char {
        if self.is_empty() {
            return std::ptr::null_mut();
        }
        let c_str = CString::new(self.as_str()).unwrap();
        c_str.into_raw()
    }
}

impl RustToCstr for &str {
    fn to_cstr(&self) -> *mut c_char {
        if self.is_empty() {
            return std::ptr::null_mut();
        }
        let c_str = CString::new(*self).unwrap();
        c_str.into_raw()
    }
}
