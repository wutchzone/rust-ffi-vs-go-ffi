use libc::{c_int, c_void};
use std::fs;
use std::io::prelude::*;

type Callback = unsafe extern "C" fn(*mut c_void, *mut c_void, c_int) -> c_int;

extern "C" fn rust_read(_: *mut c_void, buf: *mut c_void, size: c_int) -> c_int {
    let file: &fs::File = unsafe { FILE.as_ref().unwrap() };
    let tmp_ptr = buf as *mut [u8];
    let n = unsafe {
        file.take(size as u64)
            .read(std::mem::transmute_copy::<&[u8]>(&buf))
            .unwrap()
    };
    //unsafe { libc::memcpy(buf, tmp.as_ptr() as *const c_void, n) };
    //println!("Read: {} want {}", n, size);
    return n as i32;
}

extern "C" fn rust_write(_: *mut c_void, buf: *mut c_void, size: c_int) -> c_int {
    let mut tmp = [0 as u8; 2 * 8192];
    let mut file: &fs::File = unsafe { FILE_OUTPUT.as_ref().unwrap() };

    unsafe { libc::memcpy(tmp.as_ptr() as *mut c_void, buf, size as usize) };
    file.write(&tmp).expect("not to fail");
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
            FILE_OUTPUT = Some(fs::File::create("trololo.ts").unwrap());
        }
        unsafe { rust_transmuxer(rust_read, rust_write) };
    }
}
