pub mod benchmark;

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::ffi::CStr;
use opengl_sys::*;
use log::debug;
use crate::Core;
use crate::resources::Resources;

pub static mut DEBUG_CORE_EXIT: bool = false;

pub fn analysis(core: &Core) {
    debug!("Core debug analysis:");
    let debug_path = core.config.workspace_path.join(r"debug/");
    std::fs::create_dir_all(&debug_path).unwrap();
    dump_memory(&debug_path, &core.resources);
    trace(&core.resources);
}

pub fn dump_memory(base_dir_path: &PathBuf, resources: &Resources) {
    dump_memory_main(resources, base_dir_path);
    dump_memory_spu(resources, base_dir_path);
}

pub fn dump_memory_main(resources: &Resources, base_dir_path: &PathBuf) {
    let memory_path = base_dir_path.join(r"main_memory.bin");
    let mut f = File::create(&memory_path).unwrap();
    f.write(&resources.main_memory.read_raw(0)).unwrap();
    debug!("Dumped main memory to {}", memory_path.to_str().unwrap());
}

pub fn dump_memory_spu(resources: &Resources, base_dir_path: &PathBuf) {
    let memory_path = base_dir_path.join(r"spu_memory.bin");
    let mut f = File::create(&memory_path).unwrap();
    f.write(&resources.spu.memory.read_raw(0)).unwrap();
    debug!("Dumped SPU memory to {}", memory_path.to_str().unwrap());
}

pub fn trace(resources: &Resources) {
    trace_r3000(resources);
    trace_intc(resources, false);
    trace_dmac(resources, false);
}

pub fn trace_r3000(resources: &Resources) {
    crate::controllers::r3000::debug::trace_pc(resources);
    crate::controllers::r3000::debug::disassembler::trace_instructions_at_pc(resources, None);
    crate::controllers::r3000::debug::register::trace_registers(resources);
}

pub fn trace_intc(resources: &Resources, only_enabled: bool) {
    crate::controllers::intc::debug::trace_intc(resources, only_enabled);
}

pub fn trace_dmac(resources: &Resources, only_enabled: bool) {
    crate::controllers::dmac::debug::trace_dmac(resources, only_enabled);
}

pub extern "C" fn debug_opengl_trace(_source: GLenum, type_: GLenum, _id: GLuint, severity: GLenum, _length: GLsizei, message: *const GLchar, _user_param: *const std::ffi::c_void) {
    unsafe {
        if type_ == GL_DEBUG_TYPE_ERROR_ARB {
            let message = CStr::from_ptr(message);
            debug!("OpenGL error: type: {}, severity = {}, message = {}", type_, severity, message.to_str().unwrap());
        }
    }
}
