include!("../utilities/external_check.rs");

fn main() {
    external_check!("opengl", "opengl");
    external_check!("openal", "openal");
    external_check!("libmirage", "libmirage");
}
