macro_rules! external_build {
    ($external_folder:expr, $crate_name:expr) => {
        {
            use std::env;
            use std::path::PathBuf;
            use std::process::Command;
            use bindgen::Builder;
            use serde::Deserialize;

            #[derive(Deserialize, Debug)]
            struct Output {
                include_paths: Vec<String>,
                header_paths: Vec<String>,
                library_search_paths: Vec<String>,
                library_names: Vec<String>,
            }

            let output = Command::new("python")
                .arg(concat!("../external/", $external_folder, "/build.py"))
                .output()
                .unwrap();

            let output_str_stdout = String::from_utf8(output.stdout).unwrap();
            let output_str_stderr = String::from_utf8(output.stderr).unwrap();

            if !output.status.success() {
                panic!("Non-success return code: \nstdout: \n{}\nstderr: \n{}\n", &output_str_stdout, &output_str_stderr);
            }

            let output: Output = serde_json::from_str(&output_str_stdout).unwrap();

            let mut builder = Builder::default();
            builder = builder.rustfmt_bindings(true);

            // Add include paths.
            for path in output.include_paths {
                builder = builder.clang_arg(&format!("-I{}", &path));
            }

            // Add headers.
            for header_path in output.header_paths {
                builder = builder.header(&header_path);
            }

            // Generate bindings.
            builder
                .generate()
                .unwrap()
                .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join(concat!($crate_name, ".rs")))
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
