use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};

pub(crate) fn bin_name() -> String {
    let try_exec = |bin_name: &str| {
        if let Ok(mut child) = Command::new(bin_name).arg("--version").spawn() {
            child.wait().unwrap().success()
        } else {
            false
        }
    };

    if let Ok(bin_name) = env::var("CI_PYTHON_BIN_NAME") {
        if try_exec(&bin_name) {
            return bin_name;
        }
    }

    if try_exec("python3") {
        return "python3".to_owned();
    }

    if try_exec("python") {
        return "python".to_owned();
    }

    panic!("Cannot determine Python 3 bin name");
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
