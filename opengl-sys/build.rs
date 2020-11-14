use build_tools::external_build;

fn main() {
    println!("cargo:rerun-if-changed=../external/opengl/build.py");
    println!("cargo:rerun-if-changed=../external/opengl/check.py");
    external_build("opengl");
}
