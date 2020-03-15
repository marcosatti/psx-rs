use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
struct ParsingCallback(Vec<&'static str>);

impl ParseCallbacks for ParsingCallback {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(&name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

include!("../utilities/external_build.rs");

fn main() {
    let callback = ParsingCallback(vec![
        "FP_INFINITE", 
        "FP_NAN", 
        "FP_NORMAL", 
        "FP_SUBNORMAL", 
        "FP_ZERO"
    ]);

    external_build!("libmirage", "libmirage_sys_bindgen", callback);
}
