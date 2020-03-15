use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
struct ParsingCallback();
impl ParseCallbacks for ParsingCallback {}

include!("../utilities/external_build.rs");

fn main() {
    external_build!("opengl", "opengl_sys_bindgen", ParsingCallback());
}
