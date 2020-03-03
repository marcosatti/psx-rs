use crate::backends::cdrom::CdromBackend;
use crate::resources::Resources;
use crate::controllers::cdrom::interrupt::*;
use crate::resources::cdrom::*;

pub fn handle_read(resources: &mut Resources, cdrom_backend: &CdromBackend<'_>) -> bool {
    let reading = resources.cdrom.reading;
    if !reading {
        return false;
    }

    let request = &mut resources.cdrom.request;
    let load_data = request.register.read_bitfield(REQUEST_BFRD) > 0;
    if !load_data {
        return false;
    }

    let data = &mut resources.cdrom.data;

    // Let the BIOS read a bit of data before filling the FIFO up again.
    if data.write_available() < 12 {
        return false;
    }

    let response = &mut resources.cdrom.response;

    response.write_one(0b0010_0010).unwrap(); // Motor on | Reading

    let read_buffer = &mut resources.cdrom.read_buffer;

    if read_buffer.is_empty() {
        let data_block = match cdrom_backend {
            CdromBackend::None => panic!(),
            CdromBackend::Libmirage(ref params) => libmirage::read_sector(params, resources.cdrom.lba_address),
        };

        resources.cdrom.lba_address += 1;
        read_buffer.extend(&data_block);
    }

    loop {
        if data.is_full() {
            break;
        }

        match read_buffer.pop_front() {
            Some(v) => data.write_one(v).unwrap(),
            None => break,
        }
    }

    raise_irq(resources, 1);
    true
}
