use std::os::raw::c_char;
use std::ffi::{CString, CStr};

extern {
    fn puts(s: *const c_char);
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    let s = "Hello, Rust World";
    let s_null_term = CString::new(s).unwrap();
    unsafe {
        puts(s_null_term.as_ptr());
    }
    let n = unsafe {
        strlen(s_null_term.as_ptr())
    };

    println!("s.len() is {}", n);
}

