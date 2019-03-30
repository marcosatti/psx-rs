use std::cell::UnsafeCell;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
use std::ffi::CStr;
use opengl_sys::*;
use log::debug;
use crate::Core;
use crate::resources::{self, Resources};

pub static mut DEBUG_CORE_EXIT: bool = false;

pub struct BenchmarkDebug {
    pub r3000_benchmark: UnsafeCell<Duration>,
    pub dmac_benchmark: UnsafeCell<Duration>,
    pub gpu_benchmark: UnsafeCell<Duration>,
    pub spu_benchmark: UnsafeCell<Duration>,
    pub gpu_crtc_benchmark: UnsafeCell<Duration>,
    pub spu_dac_benchmark: UnsafeCell<Duration>,
    pub intc_benchmark: UnsafeCell<Duration>,
}

impl BenchmarkDebug {
    pub const fn empty() -> BenchmarkDebug {
        BenchmarkDebug {
            r3000_benchmark: UnsafeCell::new(Duration::from_secs(0)),
            dmac_benchmark: UnsafeCell::new(Duration::from_secs(0)),
            gpu_benchmark: UnsafeCell::new(Duration::from_secs(0)),
            spu_benchmark: UnsafeCell::new(Duration::from_secs(0)),
            gpu_crtc_benchmark: UnsafeCell::new(Duration::from_secs(0)),
            spu_dac_benchmark: UnsafeCell::new(Duration::from_secs(0)),
            intc_benchmark: UnsafeCell::new(Duration::from_secs(0)),
        }
    }
}

unsafe impl Sync for BenchmarkDebug {}

static mut BENCHMARK_TIME_ELAPSED: Duration = Duration::from_secs(0);
static mut BENCHMARK_CONTROLLERS: BenchmarkDebug = BenchmarkDebug::empty();
static mut BENCHMARK_LAST_REPORTED: Option<Instant> = None;
static BENCHMARK_REPORTING_FREQUENCY: Duration = Duration::from_secs(3);

pub fn trace_performance(time_elapsed: &Duration, benchmark_results: &BenchmarkDebug) {
    unsafe {
        if BENCHMARK_LAST_REPORTED.is_none() {
            BENCHMARK_LAST_REPORTED = Some(Instant::now());
            BENCHMARK_TIME_ELAPSED = Duration::from_secs(0);
            BENCHMARK_CONTROLLERS = BenchmarkDebug::empty();
        }

        BENCHMARK_TIME_ELAPSED += *time_elapsed;
        *BENCHMARK_CONTROLLERS.r3000_benchmark.get() += *benchmark_results.r3000_benchmark.get();
        *BENCHMARK_CONTROLLERS.dmac_benchmark.get() += *benchmark_results.dmac_benchmark.get();
        *BENCHMARK_CONTROLLERS.gpu_benchmark.get() += *benchmark_results.gpu_benchmark.get();
        *BENCHMARK_CONTROLLERS.spu_benchmark.get() += *benchmark_results.spu_benchmark.get();
        *BENCHMARK_CONTROLLERS.gpu_crtc_benchmark.get() += *benchmark_results.gpu_crtc_benchmark.get();
        *BENCHMARK_CONTROLLERS.spu_dac_benchmark.get() += *benchmark_results.spu_dac_benchmark.get();
        *BENCHMARK_CONTROLLERS.intc_benchmark.get() += *benchmark_results.intc_benchmark.get();

        if BENCHMARK_LAST_REPORTED.unwrap().elapsed() > BENCHMARK_REPORTING_FREQUENCY {
            let overall_percentage = BENCHMARK_TIME_ELAPSED.as_secs_f64() / BENCHMARK_REPORTING_FREQUENCY.as_secs_f64() * 100.0;
            let r3000_percentage = BENCHMARK_TIME_ELAPSED.as_secs_f64() / (*BENCHMARK_CONTROLLERS.r3000_benchmark.get()).as_secs_f64() * 100.0;
            let dmac_percentage = BENCHMARK_TIME_ELAPSED.as_secs_f64() / (*BENCHMARK_CONTROLLERS.dmac_benchmark.get()).as_secs_f64() * 100.0;
            let gpu_percentage = BENCHMARK_TIME_ELAPSED.as_secs_f64() / (*BENCHMARK_CONTROLLERS.gpu_benchmark.get()).as_secs_f64() * 100.0;
            let spu_percentage = BENCHMARK_TIME_ELAPSED.as_secs_f64() / (*BENCHMARK_CONTROLLERS.spu_benchmark.get()).as_secs_f64() * 100.0;
            let gpu_crtc_percentage = BENCHMARK_TIME_ELAPSED.as_secs_f64() / (*BENCHMARK_CONTROLLERS.gpu_crtc_benchmark.get()).as_secs_f64() * 100.0;
            let spu_dac_percentage = BENCHMARK_TIME_ELAPSED.as_secs_f64() / (*BENCHMARK_CONTROLLERS.spu_dac_benchmark.get()).as_secs_f64() * 100.0;
            let intc_percentage = BENCHMARK_TIME_ELAPSED.as_secs_f64() / (*BENCHMARK_CONTROLLERS.intc_benchmark.get()).as_secs_f64() * 100.0;

            let time_elapsed = BENCHMARK_TIME_ELAPSED.as_micros();

            debug!(
                "Benchmark of {} us ({:.2}%): r3000 = {:.2}%, dmac = {:.2}%, gpu = {:.2}%, spu = {:.2}%, gpu_crtc = {:.2}%, spu_dac = {:.2}%, intc = {:.2}%", 
                time_elapsed, 
                overall_percentage,
                r3000_percentage, dmac_percentage, gpu_percentage, spu_percentage, gpu_crtc_percentage, spu_dac_percentage, intc_percentage
            );

            BENCHMARK_LAST_REPORTED = None;
        }
    }
}

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

pub extern "C" fn debug_opengl_trace(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, user_param: *const std::ffi::c_void) {
    unsafe {
        if type_ == GL_DEBUG_TYPE_ERROR_ARB {
            let message = CStr::from_ptr(message);
            debug!("OpenGL error: type: {}, severity = {}, message = {}", type_, severity, message.to_str().unwrap());
        }
    }
}
