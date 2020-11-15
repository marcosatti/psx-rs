use build_tools::external_build;

fn main() {
    println!("cargo:rerun-if-changed=../external/openal/build.py");
    println!("cargo:rerun-if-changed=../external/openal/check.py");
    external_build("openal", false);
}
