use crate::{
    system::memory::constants::*,
    types::memory::*,
};

pub struct State {
    pub main_memory: B8Memory,
    pub bios: B8Memory,
    pub expansion_1_base_address: B32LevelRegister,
    pub expansion_2_base_address: B32LevelRegister,
    pub expansion_1_delay: B32LevelRegister,
    pub expansion_3_delay: B32LevelRegister,
    pub bios_rom_control: B32LevelRegister,
    pub spu_delay: B32LevelRegister,
    pub cdrom_delay: B32LevelRegister,
    pub expansion_2_delay: B32LevelRegister,
    pub common_delay_control: B32LevelRegister,
    pub ram_size_control: B32LevelRegister,
    pub cache_control: B8Memory,
    pub post_display: B8LevelRegister,
    pub pio: B8Memory,
}

impl State {
    pub fn new() -> State {
        State {
            main_memory: B8Memory::new(MAIN_MEMORY_SIZE),
            bios: B8Memory::new(BIOS_SIZE),
            expansion_1_base_address: B32LevelRegister::new(),
            expansion_2_base_address: B32LevelRegister::new(),
            expansion_1_delay: B32LevelRegister::new(),
            expansion_3_delay: B32LevelRegister::new(),
            bios_rom_control: B32LevelRegister::new(),
            spu_delay: B32LevelRegister::new(),
            cdrom_delay: B32LevelRegister::new(),
            expansion_2_delay: B32LevelRegister::new(),
            common_delay_control: B32LevelRegister::new(),
            ram_size_control: B32LevelRegister::new(),
            cache_control: B8Memory::new(0x2_0000),
            post_display: B8LevelRegister::new(),
            pio: B8Memory::new_initialized(0x100, 0xFF),
        }
    }
}
