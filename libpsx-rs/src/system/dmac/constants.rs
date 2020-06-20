use crate::types::bitfield::Bitfield;

pub(crate) const CLOCK_SPEED: f64 = 33.8688 * 1e6; // 33.8688 MHz
pub(crate) const CLOCK_SPEED_PERIOD: f64 = 1.0 / CLOCK_SPEED;
pub(crate) const DATA_SIZE: u32 = 4;

pub(crate) const _DMA_CHANNEL_NAMES: [&str; 7] = ["MDECin", "MDECout", "GPU", "CDROM", "SPU", "PIO", "OTC"];
pub(crate) const DPCR_CHANNEL_ENABLE_BITFIELDS: [Bitfield; 7] =
    [DPCR_MDECIN_ENABLE, DPCR_MDECOUT_ENABLE, DPCR_GPU_ENABLE, DPCR_CDROM_ENABLE, DPCR_SPU_ENABLE, DPCR_PIO_ENABLE, DPCR_OTC_ENABLE];
pub(crate) const DICR_IRQ_ENABLE_BITFIELDS: [Bitfield; 7] =
    [DICR_MDECIN_IRQ_ENABLE, DICR_MDECOUT_IRQ_ENABLE, DICR_GPU_IRQ_ENABLE, DICR_CDROM_IRQ_ENABLE, DICR_SPU_IRQ_ENABLE, DICR_PIO_IRQ_ENABLE, DICR_OTC_IRQ_ENABLE];
pub(crate) const DICR_IRQ_FLAG_BITFIELDS: [Bitfield; 7] =
    [DICR_MDECIN_IRQ_FLAG, DICR_MDECOUT_IRQ_FLAG, DICR_GPU_IRQ_FLAG, DICR_CDROM_IRQ_FLAG, DICR_SPU_IRQ_FLAG, DICR_PIO_IRQ_FLAG, DICR_OTC_IRQ_FLAG];

pub(crate) const DPCR_MDECIN_ENABLE: Bitfield = Bitfield::new(3, 1);
pub(crate) const DPCR_MDECOUT_ENABLE: Bitfield = Bitfield::new(7, 1);
pub(crate) const DPCR_GPU_ENABLE: Bitfield = Bitfield::new(11, 1);
pub(crate) const DPCR_CDROM_ENABLE: Bitfield = Bitfield::new(15, 1);
pub(crate) const DPCR_SPU_ENABLE: Bitfield = Bitfield::new(19, 1);
pub(crate) const DPCR_PIO_ENABLE: Bitfield = Bitfield::new(23, 1);
pub(crate) const DPCR_OTC_ENABLE: Bitfield = Bitfield::new(27, 1);

pub(crate) const DICR_IRQ_FORCE: Bitfield = Bitfield::new(15, 1);
pub(crate) const DICR_MDECIN_IRQ_ENABLE: Bitfield = Bitfield::new(16, 1);
pub(crate) const DICR_MDECOUT_IRQ_ENABLE: Bitfield = Bitfield::new(17, 1);
pub(crate) const DICR_GPU_IRQ_ENABLE: Bitfield = Bitfield::new(18, 1);
pub(crate) const DICR_CDROM_IRQ_ENABLE: Bitfield = Bitfield::new(19, 1);
pub(crate) const DICR_SPU_IRQ_ENABLE: Bitfield = Bitfield::new(20, 1);
pub(crate) const DICR_PIO_IRQ_ENABLE: Bitfield = Bitfield::new(21, 1);
pub(crate) const DICR_OTC_IRQ_ENABLE: Bitfield = Bitfield::new(22, 1);
pub(crate) const DICR_IRQ_MASTER_ENABLE: Bitfield = Bitfield::new(23, 1);
pub(crate) const DICR_MDECIN_IRQ_FLAG: Bitfield = Bitfield::new(24, 1);
pub(crate) const DICR_MDECOUT_IRQ_FLAG: Bitfield = Bitfield::new(25, 1);
pub(crate) const DICR_GPU_IRQ_FLAG: Bitfield = Bitfield::new(26, 1);
pub(crate) const DICR_CDROM_IRQ_FLAG: Bitfield = Bitfield::new(27, 1);
pub(crate) const DICR_SPU_IRQ_FLAG: Bitfield = Bitfield::new(28, 1);
pub(crate) const DICR_PIO_IRQ_FLAG: Bitfield = Bitfield::new(29, 1);
pub(crate) const DICR_OTC_IRQ_FLAG: Bitfield = Bitfield::new(30, 1);
pub(crate) const DICR_IRQ_MASTER_FLAG: Bitfield = Bitfield::new(31, 1);

pub(crate) const CHCR_TRANSFER_DIRECTION: Bitfield = Bitfield::new(0, 1);
pub(crate) const CHCR_MADR_STEP_DIRECTION: Bitfield = Bitfield::new(1, 1);
pub(crate) const CHCR_SYNCMODE: Bitfield = Bitfield::new(9, 2);
pub(crate) const _CHCR_CHOPPING_DMA_SIZE: Bitfield = Bitfield::new(16, 3);
pub(crate) const _CHCR_CHOPPING_CPU_SIZE: Bitfield = Bitfield::new(20, 3);
pub(crate) const CHCR_CHOPPING: Bitfield = Bitfield::new(8, 1);
pub(crate) const CHCR_STARTBUSY: Bitfield = Bitfield::new(24, 1);
pub(crate) const CHCR_STARTTRIGGER: Bitfield = Bitfield::new(28, 1);
pub(crate) const CHCR_BIT30: Bitfield = Bitfield::new(30, 1);

pub(crate) const BCR_BLOCKSIZE: Bitfield = Bitfield::new(0, 16);
pub(crate) const BCR_BLOCKAMOUNT: Bitfield = Bitfield::new(16, 16);
