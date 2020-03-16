use std::path::PathBuf;
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Output {
    enable: bool,
}

pub fn external_check(external_name: &str) {
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

    let output: Output = serde_json::from_str(&output_str_stdout).unwrap();

    if output.enable {
        println!("cargo:rustc-cfg={}", external_name);
    }
}
