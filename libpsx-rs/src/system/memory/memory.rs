use crate::system::{
    bus::types::*,
    types::State,
};

pub(crate) fn main_memory_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    Ok(state.memory.main_memory.read_u8(offset))
}

pub(crate) fn main_memory_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    Ok(state.memory.main_memory.write_u8(offset, value))
}

pub(crate) fn main_memory_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.memory.main_memory.read_u16(offset))
}

pub(crate) fn main_memory_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.memory.main_memory.write_u16(offset, value))
}

pub(crate) fn main_memory_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    Ok(state.memory.main_memory.read_u32(offset))
}

pub(crate) fn main_memory_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    Ok(state.memory.main_memory.write_u32(offset, value))
}

pub(crate) fn pio_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    Ok(state.memory.pio.read_u8(offset))
}

pub(crate) fn pio_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    Ok(state.memory.pio.write_u8(offset, value))
}

pub(crate) fn pio_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.memory.pio.read_u16(offset))
}

pub(crate) fn pio_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.memory.pio.write_u16(offset, value))
}

pub(crate) fn pio_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    Ok(state.memory.pio.read_u32(offset))
}

pub(crate) fn pio_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    Ok(state.memory.pio.write_u32(offset, value))
}

pub(crate) fn expansion_1_base_address_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.memory.expansion_1_base_address.read_u32())
}

pub(crate) fn expansion_1_base_address_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.expansion_1_base_address.write_u32(value))
}

pub(crate) fn expansion_2_base_address_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.memory.expansion_2_base_address.read_u32())
}

pub(crate) fn expansion_2_base_address_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.expansion_2_base_address.write_u32(value))
}

pub(crate) fn expansion_1_delay_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.memory.expansion_1_delay.read_u32())
}

pub(crate) fn expansion_1_delay_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.expansion_1_delay.write_u32(value))
}

pub(crate) fn expansion_3_delay_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.memory.expansion_3_delay.read_u32())
}

pub(crate) fn expansion_3_delay_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.expansion_3_delay.write_u32(value))
}

pub(crate) fn bios_rom_control_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.memory.bios_rom_control.read_u32())
}

pub(crate) fn bios_rom_control_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.bios_rom_control.write_u32(value))
}

pub(crate) fn spu_delay_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.memory.spu_delay.read_u32())
}

pub(crate) fn spu_delay_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.spu_delay.write_u32(value))
}

pub(crate) fn cdrom_delay_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.memory.cdrom_delay.read_u32())
}

pub(crate) fn cdrom_delay_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.cdrom_delay.write_u32(value))
}

pub(crate) fn expansion_2_delay_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.memory.expansion_2_delay.read_u32())
}

pub(crate) fn expansion_2_delay_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.expansion_2_delay.write_u32(value))
}

pub(crate) fn common_delay_control_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.memory.common_delay_control.read_u32())
}

pub(crate) fn common_delay_control_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.common_delay_control.write_u32(value))
}

pub(crate) fn ram_size_control_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.memory.ram_size_control.read_u32())
}

pub(crate) fn ram_size_control_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.ram_size_control.write_u32(value))
}

pub(crate) fn post_display_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    Ok(state.memory.post_display.read_u8())
}

pub(crate) fn post_display_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.memory.post_display.write_u8(value))
}

pub(crate) fn bios_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    Ok(state.memory.bios.read_u8(offset))
}

pub(crate) fn bios_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    Ok(state.memory.bios.write_u8(offset, value))
}

pub(crate) fn bios_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.memory.bios.read_u16(offset))
}

pub(crate) fn bios_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.memory.bios.write_u16(offset, value))
}

pub(crate) fn bios_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    Ok(state.memory.bios.read_u32(offset))
}

pub(crate) fn bios_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    Ok(state.memory.bios.write_u32(offset, value))
}

pub(crate) fn cache_control_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    Ok(state.memory.cache_control.read_u8(offset))
}

pub(crate) fn cache_control_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    Ok(state.memory.cache_control.write_u8(offset, value))
}

pub(crate) fn cache_control_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.memory.cache_control.read_u16(offset))
}

pub(crate) fn cache_control_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.memory.cache_control.write_u16(offset, value))
}

pub(crate) fn cache_control_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    Ok(state.memory.cache_control.read_u32(offset))
}

pub(crate) fn cache_control_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    Ok(state.memory.cache_control.write_u32(offset, value))
}

pub(crate) fn scratchpad_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    Ok(state.memory.scratchpad.read_u8(offset))
}

pub(crate) fn scratchpad_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    Ok(state.memory.scratchpad.write_u8(offset, value))
}

pub(crate) fn scratchpad_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.memory.scratchpad.read_u16(offset))
}

pub(crate) fn scratchpad_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.memory.scratchpad.write_u16(offset, value))
}

pub(crate) fn scratchpad_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    Ok(state.memory.scratchpad.read_u32(offset))
}

pub(crate) fn scratchpad_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    Ok(state.memory.scratchpad.write_u32(offset, value))
}
