use build_tools::external_build;

fn main() {
    println!("cargo:rerun-if-changed=../external/libcdio/build.py");
    println!("cargo:rerun-if-changed=../external/libcdio/check.py");
    external_build("libcdio", false);
}
