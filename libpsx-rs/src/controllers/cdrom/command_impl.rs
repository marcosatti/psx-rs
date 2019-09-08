use crate::resources::Resources;
use crate::constants::cdrom::*;

pub fn command_01(resources: &mut Resources) {
    // TODO: implement.

    let response = &mut resources.cdrom.response;
    let int_flag = &mut resources.cdrom.int_flag;

    response.write_one(0x0).unwrap();
    int_flag.set_interrupt(3);

    response.write_one(0x08).unwrap();
    response.write_one(0x40).unwrap();
    response.write_one(0x00).unwrap();
    response.write_one(0x00).unwrap();
    response.write_one(0x00).unwrap();
    response.write_one(0x00).unwrap();
    response.write_one(0x00).unwrap();
    response.write_one(0x00).unwrap();
    int_flag.set_interrupt(5);
}

pub fn command_19(resources: &mut Resources) {
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
}
