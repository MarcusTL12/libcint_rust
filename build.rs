use cmake;

fn main() {
    let dst = cmake::build("libcint");

    println!("{}", dst.display());

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=cint");
}
