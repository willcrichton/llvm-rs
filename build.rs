use std::env;

fn main() {
    println!("{}", "cargo:rustc-link-lib=dylib=ffi");
    let target = env::var("TARGET").unwrap();
    if target == "x86_64-apple-darwin" {
        let libs = vec!["z", "c++"];
        for lib in libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
}
