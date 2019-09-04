pub mod r3000;
pub mod gpu;
pub mod intc;
pub mod dmac;
pub mod spu;
pub mod cdrom;

pub const BIOS_SIZE: usize = 0x8_0000; // 512 KiB
pub const MAIN_MEMORY_SIZE: usize = 0x20_0000; // 2 MiB
