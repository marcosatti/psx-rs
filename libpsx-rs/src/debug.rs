pub mod benchmark;

use crate::system::types::State;
use crate::Core;
use log::debug;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

pub static DEBUG_CORE_EXIT: AtomicBool = AtomicBool::new(false);

pub fn analysis(core: &mut Core) {
    debug!("Core debug analysis:");
    let debug_path = core.config.workspace_path.join(r"debug/");
    std::fs::create_dir_all(&debug_path).unwrap();
    dump_memory(&debug_path, &core.state);
    unsafe {
        trace(core.state.as_mut().get_unchecked_mut());
    }
}

pub fn dump_memory(base_dir_path: &PathBuf, state: &State) {
    dump_memory_main(state, base_dir_path);
    dump_memory_spu(state, base_dir_path);
}

pub fn dump_memory_main(state: &State, base_dir_path: &PathBuf) {
    let memory_path = base_dir_path.join(r"main_memory.bin");
    let mut f = File::create(&memory_path).unwrap();
    f.write(&state.main_memory.read_raw(0)).unwrap();
    debug!("Dumped main memory to {}", memory_path.to_str().unwrap());
}

pub fn dump_memory_spu(state: &State, base_dir_path: &PathBuf) {
    let memory_path = base_dir_path.join(r"spu_memory.bin");
    let mut f = File::create(&memory_path).unwrap();
    f.write(&state.spu.memory.read_raw(0)).unwrap();
    debug!("Dumped SPU memory to {}", memory_path.to_str().unwrap());
}

pub fn trace(state: &mut State) {
    trace_r3000(state);
    trace_intc(state, false);
    trace_dmac(state, false);
    trace_timers(state);
    trace_cdrom(state);
}

pub fn trace_r3000(state: &State) {
    crate::system::r3000::controllers::debug::trace_pc(state);
    crate::system::r3000::controllers::debug::disassembler::trace_instructions_at_pc(state, None);
    crate::system::r3000::controllers::debug::register::trace_registers(state);
}

pub fn trace_intc(state: &State, only_enabled: bool) {
    crate::system::intc::controllers::debug::trace_intc(state, only_enabled, false);
}

pub fn trace_dmac(state: &State, only_enabled: bool) {
    crate::system::dmac::controllers::debug::trace_dmac(state, only_enabled);
}

pub fn trace_timers(state: &mut State) {
    crate::system::timers::controllers::debug::trace_timers(state);
}

pub fn trace_cdrom(state: &State) {
    crate::system::cdrom::controllers::debug::trace_cdrom(state);
}
