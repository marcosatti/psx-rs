use build_tools::external_check;

fn main() {
    external_check("opengl", "opengl");
    external_check("openal", "openal");
    external_check("libmirage", "libmirage");
}
