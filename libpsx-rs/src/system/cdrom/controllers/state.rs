use crate::{
    system::cdrom::types::ControllerState,
    types::bitfield::Bitfield,
    utilities::bool_to_flag,
};

pub fn stat_value(state: &ControllerState) -> u8 {
    const _ERROR: Bitfield = Bitfield::new(0, 0);
    const MOTOR_ON: Bitfield = Bitfield::new(1, 1);
    const _SEEK_ERROR: Bitfield = Bitfield::new(2, 1);
    const _ID_ERROR: Bitfield = Bitfield::new(3, 1);
    const _SHELL_OPEN: Bitfield = Bitfield::new(4, 1);
    const READ: Bitfield = Bitfield::new(5, 1);
    const SEEK: Bitfield = Bitfield::new(6, 1);
    const _PLAY: Bitfield = Bitfield::new(7, 1);

    const READ_SEEK_PLAY: Bitfield = Bitfield::new(5, 3);

    let reading = state.reading;
    let seeking = state.seeking;

    let mut value = 0;

    value = MOTOR_ON.insert_into(value, 1); // Motor always on.

    // Only one of the three states below are allowed to be on, or none at all.

    if reading {
        assert!(READ_SEEK_PLAY.extract_from(value) == 0);
        value = READ.insert_into(value, bool_to_flag(reading) as u8);
    }

    if seeking {
        assert!(READ_SEEK_PLAY.extract_from(value) == 0);
        value = SEEK.insert_into(value, bool_to_flag(seeking) as u8);
    }

    value
}
