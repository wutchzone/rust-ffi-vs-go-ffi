use libc::{c_int, c_void};
use std::fs;
use std::io::prelude::*;

type Callback = unsafe extern "C" fn(*mut c_void, *mut c_void, c_int) -> c_int;

extern "C" fn rust_read(_: *mut c_void, buf: *mut c_void, size: c_int) -> c_int {
    // This is safe because in C this value is hardcoded na size won't be higher
    // than this.
    let mut tmp: Vec<u8> = vec![0 as u8; size as _];
    let mut file: &fs::File = unsafe { FILE.as_ref().unwrap() };

    let n = file.read(&mut tmp).expect("read file");
    unsafe { libc::memcpy(buf, tmp.as_ptr() as *mut c_void, n as _) };

    return n as _;
}

extern "C" fn rust_write(_: *mut c_void, buf: *mut c_void, size: c_int) -> c_int {
    let mut file: &fs::File = unsafe { FILE_OUTPUT.as_ref().unwrap() };

    let slice: &[u8] = unsafe { std::slice::from_raw_parts(buf as _, size as _) };
    file.write(slice).expect("not to fail");

    return size;
}

extern "C" {
    fn rust_transmuxer(red: Callback, write: Callback) -> libc::c_int;
}

static mut FILE: Option<fs::File> = None;
static mut FILE_OUTPUT: Option<fs::File> = None;

fn main() {
    for _ in 0..400 {
        unsafe {
            FILE = Some(fs::File::open("../test.flv").unwrap());
            FILE_OUTPUT = Some(fs::File::create("test.ts").unwrap());
            rust_transmuxer(rust_read, rust_write);
        }
    }
}
