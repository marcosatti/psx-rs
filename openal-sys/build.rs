include!("../utility/external_build.rs");

fn main() {
    external_build!("openal", "openal_sys_bindgen", vec![]);
}
