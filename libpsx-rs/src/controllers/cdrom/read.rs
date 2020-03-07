use crate::backends::cdrom::CdromBackend;
use crate::resources::Resources;
use crate::controllers::cdrom::libmirage;
use crate::controllers::cdrom::interrupt::*;
use crate::controllers::cdrom::state::*;
use crate::resources::cdrom::*;

static mut TEST: usize = 0;

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
            log::debug!("Waiting on FIFO");
            return true;
        }

        unsafe {
            if TEST > 0 {
                TEST -= 1;
                return true;
            }
        }

        let data_block = match cdrom_backend {
            CdromBackend::None => panic!(),
            CdromBackend::Libmirage(ref params) => libmirage::read_sector(params, resources.cdrom.lba_address),
        };

        resources.cdrom.lba_address += 1;
        resources.cdrom.read_buffer.extend(&data_block);

        // Raise the interrupt - we have read a sector ok and have some data ready.
        let stat_value = stat_value(resources);
        let response = &mut resources.cdrom.response;
        response.write_one(stat_value).unwrap();
        raise_irq(resources, 1);
        log::debug!("Read sector; raised data IRQ");
    }

    unsafe { TEST = 1000000; }

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
