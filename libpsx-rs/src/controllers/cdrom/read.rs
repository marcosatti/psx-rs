use crate::backends::cdrom::CdromBackend;
use crate::resources::Resources;
use crate::controllers::cdrom::backend_dispatch;
use crate::controllers::cdrom::interrupt::*;
use crate::controllers::cdrom::state::*;
use crate::resources::cdrom::*;

pub fn handle_read(resources: &mut Resources, cdrom_backend: &CdromBackend<'_>) -> bool {
    // Buffer some data first (INT1 means ready to send data?).
    // Do we always want to send data if we have read a sector regardless of the current reading status? Seems like the BIOS expects it...
    if resources.cdrom.read_buffer.is_empty() {
        if resources.cdrom.pausing {
            // Stop reading and raise interrupt.
            resources.cdrom.pausing = false;
            resources.cdrom.reading = false;
            let stat_value = stat_value(resources);
            let response = &mut resources.cdrom.response;
            response.write_one(stat_value).unwrap();
            raise_irq(resources, 2);
            return true;
        }

        if !resources.cdrom.reading {
            return false;
        }
        
        // Make sure FIFO is empty.
        if !resources.cdrom.data.is_empty() {
            return true;
        }

        let data_block = backend_dispatch::read_sector(cdrom_backend, resources.cdrom.lba_address);
        assert_eq!(data_block.len(), 2048);

        resources.cdrom.lba_address += 1;
        resources.cdrom.read_buffer.extend(&data_block);

        // Raise the interrupt - we have read a sector ok and have some data ready.
        let stat_value = stat_value(resources);
        let response = &mut resources.cdrom.response;
        response.write_one(stat_value).unwrap();
        raise_irq(resources, 1);
    }

    // Check if the CPU is ready for data and send it.
    let request = &mut resources.cdrom.request;
    let load_data = request.register.read_bitfield(REQUEST_BFRD) > 0;
    if load_data {
        let read_buffer = &mut resources.cdrom.read_buffer;
        let data = &mut resources.cdrom.data;

        loop {
            if data.is_full() {
                break;
            }
    
            match read_buffer.pop_front() {
                Some(v) => data.write_one(v).unwrap(),
                None => break,
            }
        }
    }

    true
}
