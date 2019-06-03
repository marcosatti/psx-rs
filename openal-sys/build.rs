use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    let builder = bindgen::Builder::default()
        .header("../external/openal/wrapper.h")
        .rustfmt_bindings(true);

    if target.contains("linux") {
        builder
            .generate()
            .unwrap()
            .write_to_file(out_path.join("openal-sys.rs"))
            .unwrap();
    } else if target.contains("windows") {
        builder
            .clang_arg("-IC:/Devel/openal-soft-1.19.1-bin/include")
            .generate()
            .unwrap()
            .write_to_file(out_path.join("openal-sys.rs"))
            .unwrap();
    } else {
        unimplemented!("Unsupported target");
    }

    if target.contains("linux") {
        println!("cargo:rustc-link-lib=openal");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-search=C:/Devel/openal-soft-1.19.1-bin/libs/Win64");
        println!("cargo:rustc-link-lib=openal32");
    } else {
        unimplemented!("Unsupported target");
    }
}
