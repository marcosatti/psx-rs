use std::env;
use std::path::PathBuf;
use std::collections::HashSet;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
        ]
        .into_iter()
        .collect(),
    );

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    let builder = bindgen::Builder::default()
        .parse_callbacks(Box::new(ignored_macros))
        .header("../external/libmirage/wrapper.h")
        .rustfmt_bindings(true);

    if target.contains("linux") {
        builder
            .clang_arg("-I/usr/include/blkid")
            .clang_arg("-I/usr/include/glib-2.0")
            .clang_arg("-I/usr/include/libmirage-3.2")
            .clang_arg("-I/usr/include/libmount")
            .clang_arg("-I/usr/lib/glib-2.0/include")
            .clang_arg("-I/usr/lib/libffi-3.2.1/include")
            .generate()
            .unwrap()
            .write_to_file(out_path.join("libmirage-sys.rs"))
            .unwrap();
    } else if target.contains("windows") {
        unimplemented!("Windows not implemented (header gen)");
        // builder
        //     .clang_arg("-I")
        //     .generate()
        //     .unwrap()
        //     .write_to_file(out_path.join("libmirage-sys.rs"))
        //     .unwrap();
    } else {
        unimplemented!("Unsupported target");
    }

    if target.contains("linux") {
        println!("cargo:rustc-link-lib=mirage");
        println!("cargo:rustc-link-lib=gmodule-2.0");
        println!("cargo:rustc-link-lib=glib-2.0");
        println!("cargo:rustc-link-lib=gio-2.0");
        println!("cargo:rustc-link-lib=gobject-2.0");
    } else if target.contains("windows") {
        unimplemented!("Windows not implemented (linking)");
        // println!("cargo:rustc-link-search=");
        // println!("cargo:rustc-link-lib=mirage");
    } else {
        unimplemented!("Unsupported target");
    }
}
