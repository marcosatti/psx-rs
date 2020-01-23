use crate::resources::Resources;
use crate::constants::cdrom::*;

pub fn command_01(resources: &mut Resources, _command_iteration: usize) -> bool {
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

    response.write_one(0b0000_0010).unwrap();
    int_flag.set_interrupt(3);
    true
}

pub fn command_19(resources: &mut Resources, _command_iteration: usize) -> bool {
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

pub fn command_1a(_resources: &mut Resources, command_iteration: usize) -> bool {
    // SCEA = 0x53, 0x43, 0x45, 0x41
    // SCEE = 0x53, 0x43, 0x45, 0x45
    // SCEI = 0x53, 0x43, 0x45, 0x49

    // MODE1 disk: INT3(stat)     INT2(02h,00h, 00h,00h, 53h,43h,45h,4xh)
    // MODE2 disk: INT3(stat)     INT2(02h,00h, 20h,00h, 53h,43h,45h,4xh)
    
    match command_iteration {
        0 => {
            unimplemented!();
        },
        1 => {
            unimplemented!();
        },
        _ => panic!(),
    }
}
