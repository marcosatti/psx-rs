use build_tools::external_build;

fn main() {
    println!("cargo:rerun-if-changed=../external/libmirage/build.py");
    println!("cargo:rerun-if-changed=../external/libmirage/check.py");
    external_build("libmirage");
}
