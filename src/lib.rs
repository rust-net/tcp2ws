#[path ="tcp2ws/mod.rs"]
mod tcp2ws;
mod log;
use std::ffi::{c_char, CStr};
use tcp2ws::*;

fn ptr2string(ptr: *const c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(ptr) } ;
    let str_slice = cstr.to_str().expect("Invalid UTF-8 sequence");
    let rust_string = String::from(str_slice);
    // 在这里可以使用rust_string
    d!("Rust String: {}", rust_string);
    rust_string
}

#[no_mangle]
pub extern "C" fn test() {
    tcp2ws::test();
}

#[no_mangle]
pub extern "C" fn start(name: *const c_char, ws: *const c_char, listen: *const c_char) -> bool {
    let ok = run(async {
        match service::start(config::Item { name: ptr2string(name), ws: ptr2string(ws), listen: ptr2string(listen) }).await {
            Ok(_) => true,
            _ => false,
        }
    });
    ok
}

#[no_mangle]
pub extern "C" fn stop(name: *const c_char, ws: *const c_char, listen: *const c_char) -> bool {
    let ok = run(async {
        match service::stop(config::Item { name: ptr2string(name), ws: ptr2string(ws), listen: ptr2string(listen) }).await {
            Ok(_) => true,
            _ => false,
        }
    });
    ok
}