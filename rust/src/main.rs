use std::{
    ffi::c_int,
    fs::File,
    io::{Read, Write},
    sync::Mutex,
};

use co3::{ffi, raw};

fn rust_read(_context: Option<&mut Ctx>, buffer: &mut [u8]) -> c_int {
    let mut input = FILE.lock().unwrap();
    input.as_mut().unwrap().read(buffer).expect("read file") as c_int
}

fn rust_write(_context: Option<&mut Ctx>, buffer: &[u8]) -> c_int {
    let mut output = FILE_OUTPUT.lock().unwrap();
    output
        .as_mut()
        .unwrap()
        .write_all(buffer)
        .expect("write file");
    buffer.len() as c_int
}

raw! {
    // Generates: `unsafe extern "C" fn rust_read_raw(*mut c_void, *mut u8, c_int) -> c_int`
    // `rust_read_raw` decodes the `C` arguments into `Rust` types and calls `rust_read`.
    fn rust_read(context: Option<&mut Ctx>, #[unpack(_, c_int)] buffer: &mut [u8]) -> c_int;

    // Generates `unsafe extern "C" fn rust_write_raw(*mut c_void, *const u8, c_int) -> c_int`
    // `rust_write_raw` decodes the`C`arguments into `Rust` types and calls `rust_write`.
    fn rust_write(context: Option<&mut Ctx>, #[unpack(_, c_int)] buffer: &[u8]) -> c_int;
}

ffi! {
    #![unsafe(extern("C"))]

    type Ctx;

    // `raw fn` is a function whose arguments are lowered into C types
    type WriteCallback = raw fn(Option<&mut Ctx>, #[unpack(_, c_int)] &[u8]) -> c_int;
    type ReadCallback = raw fn(Option<&mut Ctx>, #[unpack(_, c_int)] &mut [u8]) -> c_int;

    #[symbol_name = "rust_transmuxer"]
    // Generates `rust_transmuxer(read: ReadCallback, write: WriteCallback) -> c_int`
    // wrapper that imports the C `rust_transmuxer` symbol with checked conversions.
    //
    // The wrapper is considered safe provided that signature was declared correctly
    fn rust_transmuxer(read: ReadCallback, write: WriteCallback) -> c_int;
}

static FILE: Mutex<Option<File>> = Mutex::new(None);
static FILE_OUTPUT: Mutex<Option<File>> = Mutex::new(None);

fn main() {
    for _ in 0..400 {
        *FILE.lock().unwrap() = Some(File::open("../test.flv").unwrap());
        *FILE_OUTPUT.lock().unwrap() = Some(File::create("test.ts").unwrap());
        rust_transmuxer(rust_read_raw, rust_write_raw);
    }
}
