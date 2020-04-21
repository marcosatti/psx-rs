use crate::types::memory::b8_memory::B8Memory;

#[derive(Clone, Copy, Debug)]
pub enum ReadErrorKind {
    Empty,
}

pub type ReadResult<T> = Result<T, ReadErrorKind>;

#[derive(Clone, Copy, Debug)]
pub enum WriteErrorKind {
    Full,
}

pub type WriteResult = Result<(), WriteErrorKind>;

pub struct State {
    pub expansion_1_base_address: B8Memory,
    pub expansion_2_base_address: B8Memory,
    pub expansion_1_delay: B8Memory,
    pub expansion_3_delay: B8Memory,
    pub bios_rom_control: B8Memory,
    pub spu_delay: B8Memory,
    pub cdrom_delay: B8Memory,
    pub expansion_2_delay: B8Memory,
    pub common_delay_control: B8Memory,
    pub ram_size_control: B8Memory,
    pub cache_control: B8Memory,
}

impl State {
    pub fn new() -> State {
        State {
            expansion_1_base_address: B8Memory::new(4),
            expansion_2_base_address: B8Memory::new(4),
            expansion_1_delay: B8Memory::new(4),
            expansion_3_delay: B8Memory::new(4),
            bios_rom_control: B8Memory::new(4),
            spu_delay: B8Memory::new(4),
            cdrom_delay: B8Memory::new(4),
            expansion_2_delay: B8Memory::new(4),
            common_delay_control: B8Memory::new(4),
            ram_size_control: B8Memory::new(4),
            cache_control: B8Memory::new(0x2_0000),
        }
    }
}
