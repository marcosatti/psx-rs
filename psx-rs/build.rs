use build_tools::external_check;

fn main() {
    external_check("opengl");
    external_check("openal");
    external_check("libmirage");
}
