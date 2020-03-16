use build_tools::external_build;
use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
struct ParsingCallback();
impl ParseCallbacks for ParsingCallback {}

fn main() {
    external_build("opengl", ParsingCallback());
}
