use crate::types::bitfield::Bitfield;

pub const CLOCK_SPEED: f64 = 33.8688 * 1e6; // 33.8688 MHz

pub const VBLANK: Bitfield = Bitfield::new(0, 1);
pub const GPU: Bitfield = Bitfield::new(1, 1);
pub const CDROM: Bitfield = Bitfield::new(2, 1);
pub const DMA: Bitfield = Bitfield::new(3, 1);
pub const TMR0: Bitfield = Bitfield::new(4, 1);
pub const TMR1: Bitfield = Bitfield::new(5, 1);
pub const TMR2: Bitfield = Bitfield::new(6, 1);
pub const PADMC: Bitfield = Bitfield::new(7, 1);
pub const SIO: Bitfield = Bitfield::new(8, 1);
pub const SPU: Bitfield = Bitfield::new(9, 1);
pub const PIO: Bitfield = Bitfield::new(10, 1);
pub const IRQ_NAMES: [&str; 11] = [
    "VBLANK", "GPU", "CDROM", "DMA", "TMR0", "TMR1", "TMR2", "PADMC", "SIO", "SPU", "PIO",
];
pub const IRQ_BITFIELDS: [Bitfield; 11] = [
    VBLANK, GPU, CDROM, DMA, TMR0, TMR1, TMR2, PADMC, SIO, SPU, PIO,
];
