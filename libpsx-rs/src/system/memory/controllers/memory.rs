use crate::system::types::State;

fn main_memory_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    Ok(state.memory.main_memory.read_u8(offset))
}

fn main_memory_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    state.memory.main_memory.write_u8(offset, value);
    Ok(())
}

fn main_memory_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.memory.main_memory.read_u16(offset))
}

fn main_memory_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    state.memory.main_memory.write_u16(offset, value);
    Ok(())
}

fn main_memory_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    Ok(state.memory.main_memory.read_u32(offset))
}

fn main_memory_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    state.memory.main_memory.write_u32(offset, value);
    Ok(())
}
