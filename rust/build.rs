extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=test.c");
    println!("cargo:rerun-if-changed=test.h");
    cc::Build::new().file("test.c").compile("test");

    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avcodec");
}
