use log::trace;
use crate::resources::Resources;

pub fn trace_cdrom(resources: &Resources) {
    let parameter_empty = resources.cdrom.parameter.is_empty();
    let parameter_full = resources.cdrom.parameter.is_full();
    let response_empty = resources.cdrom.response.is_empty();
    let response_full = resources.cdrom.response.is_full();
    let data_empty = resources.cdrom.data.is_empty();
    let data_full = resources.cdrom.data.is_full();

    trace!("CDROM Parameter FIFO: empty = {}, full = {}", parameter_empty, parameter_full);
    trace!("CDROM Response FIFO: empty = {}, full = {}", response_empty, response_full);
    trace!("CDROM Data FIFO: empty = {}, full = {}", data_empty, data_full);
}
