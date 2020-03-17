use std::env;

pub(crate) fn bin_name() -> String {
    env::var("CI_PYTHON_BIN_NAME").unwrap_or(String::from("python"))
}
