pub mod register;
pub mod channel;
pub mod debug;

use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::bitfield::Bitfield;
use crate::resources::Resources;
use crate::resources::dmac::register::*;
use crate::resources::dmac::channel::*;

pub const DMA_CHANNEL_NAMES: [&str; 7] = ["MDECin", "MDECout", "GPU", "CDROM", "SPU", "PIO", "OTC"];
pub const DPCR_CHANNEL_ENABLE_BITFIELDS: [Bitfield; 7] = [DPCR_MDECIN_ENABLE, DPCR_MDECOUT_ENABLE, DPCR_GPU_ENABLE, DPCR_CDROM_ENABLE, DPCR_SPU_ENABLE, DPCR_PIO_ENABLE, DPCR_OTC_ENABLE];
pub const DICR_IRQ_ENABLE_BITFIELDS: [Bitfield; 7] = [DICR_MDECIN_IRQ_ENABLE, DICR_MDECOUT_IRQ_ENABLE, DICR_GPU_IRQ_ENABLE, DICR_CDROM_IRQ_ENABLE, DICR_SPU_IRQ_ENABLE, DICR_PIO_IRQ_ENABLE, DICR_OTC_IRQ_ENABLE];
pub const DICR_IRQ_FLAG_BITFIELDS: [Bitfield; 7] = [DICR_MDECIN_IRQ_FLAG, DICR_MDECOUT_IRQ_FLAG, DICR_GPU_IRQ_FLAG, DICR_CDROM_IRQ_FLAG, DICR_SPU_IRQ_FLAG, DICR_PIO_IRQ_FLAG, DICR_OTC_IRQ_FLAG];

pub const DPCR_MDECIN_ENABLE: Bitfield = Bitfield::new(3, 1); 
pub const DPCR_MDECOUT_ENABLE: Bitfield = Bitfield::new(7, 1); 
pub const DPCR_GPU_ENABLE: Bitfield = Bitfield::new(11, 1); 
pub const DPCR_CDROM_ENABLE: Bitfield = Bitfield::new(15, 1); 
pub const DPCR_SPU_ENABLE: Bitfield = Bitfield::new(19, 1); 
pub const DPCR_PIO_ENABLE: Bitfield = Bitfield::new(23, 1); 
pub const DPCR_OTC_ENABLE: Bitfield = Bitfield::new(27, 1); 
    
pub const DICR_IRQ_FORCE: Bitfield = Bitfield::new(15, 1);
pub const DICR_MDECIN_IRQ_ENABLE: Bitfield = Bitfield::new(16, 1);
pub const DICR_MDECOUT_IRQ_ENABLE: Bitfield = Bitfield::new(17, 1);
pub const DICR_GPU_IRQ_ENABLE: Bitfield = Bitfield::new(18, 1);
pub const DICR_CDROM_IRQ_ENABLE: Bitfield = Bitfield::new(19, 1);
pub const DICR_SPU_IRQ_ENABLE: Bitfield = Bitfield::new(20, 1);
pub const DICR_PIO_IRQ_ENABLE: Bitfield = Bitfield::new(21, 1);
pub const DICR_OTC_IRQ_ENABLE: Bitfield = Bitfield::new(22, 1);
pub const DICR_IRQ_MASTER_ENABLE: Bitfield = Bitfield::new(23, 1);
pub const DICR_MDECIN_IRQ_FLAG: Bitfield = Bitfield::new(24, 1);
pub const DICR_MDECOUT_IRQ_FLAG: Bitfield = Bitfield::new(25, 1);
pub const DICR_GPU_IRQ_FLAG: Bitfield = Bitfield::new(26, 1);
pub const DICR_CDROM_IRQ_FLAG: Bitfield = Bitfield::new(27, 1);
pub const DICR_SPU_IRQ_FLAG: Bitfield = Bitfield::new(28, 1);
pub const DICR_PIO_IRQ_FLAG: Bitfield = Bitfield::new(29, 1);
pub const DICR_OTC_IRQ_FLAG: Bitfield = Bitfield::new(30, 1);
pub const DICR_IRQ_MASTER_FLAG: Bitfield = Bitfield::new(31, 1);

pub const CHCR_TRANSFER_DIRECTION: Bitfield = Bitfield::new(0, 1);
pub const CHCR_MADR_STEP_DIRECTION: Bitfield = Bitfield::new(1, 1);
pub const CHCR_SYNCMODE: Bitfield = Bitfield::new(9, 2);
pub const _CHCR_CHOPPING_DMA_SIZE: Bitfield = Bitfield::new(16, 3); 
pub const _CHCR_CHOPPING_CPU_SIZE: Bitfield = Bitfield::new(20, 3); 
pub const CHCR_CHOPPING: Bitfield = Bitfield::new(8, 1);
pub const CHCR_STARTBUSY: Bitfield = Bitfield::new(24, 1);
pub const CHCR_STARTTRIGGER: Bitfield = Bitfield::new(28, 1);
pub const CHCR_BIT30: Bitfield = Bitfield::new(30, 1);

