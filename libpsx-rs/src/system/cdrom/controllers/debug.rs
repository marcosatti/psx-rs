use crate::system::types::State;
use log::trace;

pub fn trace_cdrom(state: &State) {
    let parameter_empty = state.cdrom.parameter.is_empty();
    let parameter_full = state.cdrom.parameter.is_full();
    let response_empty = state.cdrom.response.is_empty();
    let response_full = state.cdrom.response.is_full();
    let data_empty = state.cdrom.data.is_empty();
    let data_full = state.cdrom.data.is_full();

    trace!("CDROM Parameter FIFO: empty = {}, full = {}", parameter_empty, parameter_full);
    trace!("CDROM Response FIFO: empty = {}, full = {}", response_empty, response_full);
    trace!("CDROM Data FIFO: empty = {}, full = {}", data_empty, data_full);
}
