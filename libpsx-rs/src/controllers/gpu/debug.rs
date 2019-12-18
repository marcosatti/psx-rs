use std::sync::atomic::{AtomicBool, Ordering};
use log::trace;

pub static ENABLE_GP0_COMMAND_TRACING: AtomicBool = AtomicBool::new(false);

pub fn trace_gp0_command(description: &str, data: &[u32]) {
    if !ENABLE_GP0_COMMAND_TRACING.load(Ordering::Acquire) {
        return;
    }

    let data_str = {
        let data_strs: Vec<String> = data.iter().map(|d| format!("0x{:08X}", d)).collect();
        data_strs.join(", ")
    };

    trace!("GP0 Comamnd: {}: data = [{}]", description, &data_str);
}
