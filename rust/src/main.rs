use std::{
    ffi::c_int,
    fs::File,
    io::{Read, Write},
};

use co3::{ffi, raw};

struct Ctx {
    input: File,
    output: File,
}

ffi! {
    #![unsafe(export("C"))]

    type Ctx;
}

impl Ctx {
    fn rust_read(&mut self, buffer: &mut [u8]) -> c_int {
        self.input.read(buffer).expect("read file") as c_int
    }

    fn rust_write(&mut self, buffer: &[u8]) -> c_int {
        self.output.write_all(buffer).expect("write file");
        buffer.len() as c_int
    }
}

raw! {
    impl Ctx {
        // Generates: `unsafe extern "C" fn rust_read_raw(*mut c_void, *mut u8, c_int) -> c_int`
        // `Ctx::rust_read_raw` decodes the arguments as `Rust` types and calls `Ctx::rust_read`
        fn rust_read(&mut self, #[unpack(_, c_int)] buffer: &mut [u8]) -> c_int;

        // Generates `unsafe extern "C" fn rust_write_raw(*mut c_void, *const u8, c_int) -> c_int`
        // `Ctx::rust_write_raw` decodes the arguments as `Rust` types and calls `Ctx::rust_write`
        fn rust_write(&mut self, #[unpack(_, c_int)] buffer: &[u8]) -> c_int;
    }
}

ffi! {
    #![unsafe(extern("C"))]

    // `raw fn` is a function whose arguments are lowered into C types
    type WriteCallback = raw fn(&mut Ctx, #[unpack(_, c_int)] &[u8]) -> c_int;
    type ReadCallback = raw fn(&mut Ctx, #[unpack(_, c_int)] &mut [u8]) -> c_int;

    impl Ctx {
        // Generates `Ctx::rust_transmuxer(read: ReadCallback, write: WriteCallback) -> c_int`
        // wrapper that imports the C `rust_transmuxer` symbol with checked conversions.
        //
        // The wrapper is considered safe provided that signature was declared correctly
        #[symbol_name = "rust_transmuxer"]
        fn rust_transmuxer(&mut self, read: ReadCallback, write: WriteCallback) -> c_int;
    }
}

fn main() {
    for _ in 0..400 {
        let mut context = Ctx {
            input: File::open("../test.flv").unwrap(),
            output: File::create("test.ts").unwrap(),
        };

        context.rust_transmuxer(Ctx::rust_read_raw, Ctx::rust_write_raw);
    }
}
