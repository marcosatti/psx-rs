use crate::{
    system::{
        bus::types::*,
        padmc::constants::*,
        types::State,
    },
    utilities::bool_to_flag,
};

pub(crate) fn padmc1040_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    Ok(state.padmc.rx_fifo.read_one().unwrap_or_else(|_| {
        // log::warn!("Empty RX FIFO; proper behaviour not implemented (see SIO docs); returning 0x0");
        0x0
    }))
}

pub(crate) fn padmc1040_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    state.padmc.tx_fifo.write_one(value).map_err(|_| WriteErrorKind::Full)
}

pub(crate) fn padmc1040_read_u32(_state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    unimplemented!();
}

pub(crate) fn padmc1040_write_u32(_state: &State, offset: u32, _value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    unimplemented!();
}

pub(crate) fn stat_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    let mut value = state.padmc.stat.read_u16(0);
    // TODO: implement properly.
    value = STAT_TXRDY_1.insert_into(value, 1);
    value = STAT_RXFIFO_READY.insert_into(value, !bool_to_flag(state.padmc.rx_fifo.is_empty()) as u16);
    value = STAT_TXRDY_2.insert_into(value, 1);
    Ok(value)
}

pub(crate) fn stat_write_u16(_state: &State, _offset: u32, _value: u16) -> WriteResult {
    unimplemented!();
}

pub(crate) fn stat_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    let mut value = state.padmc.stat.read_u32();
    // TODO: implement properly.
    value = STAT_TXRDY_1.insert_into(value, 1);
    value = STAT_RXFIFO_READY.insert_into(value, !bool_to_flag(state.padmc.rx_fifo.is_empty()));
    value = STAT_TXRDY_2.insert_into(value, 1);
    Ok(value)
}

pub(crate) fn stat_write_u32(_state: &State, _offset: u32, _value: u32) -> WriteResult {
    unimplemented!();
}

pub(crate) fn mode_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.padmc.mode.read_u16())
}

pub(crate) fn mode_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.padmc.mode.write_u16(value))
}

pub(crate) fn ctrl_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    state.padmc.ctrl.read_u16().map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn ctrl_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    state.padmc.ctrl.write_u16(value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn baud_reload_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.padmc.baud_reload.read_u16())
}

pub(crate) fn baud_reload_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.padmc.baud_reload.write_u16(value))
}
