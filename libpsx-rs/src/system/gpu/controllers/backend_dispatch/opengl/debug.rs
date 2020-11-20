pub(crate) const DEBUG_DRAW_OUTLINE: bool = false;
pub(crate) const TRACE_CALLS: bool = false;

pub(crate) fn trace_call(description: &str) {
    if TRACE_CALLS {
        log::trace!("GPU: OpenGL call: {}", description);
    }
}
