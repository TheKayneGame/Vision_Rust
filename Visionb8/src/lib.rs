#![feature(rustc_private)]
extern crate libc;
mod vision8b;

use libc::{c_char, uint32_t};
use std::ffi::CStr;
use std::str;

#[repr(C)]
pub struct SampleStruct {    
    pub field_one: i16,
    pub field_two: i32,
}

#[no_mangle]
pub extern fn count_eyes(path: *const c_char) -> u32 {
    5
}