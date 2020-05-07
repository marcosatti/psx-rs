pub mod benchmark;

use crate::{
    system::types::State,
    Core,
};
use log::debug;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::atomic::AtomicBool,
};

pub static DEBUG_CORE_EXIT: AtomicBool = AtomicBool::new(false);

pub fn analysis(core: &mut Core) {
    debug!("Core debug analysis:");
    let debug_path = core.config.workspace_path.join(r"debug/");
    std::fs::create_dir_all(&debug_path).unwrap();
    dump_memory(&mut core.state, &debug_path);
    trace(&mut core.state);
}

pub fn dump_memory(state: &mut State, base_dir_path: &PathBuf) {
    dump_memory_main(state, base_dir_path);
    dump_memory_spu(state, base_dir_path);
}

pub fn dump_memory_main(state: &State, base_dir_path: &PathBuf) {
    let memory_path = base_dir_path.join(r"main_memory.bin");
    let mut f = File::create(&memory_path).unwrap();
    f.write(&state.memory.main_memory.read_raw(0)).unwrap();
    debug!("Dumped main memory to {}", memory_path.to_str().unwrap());
}

pub fn dump_memory_spu(state: &mut State, base_dir_path: &PathBuf) {
    let memory_path = base_dir_path.join(r"spu_memory.bin");
    let mut f = File::create(&memory_path).unwrap();
    f.write(&state.spu.controller_state.get_mut().memory).unwrap();
    debug!("Dumped SPU memory to {}", memory_path.to_str().unwrap());
}

pub fn trace(state: &mut State) {
    trace_r3000(state);
    trace_intc(state, false);
    trace_dmac(state, false);
    trace_timers(state);
    trace_cdrom(state);
}

pub fn trace_r3000(state: &mut State) {
    let r3000_state = state.r3000.controller_state.get_mut();
    let cp0_state = state.r3000.cp0.controller_state.get_mut();
    crate::system::r3000::controllers::debug::trace_pc(r3000_state, cp0_state);
    // crate::system::r3000::controllers::debug::disassembler::trace_instructions_at_pc(&state.memory.main_memory,
    // &state.memory.bios, r3000_state.pc.read_u32(), None);
    // crate::system::r3000::controllers::debug::register::trace_registers(r3000_state);
}

pub fn trace_intc(state: &mut State, only_enabled: bool) {
    crate::system::intc::controllers::debug::trace_intc(state, only_enabled, false);
}

pub fn trace_dmac(state: &mut State, only_enabled: bool) {
    crate::system::dmac::controllers::debug::trace_dmac(state, only_enabled);
}

pub fn trace_timers(_state: &mut State) {
    //crate::system::timers::controllers::debug::trace_timers(state);
}

pub fn trace_cdrom(state: &State) {
    crate::system::cdrom::controllers::debug::trace_cdrom(state);
}
