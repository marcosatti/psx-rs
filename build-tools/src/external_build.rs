use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::fs::write;
use bindgen::Builder;
use bindgen::callbacks::ParseCallbacks;
use serde::Deserialize;
use crate::external_check::external_check_inner;
use crate::python;

#[derive(Deserialize, Debug)]
struct Output {
    include_paths: Vec<String>,
    header_paths: Vec<String>,
    library_search_paths: Vec<String>,
    library_names: Vec<String>,
    defines: Vec<String>,
}

pub fn external_build<T: 'static + ParseCallbacks>(external_name: &str, parsing_callback: T) {
    let out_file_name = format!("{}_build.rs", external_name);
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(out_file_name);
    println!("cargo:rustc-env=EXTERNAL_INCLUDE={}", out_file_path.to_str().unwrap());

    if !external_check_inner(external_name).enable {
        write(out_file_path, b"").unwrap();
        return;
    }

    let output = Command::new(python::bin_name())
        .current_dir(PathBuf::from(".."))
        .arg(format!("external/{}/build.py", external_name))
        .output()
        .unwrap();

    let output_str_stdout = String::from_utf8(output.stdout).unwrap();
    let output_str_stderr = String::from_utf8(output.stderr).unwrap();

    if !output.status.success() {
        panic!("Non-success return code: \nstdout: \n{}\nstderr: \n{}\n", &output_str_stdout, &output_str_stderr);
    }

    if false {
        panic!("Debug\nstdout: \n{}\nstderr: \n{}\n", output_str_stdout, output_str_stderr);
    }

    let output: Output = serde_json::from_str(&output_str_stdout).unwrap();

    let mut builder = Builder::default();
    builder = builder.parse_callbacks(Box::new(parsing_callback));
    builder = builder.rustfmt_bindings(false);
    builder = builder.derive_debug(false);

    // Add in defines.
    for define in output.defines {
        builder = builder.clang_arg(&format!("-D{}", &define));
    }

    // Add include paths.
    for path in output.include_paths {
        builder = builder.clang_arg(&format!("-I{}", &path));
    }

    // Add headers.
    for header_path in output.header_paths {
        builder = builder.header(&header_path);
    }

    // Generate bindings.
    builder
        .generate()
        .unwrap()
        .write_to_file(&out_file_path)
        .unwrap();

    // Add library search paths.
    for library_search_path in output.library_search_paths {
        println!("cargo:rustc-link-search={}", &library_search_path);
    }
    
    // Add library names.
    for library_name in output.library_names {
        println!("cargo:rustc-link-lib={}", &library_name);
    }
}
