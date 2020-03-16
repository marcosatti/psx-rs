use std::path::PathBuf;
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Output {
    pub(crate) enable: bool,
}

pub(crate) fn external_check_inner(external_name: &str) -> Output {
    let output = Command::new("python")
        .current_dir(PathBuf::from(".."))
        .arg(format!("external/{}/check.py", external_name))
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

    serde_json::from_str(&output_str_stdout).unwrap()
}

pub fn external_check(external_name: &str) {
    let output = external_check_inner(external_name);

    if output.enable {
        println!("cargo:rustc-cfg={}", external_name);
    }
}
