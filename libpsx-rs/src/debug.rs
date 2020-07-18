use crate::{
    system::types::State,
    Core,
};
use log::debug;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
};

pub(crate) fn analysis(core: &mut Core) {
    let debug_path = core.config.workspace_path.join(r"debug/");
    std::fs::create_dir_all(&debug_path).unwrap();

    dump_memory(&mut core.state, &debug_path);
}

fn dump_memory(state: &mut State, base_dir_path: &PathBuf) {
    dump_memory_main(state, base_dir_path);
    dump_memory_spu(state, base_dir_path);
}

fn dump_memory_main(state: &State, base_dir_path: &PathBuf) {
    let memory_path = base_dir_path.join(r"main_memory.bin");
    let mut f = File::create(&memory_path).unwrap();
    f.write(&state.memory.main_memory.read_raw(0)).unwrap();
    debug!("Dumped main memory to {}", memory_path.to_str().unwrap());
}

fn dump_memory_spu(state: &mut State, base_dir_path: &PathBuf) {
    let memory_path = base_dir_path.join(r"spu_memory.bin");
    let mut f = File::create(&memory_path).unwrap();
    f.write(&state.spu.controller_state.get_mut().memory).unwrap();
    debug!("Dumped SPU memory to {}", memory_path.to_str().unwrap());
}
