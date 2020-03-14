include!("../utility/external_build.rs");

fn main() {
    external_build!("openal", "openal-sys");
}
