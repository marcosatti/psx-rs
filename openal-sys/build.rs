use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    bindgen::Builder::default()
        .header("../external/openal/wrapper.h")
        .clang_arg("-IC:/Devel/openal-soft-1.19.1-bin/include")
        .generate()
        .unwrap()
        .write_to_file(out_path.join("openal-sys.rs"))
        .unwrap();

    if target.contains("unknown-linux-gnu") {
        println!("cargo:rustc-link-lib=openal");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-search=C:/Devel/openal-soft-1.19.1-bin/libs/Win64");
        println!("cargo:rustc-link-lib=openal32");
    }
}
