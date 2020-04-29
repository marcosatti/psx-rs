use crate::types::memory::*;
use crate::system::memory::constants::*;

pub struct State {
    pub main_memory: B8Memory,
    pub bios: B8Memory,
    pub expansion_1_base_address: B32Register,
    pub expansion_2_base_address: B32Register,
    pub expansion_1_delay: B32Register,
    pub expansion_3_delay: B32Register,
    pub bios_rom_control: B32Register,
    pub spu_delay: B32Register,
    pub cdrom_delay: B32Register,
    pub expansion_2_delay: B32Register,
    pub common_delay_control: B32Register,
    pub ram_size_control: B32Register,
    pub cache_control: B8Memory,
    pub post_display: B8Register,
    pub pio: B8Memory,
}

impl State {
    pub fn new() -> State {
        State {
            main_memory: B8Memory::new(MAIN_MEMORY_SIZE),
            bios: B8Memory::new(BIOS_SIZE),
            expansion_1_base_address: B32Register::new(),
            expansion_2_base_address: B32Register::new(),
            expansion_1_delay: B32Register::new(),
            expansion_3_delay: B32Register::new(),
            bios_rom_control: B32Register::new(),
            spu_delay: B32Register::new(),
            cdrom_delay: B32Register::new(),
            expansion_2_delay: B32Register::new(),
            common_delay_control: B32Register::new(),
            ram_size_control: B32Register::new(),
            cache_control: B8Memory::new(0x2_0000),
            post_display: B8Register::new(),
            pio: B8Memory::new_initialized(0x100, 0xFF),
        }
    }
}
