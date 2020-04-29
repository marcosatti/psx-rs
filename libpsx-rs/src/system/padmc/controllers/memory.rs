use crate::system::types::State;
use crate::system::memory::types::*;

pub fn padmc1040_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    Ok(state.padmc.rx_fifo.read_one().unwrap_or_else(|_| { 
        log::warn!("PADMC RX FIFO empty - returning 0xFF");
        0xFF 
    }))
}

pub fn padmc1040_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    state.padmc.tx_fifo.write_one(value).map_err(|_| WriteErrorKind::Full)
}

pub fn padmc1040_read_u32(_state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    unimplemented!();
    // warn!("PADMC RX FIFO u32 preview reads not properly implemented");
    // rx_fifo.read_one().map(|v| v as u32).map_err(|_| ReadError::Empty)
}

pub fn padmc1040_write_u32(_state: &State, offset: u32, _value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    unimplemented!();
    // let value_u8 = value as u8;
    // tx_fifo.write_one(value_u8).map_err(|_| WriteError::Full)
}

pub fn stat_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.padmc.stat.read_u32())
}

pub fn stat_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.padmc.stat.write_u32(value))
}

pub fn mode_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.padmc.mode.read_u16())
}

pub fn mode_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.padmc.mode.write_u16(value))
}

pub fn ctrl_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.padmc.ctrl.read_u16())
}

pub fn ctrl_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.padmc.ctrl.write_u16(value))
}

pub fn baud_reload_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.padmc.baud_reload.read_u16())
}

pub fn baud_reload_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.padmc.baud_reload.write_u16(value))
}
