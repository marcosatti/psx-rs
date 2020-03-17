use build_tools::external_build;
use bindgen::callbacks::ParseCallbacks;

#[derive(Copy, Clone, Debug)]
struct ParsingCallback(&'static [&'static str]);

impl ParseCallbacks for ParsingCallback {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(&name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

static CALLBACK: ParsingCallback = ParsingCallback(&[
    "FP_INFINITE", 
    "FP_NAN", 
    "FP_NORMAL", 
    "FP_SUBNORMAL", 
    "FP_ZERO"
]);

fn main() {
    external_build("libmirage", CALLBACK);
}
