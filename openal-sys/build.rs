use build_macros::external_build;
use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
struct ParsingCallback();
impl ParseCallbacks for ParsingCallback {}

fn main() {
    external_build("openal", "openal_sys_bindgen", ParsingCallback());
}
