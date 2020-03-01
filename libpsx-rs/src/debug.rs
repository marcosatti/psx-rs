pub mod benchmark;

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::AtomicBool;
use log::debug;
use crate::Core;
use crate::resources::Resources;

pub static DEBUG_CORE_EXIT: AtomicBool = AtomicBool::new(false);

pub fn analysis(core: &mut Core) {
    debug!("Core debug analysis:");
    let debug_path = core.config.workspace_path.join(r"debug/");
    std::fs::create_dir_all(&debug_path).unwrap();
    dump_memory(&debug_path, &core.resources);
    unsafe {
        trace(core.resources.as_mut().get_unchecked_mut());
    }
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

pub fn trace(resources: &mut Resources) {
    trace_r3000(resources);
    trace_intc(resources, false);
    trace_dmac(resources, false);
    trace_timers(resources);
    trace_cdrom(resources);
}

pub fn trace_r3000(resources: &Resources) {
    crate::controllers::r3000::debug::trace_pc(resources);
    crate::controllers::r3000::debug::disassembler::trace_instructions_at_pc(resources, None);
    crate::controllers::r3000::debug::register::trace_registers(resources);
}

pub fn trace_intc(resources: &Resources, only_enabled: bool) {
    crate::controllers::intc::debug::trace_intc(resources, only_enabled, false);
}

pub fn trace_dmac(resources: &Resources, only_enabled: bool) {
    crate::controllers::dmac::debug::trace_dmac(resources, only_enabled);
}

pub fn trace_timers(resources: &mut Resources) {
    crate::controllers::timers::debug::trace_timers(resources);
}

pub fn trace_cdrom(resources: &Resources) {
    crate::controllers::cdrom::debug::trace_cdrom(resources);
}
