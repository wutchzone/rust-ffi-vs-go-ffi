extern crate cc;

fn main() {
    cc::Build::new().file("vertigo.c").compile("rust_vertigo");

    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avcodec");
}
