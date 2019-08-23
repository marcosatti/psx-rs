pub mod benchmark;

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::ffi::CStr;
use opengl_sys::*;
use log::debug;
use crate::Core;
use crate::resources::{self, Resources};

pub static ENABLE_DMAC_CHANNEL_TRACE: bool = false;
pub static ENABLE_FIFO_TRACE: bool = true;

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
    let pc = resources.r3000.pc.read_u32();
    let kuc = resources.r3000.cp0.status.read_bitfield(resources::r3000::cp0::STATUS_KUC);
    let iec = resources.r3000.cp0.status.read_bitfield(resources::r3000::cp0::STATUS_IEC);
    debug!("R3000 at pc = 0x{:0X}, kuc = {}, iec = {}", pc, kuc, iec);
}

pub fn trace_intc(resources: &Resources, only_enabled: bool) {
    use crate::resources::intc::*;
    let stat = unsafe { resources.intc.stat.register.value.v32 };
    let mask = unsafe { resources.intc.mask.value.v32 };
    for (name, bitfield) in IRQ_NAMES.iter().zip(IRQ_BITFIELDS.iter()) {
        let stat_value = bitfield.extract_from(stat) != 0;
        let mask_value = bitfield.extract_from(mask) != 0;
        let pending = stat_value && mask_value;

        if only_enabled && !mask_value {
            continue;
        }

        debug!("INTC [{}]: stat = {}, mask = {} (pending = {})", name, stat_value, mask_value, pending);
    }
}

pub fn trace_dmac(resources: &Resources, only_enabled: bool) {
    use crate::resources::dmac::*;

    let dpcr = unsafe { resources.dmac.dpcr.value.v32 };
    for (name, bitfield) in DMA_CHANNEL_NAMES.iter().zip(DPCR_CHANNEL_ENABLE_BITFIELDS.iter()) {
        let dpcr_value = bitfield.extract_from(dpcr) != 0;

        if only_enabled && !dpcr_value {
            continue;
        }

        debug!("DMAC DPCR [{}]: dma enabled = {}", name, dpcr_value);
    }

    let dicr = unsafe { resources.dmac.dicr.register.value.v32 };
    let dicr_irq_master_enable_value = DICR_IRQ_MASTER_ENABLE.extract_from(dicr) != 0;
    debug!("DMAC DICR: master enable = {}", dicr_irq_master_enable_value);
    let dicr_irq_force_value = DICR_IRQ_FORCE.extract_from(dicr) != 0;
    debug!("DMAC DICR: irq force = {}", dicr_irq_force_value);
    for (name, (enable_bitfield, flag_bitfield)) in DMA_CHANNEL_NAMES.iter().zip(DICR_IRQ_ENABLE_BITFIELDS.iter().zip(DICR_IRQ_FLAG_BITFIELDS.iter())) {
        let dicr_enable_value = enable_bitfield.extract_from(dicr) != 0; 
        let dicr_flag_value = flag_bitfield.extract_from(dicr) != 0; 

        if only_enabled && !dicr_enable_value {
            continue;
        }

        debug!("DMAC DICR [{}]: irq enabled = {}, irq flag = {}", name, dicr_enable_value, dicr_flag_value);
    }
    let dicr_irq_master_flag_value = DICR_IRQ_MASTER_FLAG.extract_from(dicr) != 0;
    debug!("DMAC DICR: master flag = {}", dicr_irq_master_flag_value);
}

pub extern "C" fn debug_opengl_trace(_source: GLenum, type_: GLenum, _id: GLuint, severity: GLenum, _length: GLsizei, message: *const GLchar, _user_param: *const std::ffi::c_void) {
    unsafe {
        if type_ == GL_DEBUG_TYPE_ERROR_ARB {
            let message = CStr::from_ptr(message);
            debug!("OpenGL error: type: {}, severity = {}, message = {}", type_, severity, message.to_str().unwrap());
        }
    }
}
