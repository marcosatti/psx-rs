const ENABLE_GP0_COMMAND_TRACING: bool = false;

pub(crate) fn trace_gp0_command(description: &str, data: &[u32]) {
    if !ENABLE_GP0_COMMAND_TRACING {
        return;
    }

    let data_str = data.iter().map(|d| format!("0x{:08X}", d)).collect::<Vec<String>>().join(", ");

    if false {
        log::trace!("GP0 Comamnd: {}: data = [{}]", description, &data_str);
    } else {
        log::trace!("GP0 Comamnd: {}", description);
    }
}
