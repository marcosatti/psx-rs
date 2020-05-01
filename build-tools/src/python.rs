use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};

pub(crate) fn bin_name() -> String {
    env::var("CI_PYTHON_BIN_NAME").unwrap_or(String::from("python"))
}

pub fn run_script(relative_path: &Path) -> (String, String) {
    let output = Command::new(bin_name()).current_dir(PathBuf::from("..")).arg(relative_path).output().unwrap();

    let output_str_stdout = String::from_utf8(output.stdout).unwrap();
    let output_str_stderr = String::from_utf8(output.stderr).unwrap();

    if !output.status.success() {
        panic!("Non-success return code: \nstdout: \n{}\nstderr: \n{}\n", &output_str_stdout, &output_str_stderr);
    }

    (output_str_stdout, output_str_stderr)
}
