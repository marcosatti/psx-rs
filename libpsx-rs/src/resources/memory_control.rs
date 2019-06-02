use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::memory::b8_memory::B8Memory;
use crate::resources::Resources;

pub struct MemoryControl {
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
}

impl MemoryControl {
    pub fn new() -> MemoryControl {
        MemoryControl {
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
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1000, 4, &mut resources.memory_control.expansion_1_base_address as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1004, 4, &mut resources.memory_control.expansion_2_base_address as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1008, 4, &mut resources.memory_control.expansion_1_delay as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_100C, 4, &mut resources.memory_control.expansion_3_delay as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1010, 4, &mut resources.memory_control.bios_rom_control as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1014, 4, &mut resources.memory_control.spu_delay as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1018, 4, &mut resources.memory_control.cdrom_delay as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_101C, 4, &mut resources.memory_control.expansion_2_delay as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1020, 4, &mut resources.memory_control.common_delay_control as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1060, 4, &mut resources.memory_control.ram_size_control as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0xFFFE_0000, 0x2_0000, &mut resources.memory_control.cache_control as *mut dyn B8MemoryMap);
}