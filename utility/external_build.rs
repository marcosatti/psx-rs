macro_rules! external_build {
    ($external_folder:literal, $out_file_name:literal, $ignore_macros:expr) => {
        {
            use std::env;
            use std::path::PathBuf;
            use std::process::Command;
            use bindgen::Builder;
            use serde::Deserialize;
            use bindgen::callbacks::{ParseCallbacks, MacroParsingBehavior};
    
            #[derive(Deserialize, Debug)]
            struct Output {
                include_paths: Vec<String>,
                header_paths: Vec<String>,
                library_search_paths: Vec<String>,
                library_names: Vec<String>,
                defines: Vec<String>,
            }

            #[derive(Debug)]
            struct ParsingOptions(Vec<&'static str>);

            impl ParseCallbacks for ParsingOptions {
                fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
                    if self.0.contains(&name) {
                        MacroParsingBehavior::Ignore
                    } else {
                        MacroParsingBehavior::Default
                    }
                }
            }
    
            let output = Command::new("python")
                .current_dir(PathBuf::from(".."))
                .arg(concat!("external/", $external_folder, "/build.py"))
                .output()
                .unwrap();
    
            let output_str_stdout = String::from_utf8(output.stdout).unwrap();
            let output_str_stderr = String::from_utf8(output.stderr).unwrap();
    
            if !output.status.success() {
                panic!("Non-success return code: \nstdout: \n{}\nstderr: \n{}\n", &output_str_stdout, &output_str_stderr);
            }
    
            let output: Output = serde_json::from_str(&output_str_stdout).unwrap();
    
            let mut builder = Builder::default();
            builder = builder.parse_callbacks(Box::new(ParsingOptions(ignore_macros)));
            builder = builder.rustfmt_bindings(false);
            builder = builder.derive_debug(false);
    
            // Add in defines.
            for define in output.defines {
                builder = builder.clang_arg(&format!("-D{}", &define));
            }
    
            // Add include paths.
            for path in output.include_paths {
                builder = builder.clang_arg(&format!("-I{}", &path));
            }
    
            // Add headers.
            for header_path in output.header_paths {
                builder = builder.header(&header_path);
            }
    
            // Generate bindings.
            let out_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join(concat!($out_file_name, ".rs"));
            builder
                .generate()
                .unwrap()
                .write_to_file(out_file)
                .unwrap();
    
            // Add library search paths.
            for library_search_path in output.library_search_paths {
                println!("cargo:rustc-link-search={}", &library_search_path);
            }
            
            // Add library names.
            for library_name in output.library_names {
                println!("cargo:rustc-link-lib={}", &library_name);
            }
        }
    };
}
