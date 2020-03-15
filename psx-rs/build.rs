use build_macros::external_check;

fn main() {
    external_check("opengl", "opengl");
    external_check("openal", "openal");
    external_check("libmirage", "libmirage");
}
