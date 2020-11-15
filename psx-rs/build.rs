use build_tools::external_check;

fn main() {
    // OpenGL is always enabled.
    println!("cargo:warning=Enabling opengl");
    println!("cargo:rustc-cfg=opengl");

    external_check("openal");
    external_check("libmirage");
    external_check("libcdio");
}
