extern crate cc;

fn main() {
    cc::Build::new().file("test.c").compile("test");

    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avcodec");
}
