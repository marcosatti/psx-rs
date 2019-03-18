use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    bindgen::Builder::default()
        .header("../external/opengl/wrapper.h")
        .clang_arg("-IC:/Devel/mesa/include")
        .clang_arg("-IC:/Devel/mesa-18.2.6-devel/include/x64")
        .rustfmt_bindings(false)
        .derive_debug(false)
        .generate()
        .unwrap()
        .write_to_file(out_path.join("opengl-sys.rs"))
        .unwrap();

    if target.contains("unknown-linux-gnu") {
        println!("cargo:rustc-link-lib=GL");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-search=C:/Devel/mesa-18.2.6-devel/lib/x64/gallium/targets/libgl-gdi");
        println!("cargo:rustc-link-lib=opengl32");
    }
}
