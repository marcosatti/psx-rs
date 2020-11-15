use build_tools::generate_external_include;
use gl_generator::*;
use std::fs::File;

fn main() {
    let external_name = "opengl";

    let out_file_path = generate_external_include(external_name);
    let mut file = File::create(&out_file_path).unwrap();
    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::None, []).write_bindings(bindings_generator::OpenglGenerator, &mut file).unwrap();
}

mod bindings_generator {
    use gl_generator::*;

    // Copyright 2015 Brendan Zabarauskas and the gl-rs developers
    //
    // Licensed under the Apache License, Version 2.0 (the "License");
    // you may not use this file except in compliance with the License.
    // You may obtain a copy of the License at
    //
    //     http://www.apache.org/licenses/LICENSE-2.0
    //
    // Unless required by applicable law or agreed to in writing, software
    // distributed under the License is distributed on an "AS IS" BASIS,
    // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    // See the License for the specific language governing permissions and
    // limitations under the License.

    // Modifications:
    // Prefix generated functions / enums with gl and/or GL_.
    // Removed fallback functionality as it's not needed.
    // Removed comments as they are not needed (reduce size).

    use std::io;
    use Registry;

    #[allow(missing_copy_implementations)]
    pub struct OpenglGenerator;

    impl Generator for OpenglGenerator {
        fn write<W>(&self, registry: &Registry, dest: &mut W) -> io::Result<()>
        where W: io::Write {
            write_header(dest)?;
            write_metaloadfn(dest)?;
            write_type_aliases(registry, dest)?;
            write_enums(registry, dest)?;
            write_fns(registry, dest)?;
            write_fnptr_struct_def(dest)?;
            write_ptrs(registry, dest)?;
            write_fn_mods(registry, dest)?;
            write_panicking_fns(registry, dest)?;
            write_load_fn(registry, dest)?;
            Ok(())
        }
    }

    fn write_header<W>(dest: &mut W) -> io::Result<()>
    where W: io::Write {
        writeln!(
            dest,
            r#"
            mod __gl_imports {{
                pub use std::mem;
                pub use std::os::raw;
            }}
        "#
        )
    }

    fn write_metaloadfn<W>(dest: &mut W) -> io::Result<()>
    where W: io::Write {
        writeln!(
            dest,
            r#"
            #[inline(never)]
            fn metaloadfn(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void,
                        symbol: &'static str) -> *const __gl_imports::raw::c_void {{
                loadfn(symbol)
            }}
        "#
        )
    }

    fn write_type_aliases<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
    where W: io::Write {
        writeln!(
            dest,
            r#"
            pub mod types {{
                #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
        "#
        )?;

        generators::gen_types(registry.api, dest)?;

        writeln!(
            dest,
            "
            }}
        "
        )
    }

    fn write_enums<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
    where W: io::Write {
        for enm in &registry.enums {
            let mut prefixed_enum = enm.clone();
            prefixed_enum.ident = format!("{}_{}", generators::gen_symbol_name(registry.api, "").to_uppercase(), enm.ident);
            generators::gen_enum_item(&prefixed_enum, "types::", dest)?;
        }

        Ok(())
    }

    fn write_fns<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
    where W: io::Write {
        for cmd in &registry.cmds {
            writeln!(
                dest,
                "#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
                pub unsafe fn {name}({params}) -> {return_suffix} {{ \
                    __gl_imports::mem::transmute::<_, extern \"system\" fn({typed_params}) -> {return_suffix}>\
                        (storage::{name}.f)({idents}) \
                }}",
                name = generators::gen_symbol_name(registry.api, &cmd.proto.ident),
                params = generators::gen_parameters(cmd, true, true).join(", "),
                typed_params = generators::gen_parameters(cmd, false, true).join(", "),
                return_suffix = cmd.proto.ty,
                idents = generators::gen_parameters(cmd, true, false).join(", "),
            )?;
        }

        Ok(())
    }

    fn write_fnptr_struct_def<W>(dest: &mut W) -> io::Result<()>
    where W: io::Write {
        writeln!(
            dest,
            "
            #[allow(missing_copy_implementations)]
            pub struct FnPtr {{
                f: *const __gl_imports::raw::c_void,
                is_loaded: bool,
            }}

            impl FnPtr {{
                pub fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {{
                    if ptr.is_null() {{
                        FnPtr {{ f: missing_fn_panic as *const __gl_imports::raw::c_void, is_loaded: false }}
                    }} else {{
                        FnPtr {{ f: ptr, is_loaded: true }}
                    }}
                }}
            }}
        "
        )
    }

    fn write_ptrs<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
    where W: io::Write {
        writeln!(
            dest,
            "mod storage {{
                #![allow(non_snake_case)]
                #![allow(non_upper_case_globals)]
                use super::__gl_imports::raw;
                use super::FnPtr;"
        )?;

        for c in &registry.cmds {
            writeln!(
                dest,
                "pub static mut {name}: FnPtr = FnPtr {{
                    f: super::missing_fn_panic as *const raw::c_void,
                    is_loaded: false
                }};",
                name = generators::gen_symbol_name(registry.api, &c.proto.ident),
            )?;
        }

        writeln!(dest, "}}")
    }

    fn write_fn_mods<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
    where W: io::Write {
        for c in &registry.cmds {
            let fnname = generators::gen_symbol_name(registry.api, &c.proto.ident);
            let symbol = generators::gen_symbol_name(registry.api, &c.proto.ident[..]);
            let symbol = &symbol[..];

            writeln!(
                dest,
                r##"
                #[allow(non_snake_case)]
                pub mod {fnname} {{
                    use super::{{storage, metaloadfn}};
                    use super::__gl_imports::raw;
                    use super::FnPtr;

                    #[inline]
                    #[allow(dead_code)]
                    pub fn is_loaded() -> bool {{
                        unsafe {{ storage::{fnname}.is_loaded }}
                    }}

                    #[allow(dead_code)]
                    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {{
                        unsafe {{
                            storage::{fnname} = FnPtr::new(metaloadfn(&mut loadfn, "{symbol}"))
                        }}
                    }}
                }}
            "##,
                fnname = fnname,
                symbol = symbol
            )?;
        }

        Ok(())
    }

    fn write_panicking_fns<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
    where W: io::Write {
        writeln!(
            dest,
            "#[inline(never)]
            fn missing_fn_panic() -> ! {{
                panic!(\"{api} function was not loaded\")
            }}
            ",
            api = registry.api
        )
    }

    fn write_load_fn<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
    where W: io::Write {
        writeln!(
            dest,
            "
            #[allow(dead_code)]
            pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const __gl_imports::raw::c_void {{
                #[inline(never)]
                fn inner(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void) {{
        "
        )?;

        for c in &registry.cmds {
            writeln!(dest, "{cmd_name}::load_with(&mut *loadfn);", cmd_name = generators::gen_symbol_name(registry.api, &c.proto.ident),)?;
        }

        writeln!(
            dest,
            "
                }}

                inner(&mut loadfn)
            }}
        "
        )
    }
}
