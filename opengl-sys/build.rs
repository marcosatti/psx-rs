include!("../utility/external_build.rs");

fn main() {
    external_build!("opengl", "opengl_sys_bindgen", vec![]);
}
