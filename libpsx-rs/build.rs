use build_tools::{
    external_check,
    run_script,
};
use std::{
    env,
    fs::write,
    path::PathBuf,
};

fn main() {
    // OpenGL is always enabled.
    println!("cargo:warning=Enabling opengl");
    println!("cargo:rustc-cfg=opengl");

    external_check("openal");
    external_check("libmirage");
    external_check("libcdio");
    generate_instruction_lookup();
    generate_memory_map();
}

fn generate_instruction_lookup() {
    const SCRIPT_PATH: &str = "libpsx-rs/tools/Instruction Lookup Generation/Instruction Lookup Generation.py";
    const OUTPUT_FILE_NAME: &str = "instruction_lookup.rs";

    println!("cargo:warning=Generating instruction lookup table");

    let (stdout, _) = run_script(&PathBuf::from(SCRIPT_PATH));

    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(OUTPUT_FILE_NAME);
    write(&out_file_path, stdout.as_bytes()).unwrap();
    println!("cargo:rustc-env=GENERATED_INSTRUCTION_LOOKUP={}", out_file_path.to_str().unwrap());
}

fn generate_memory_map() {
    const SCRIPT_PATH: &str = "libpsx-rs/tools/Memory Map Generation/Memory Map Generation.py";
    const OUTPUT_FILE_NAME: &str = "memory_map.rs";

    println!("cargo:warning=Generating memory map");

    let (stdout, _) = run_script(&PathBuf::from(SCRIPT_PATH));

    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(OUTPUT_FILE_NAME);
    write(&out_file_path, stdout.as_bytes()).unwrap();
    println!("cargo:rustc-env=GENERATED_MEMORY_MAP={}", out_file_path.to_str().unwrap());
}
