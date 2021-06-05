use crate::{
    external_check::external_check_inner,
    python,
};
use bindgen::Builder;
use serde::Deserialize;
use std::{
    env,
    fs::write,
    path::PathBuf,
};

#[derive(Deserialize, Debug)]
struct Output {
    include_paths: Vec<String>,
    header_paths: Vec<String>,
    library_search_paths: Vec<String>,
    library_names: Vec<String>,
    defines: Vec<String>,
    blocklist_item_regexes: Vec<String>,
    allowlist_function_regexes: Vec<String>,
    allowlist_type_regexes: Vec<String>,
    allowlist_variable_regexes: Vec<String>,
}

pub fn generate_external_include(external_name: &str) -> PathBuf {
    let out_file_name = format!("{}_build.rs", external_name);
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(out_file_name);
    println!("cargo:rustc-env=EXTERNAL_INCLUDE={}", out_file_path.to_str().unwrap());
    out_file_path
}

pub fn external_build(external_name: &str, cxx_mode: bool) {
    println!("cargo:rerun-if-changed=../external/{}/build.py", external_name);

    let out_file_path = generate_external_include(external_name);

    if !external_check_inner(external_name).enable {
        write(out_file_path, b"").unwrap();
        return;
    }

    let path = PathBuf::from(format!("external/{}/build.py", external_name));
    let (output_str_stdout, output_str_stderr) = python::run_script(&path);

    if false {
        panic!("Debug\nstdout: \n{}\nstderr: \n{}\n", output_str_stdout, output_str_stderr);
    }

    let output: Output = serde_json::from_str(&output_str_stdout).unwrap();

    let mut builder = Builder::default();
    builder = builder.rustfmt_bindings(false);
    builder = builder.derive_debug(false);
    builder = builder.layout_tests(false);
    builder = builder.generate_comments(false);

    if cxx_mode {
        builder = builder.enable_cxx_namespaces();
        builder = builder.clang_arg("--language=c++");
    }

    for define in output.defines {
        builder = builder.clang_arg(&format!("-D{}", &define));
    }

    for path in output.include_paths {
        builder = builder.clang_arg(&format!("-I{}", &path));
    }

    for header_path in output.header_paths {
        builder = builder.header(&header_path);
    }

    for blocklist_item_regex in output.blocklist_item_regexes {
        builder = builder.blocklist_item(blocklist_item_regex);
    }

    for allowlist_function_regex in output.allowlist_function_regexes {
        builder = builder.allowlist_function(allowlist_function_regex);
    }

    for allowlist_type_regex in output.allowlist_type_regexes {
        builder = builder.allowlist_type(allowlist_type_regex);
    }

    for allowlist_variable_regex in output.allowlist_variable_regexes {
        builder = builder.allowlist_var(allowlist_variable_regex);
    }

    builder.generate().unwrap().write_to_file(&out_file_path).unwrap();

    for library_search_path in output.library_search_paths {
        println!("cargo:rustc-link-search={}", &library_search_path);
    }

    for library_name in output.library_names {
        println!("cargo:rustc-link-lib={}", &library_name);
    }
}
