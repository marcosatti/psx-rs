use crate::python;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub(crate) struct Output {
    pub(crate) enable: bool,
}

pub(crate) fn external_check_inner(external_name: &str) -> Output {
    let path = PathBuf::from(format!("external/{}/check.py", external_name));
    let (output_str_stdout, output_str_stderr) = python::run_script(&path);

    if false {
        panic!("Debug\nstdout: \n{}\nstderr: \n{}\n", output_str_stdout, output_str_stderr);
    }

    serde_json::from_str(&output_str_stdout).unwrap()
}

pub fn external_check(external_name: &str) {
    let output = external_check_inner(external_name);

    if output.enable {
        println!("cargo:warning=Enabling {}", external_name);
        println!("cargo:rustc-cfg={}", external_name);
    } else {
        println!("cargo:warning=Disabling {}", external_name);
    }
}
