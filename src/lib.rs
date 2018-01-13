#![feature(libc)]
extern crate libc;
use std::os::raw::c_char;
// use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::mem;

mod externs;

// #[macro_use]
// extern crate stdweb;

#[no_mangle]
pub fn get_str() -> *mut c_char {
  let s = CString::new("ASDF").unwrap();
  s.into_raw()
}

#[no_mangle]
pub fn do_console() {
  println!("WHY YOU NO WORk");
  // js! { alert(); }
  externs::alert("HELLO");
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf); // This is JS' responsibility now
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn time() -> String {
    String::from("Beer o'clock")
}