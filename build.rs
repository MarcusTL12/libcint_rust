// use std::env;

use cmake;

fn main() {
    println!("Hei build!");

    let dst = cmake::build("libcint");

    println!("{}", dst.display());

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=cint");
}
