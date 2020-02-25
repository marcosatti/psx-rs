use crate::backends::cdrom::CdromBackend;
use crate::resources::Resources;
use crate::constants::cdrom::*;
use crate::controllers::cdrom::libmirage;

pub fn command_01(resources: &mut Resources, _cdrom_backend: &CdromBackend<'_>, command_iteration: usize) -> bool {
    assert_eq!(command_iteration, 0);

    let response = &mut resources.cdrom.response;
    let int_flag = &mut resources.cdrom.int_flag;

    // No CD inserted.
    // match command_iteration {
    //     0 => {
    //         response.write_one(0x1).unwrap();
    //         int_flag.set_interrupt(3);
    //         false
    //     },
    //     1 => {
    //         response.write_one(0x80).unwrap();
    //         int_flag.set_interrupt(5);
    //         true
    //     },
    //     _ => panic!(),
    // }

    response.write_one(0b0000_0010).unwrap(); // Motor on
    int_flag.set_interrupt(3);
    true
}

pub fn command_19(resources: &mut Resources, _cdrom_backend: &CdromBackend<'_>, command_iteration: usize) -> bool {
    assert_eq!(command_iteration, 0);

    let parameter = &resources.cdrom.parameter;
    let response = &resources.cdrom.response;

    let sub_function = parameter.read_one().unwrap();

    match sub_function {
        0x20 => {
            for &i in VERSION.iter() {
                response.write_one(i).unwrap();
            }
        },
        _ => unimplemented!(),
    }

    let int_flag = &mut resources.cdrom.int_flag;
    int_flag.set_interrupt(3);

    true
}

pub fn command_1a(resources: &mut Resources, cdrom_backend: &CdromBackend<'_>, command_iteration: usize) -> bool {
    match command_iteration {
        0 => {
            let response = &resources.cdrom.response;
            let int_flag = &mut resources.cdrom.int_flag;

            response.write_one(0b0000_0010).unwrap(); // Reading data | Motor on
            int_flag.set_interrupt(3);

            false
        },
        1 => {
            let response = &resources.cdrom.response;
            let int_flag = &mut resources.cdrom.int_flag;

            // Determine disc mode type.
            let mode = match cdrom_backend {
                CdromBackend::None => panic!("Unsupported"),
                CdromBackend::Libmirage(ref params) => libmirage::disc_mode(params),
            };

            match mode {
                2 => {
                    response.write_one(0x02).unwrap(); 
                    response.write_one(0x00).unwrap(); 
                    response.write_one(0x20).unwrap(); 
                    response.write_one(0x00).unwrap(); 
                    response.write_one(0x53).unwrap(); 
                    response.write_one(0x43).unwrap(); 
                    response.write_one(0x45).unwrap(); 
                    response.write_one(0x41).unwrap(); // SCEx: ASCII A = 0x41, E = 0x45, I = 0x49 
                },
                _ => unimplemented!("Disc mode {} not handled", mode),
            }
            
            int_flag.set_interrupt(2);

            true
        },
        _ => panic!(),
    }
}
