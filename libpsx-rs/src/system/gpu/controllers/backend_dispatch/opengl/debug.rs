pub(crate) const TRACE_CALLS: bool = true;

pub(crate) fn trace_call(description: &str) {
    if TRACE_CALLS {
        log::trace!("GPU: OpenGL call: {}", description);
    }
}
