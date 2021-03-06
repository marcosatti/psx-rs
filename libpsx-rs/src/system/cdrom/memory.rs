use crate::{
    system::{
        bus::types::*,
        cdrom::constants::*,
        types::State,
    },
    utilities::bool_to_flag,
};

// Emulation note:
// BIOS accesses ports at 0x1F80_1801 / 2 / 3 immediately after writing to the status register (no wait cycles or
// acknowledgment). Not an expert... but this suggests that the register is not latched, and the FIFO status bits are
// directly wired to the FIFO's themselves. If anyone can explain this more then let me know.

pub(crate) fn status_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    state.cdrom.status.write_bitfield(STATUS_PRMEMPT, bool_to_flag(state.cdrom.parameter.is_empty()) as u8);
    state.cdrom.status.write_bitfield(STATUS_PRMWRDY, bool_to_flag(!state.cdrom.parameter.is_full()) as u8);
    state.cdrom.status.write_bitfield(STATUS_RSLRRDY, bool_to_flag(!state.cdrom.response.is_empty()) as u8);
    state.cdrom.status.write_bitfield(STATUS_DRQSTS, bool_to_flag(!state.cdrom.data.is_empty()) as u8);
    Ok(state.cdrom.status.read_u8())
}

pub(crate) fn status_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.cdrom.status.write_u8(value))
}

pub(crate) fn cdrom1801_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0..=3 => state.cdrom.response.read_one().map_err(|_| ReadErrorKind::Empty),
        _ => unreachable!(),
    }
}

pub(crate) fn cdrom1801_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0 => state.cdrom.command.write_u8(value).map_err(|_| WriteErrorKind::NotReady),
        1..=2 => unimplemented!(),
        3 => Ok(state.cdrom.audio_right_to_right.write_u8(value)),
        _ => unreachable!(),
    }
}

pub(crate) fn cdrom1802_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0..=3 => state.cdrom.data.read_one().map_err(|_| ReadErrorKind::Empty),
        _ => unreachable!(),
    }
}

pub(crate) fn cdrom1802_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0 => state.cdrom.parameter.write_one(value).map_err(|_| WriteErrorKind::Full),
        1 => Ok(state.cdrom.interrupt_enable.write_u8(value)),
        2 => Ok(state.cdrom.audio_left_to_left.write_u8(value)),
        3 => Ok(state.cdrom.audio_right_to_left.write_u8(value)),
        _ => unreachable!(),
    }
}

pub(crate) fn cdrom1803_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0 => Ok(state.cdrom.interrupt_enable.read_u8()),
        1 => state.cdrom.interrupt_flag.read_u8().map_err(|_| ReadErrorKind::NotReady),
        2 => Ok(state.cdrom.interrupt_enable.read_u8()),
        3 => state.cdrom.interrupt_flag.read_u8().map_err(|_| ReadErrorKind::NotReady),
        _ => unreachable!(),
    }
}

pub(crate) fn cdrom1803_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0 => state.cdrom.request.write_u8(value).map_err(|_| WriteErrorKind::NotReady),
        1 => {
            state.cdrom.interrupt_flag.write_u8(value).map_err(|_| WriteErrorKind::NotReady)?;

            if INT_FLAG_CLRPRM.extract_from(value) > 0 {
                state.cdrom.parameter.clear();
                // log::debug!("Cleared parameter FIFO");
            }

            Ok(())
        },
        2 => Ok(state.cdrom.audio_left_to_right.write_u8(value)),
        3 => state.cdrom.audio_apply.write_u8(value).map_err(|_| WriteErrorKind::NotReady),
        _ => unreachable!(),
    }
}