pub const BCR_BLOCKSIZE: Bitfield = Bitfield::new(0, 16);
pub const BCR_BLOCKAMOUNT: Bitfield = Bitfield::new(16, 16);

pub struct Dmac {
    pub dpcr: B32Register,
    pub dicr: Dicr,

    pub mdecin_madr: B32Register,
    pub mdecin_bcr: B32Register,
    pub mdecin_chcr: Chcr,
    pub mdecin_transfer_state: TransferState,

    pub mdecout_madr: B32Register,
    pub mdecout_bcr: B32Register,
    pub mdecout_chcr: Chcr,
    pub mdecout_transfer_state: TransferState,

    pub gpu_madr: B32Register,
    pub gpu_bcr: B32Register,
    pub gpu_chcr: Chcr,
    pub gpu_transfer_state: TransferState,

    pub cdrom_madr: B32Register,
    pub cdrom_bcr: B32Register,
    pub cdrom_chcr: Chcr,
    pub cdrom_transfer_state: TransferState,

    pub spu_madr: B32Register,
    pub spu_bcr: B32Register,
    pub spu_chcr: Chcr,
    pub spu_transfer_state: TransferState,

    pub pio_madr: B32Register,
    pub pio_bcr: B32Register,
    pub pio_chcr: Chcr,
    pub pio_transfer_state: TransferState,
    
    pub otc_madr: B32Register,
    pub otc_bcr: B32Register,
    pub otc_chcr: OtcChcr,
    pub otc_transfer_state: TransferState,
}

impl Dmac {
    pub fn new() -> Dmac {
        Dmac {
            dpcr: B32Register::new(),
            dicr: Dicr::new(),
            mdecin_madr: B32Register::new(),
            mdecin_bcr: B32Register::new(),
            mdecin_chcr: Chcr::new(),
            mdecin_transfer_state: TransferState::reset(),
            mdecout_madr: B32Register::new(),
            mdecout_bcr: B32Register::new(),
            mdecout_chcr: Chcr::new(),
            mdecout_transfer_state: TransferState::reset(),
            gpu_madr: B32Register::new(),
            gpu_bcr: B32Register::new(),
            gpu_chcr: Chcr::new(),
            gpu_transfer_state: TransferState::reset(),
            cdrom_madr: B32Register::new(),
            cdrom_bcr: B32Register::new(),
            cdrom_chcr: Chcr::new(),
            cdrom_transfer_state: TransferState::reset(),
            spu_madr: B32Register::new(),
            spu_bcr: B32Register::new(),
            spu_chcr: Chcr::new(),
            spu_transfer_state: TransferState::reset(),
            pio_madr: B32Register::new(),
            pio_bcr: B32Register::new(),
            pio_chcr: Chcr::new(),
            pio_transfer_state: TransferState::reset(),
            otc_madr: B32Register::new(),
            otc_bcr: B32Register::new(),
            otc_chcr: OtcChcr::new(),         
            otc_transfer_state: TransferState::reset(),   
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.r3000.memory_mapper.map(0x1F80_1080, 4, &mut resources.dmac.mdecin_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1084, 4, &mut resources.dmac.mdecin_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1088, 4, &mut resources.dmac.mdecin_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1090, 4, &mut resources.dmac.mdecout_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1094, 4, &mut resources.dmac.mdecout_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1098, 4, &mut resources.dmac.mdecout_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10A0, 4, &mut resources.dmac.gpu_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10A4, 4, &mut resources.dmac.gpu_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10A8, 4, &mut resources.dmac.gpu_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10B0, 4, &mut resources.dmac.cdrom_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10B4, 4, &mut resources.dmac.cdrom_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10B8, 4, &mut resources.dmac.cdrom_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10C0, 4, &mut resources.dmac.spu_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10C4, 4, &mut resources.dmac.spu_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10C8, 4, &mut resources.dmac.spu_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10D0, 4, &mut resources.dmac.pio_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10D4, 4, &mut resources.dmac.pio_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10D8, 4, &mut resources.dmac.pio_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10E0, 4, &mut resources.dmac.otc_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10E4, 4, &mut resources.dmac.otc_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10E8, 4, &mut resources.dmac.otc_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10F0, 4, &mut resources.dmac.dpcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10F4, 4, &mut resources.dmac.dicr as *mut dyn B8MemoryMap);
}
