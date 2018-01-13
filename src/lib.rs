#![feature(libc)]
extern crate libc;
use std::os::raw::c_char;
// use std::ffi::CStr;
use std::ffi::CString;
// use std::os::raw::c_void;
// use std::mem;

mod externs;

// #[macro_use]
// extern crate stdweb;

#[no_mangle]
pub fn get_str() -> *mut c_char {
  let s = CString::new("ASDF").unwrap();
  s.into_raw()
}

#[no_mangle]
pub fn add_one(x: i32) -> i32 {
  x + 1
}

#[no_mangle]
pub fn alert_me() {
  // if you uncomment the following, along with #[macro_use] extern crate stdweb,
  // the file will still compile, but the wasm bundle will throw an error:
  // LinkError: WebAssembly Instantiation: Import #0 module="env" function="__js_1"
  // error: function import requires a callable
  //  at <anonymous>
  //
  // js! { alert(); }

  // this does nothing:
  externs::alert("HELLO");
}