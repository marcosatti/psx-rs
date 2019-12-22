use std::sync::atomic::{Ordering, AtomicBool};
use crate::utilities::bool_to_flag;
use crate::types::b8_memory_mapper::*;
use crate::resources::intc::*;

pub enum Line {
    Vblank,
    Gpu,
    Cdrom,
    Dma,
    Tmr0,
    Tmr1,
    Tmr2,
    Padmc,
    Sio,
    Spu,
    Pio,
}

pub struct Stat {
    vblank: AtomicBool,
    gpu: AtomicBool,
    cdrom: AtomicBool,
    dma: AtomicBool,
    tmr0: AtomicBool,
    tmr1: AtomicBool,
    tmr2: AtomicBool,
    padmc: AtomicBool,
    sio: AtomicBool,
    spu: AtomicBool,
    pio: AtomicBool,
}

impl Stat {
    pub fn new() -> Stat {
        Stat {
            vblank: AtomicBool::new(false),
            gpu: AtomicBool::new(false),
            cdrom: AtomicBool::new(false),
            dma: AtomicBool::new(false),
            tmr0: AtomicBool::new(false),
            tmr1: AtomicBool::new(false),
            tmr2: AtomicBool::new(false),
            padmc: AtomicBool::new(false),
            sio: AtomicBool::new(false),
            spu: AtomicBool::new(false),
            pio: AtomicBool::new(false),
        }
    }

    pub fn assert_line(&self, line: Line) {
        match line {
            Line::Vblank => self.vblank.store(true, Ordering::Release),
            Line::Gpu => self.gpu.store(true, Ordering::Release),
            Line::Cdrom => self.cdrom.store(true, Ordering::Release),
            Line::Dma => self.dma.store(true, Ordering::Release),
            Line::Tmr0 => self.tmr0.store(true, Ordering::Release),
            Line::Tmr1 => self.tmr1.store(true, Ordering::Release),
            Line::Tmr2 => self.tmr2.store(true, Ordering::Release),
            Line::Padmc => self.padmc.store(true, Ordering::Release),
            Line::Sio => self.sio.store(true, Ordering::Release),
            Line::Spu => self.spu.store(true, Ordering::Release),
            Line::Pio => self.pio.store(true, Ordering::Release),
        }
    }

    fn acknowledge(&self, acknowledge_mask: u32) {
        for i in 0..32 {
            let acknowledged = ((acknowledge_mask >> i) & 1) > 0;
            if acknowledged {
                match i {
                    0 => self.vblank.store(false, Ordering::Release),
                    1 => self.gpu.store(false, Ordering::Release),
                    2 => self.cdrom.store(false, Ordering::Release),
                    3 => self.dma.store(false, Ordering::Release),
                    4 => self.tmr0.store(false, Ordering::Release),
                    5 => self.tmr1.store(false, Ordering::Release),
                    6 => self.tmr2.store(false, Ordering::Release),
                    7 => self.padmc.store(false, Ordering::Release),
                    8 => self.sio.store(false, Ordering::Release),
                    9 => self.spu.store(false, Ordering::Release),
                    10 => self.pio.store(false, Ordering::Release),
                    // Ignore (always zero).
                    _ => {}, 
                }
            }
        }
        log::debug!("value = 0x{:08X}", self.value());
    }

    pub fn value(&self) -> u32 {
        let mut value = 0;
        value = VBLANK.insert_into(value, bool_to_flag(self.vblank.load(Ordering::Acquire)));
        value = GPU.insert_into(value, bool_to_flag(self.gpu.load(Ordering::Acquire)));
        value = CDROM.insert_into(value, bool_to_flag(self.cdrom.load(Ordering::Acquire)));
        value = DMA.insert_into(value, bool_to_flag(self.dma.load(Ordering::Acquire)));
        value = TMR0.insert_into(value, bool_to_flag(self.tmr0.load(Ordering::Acquire)));
        value = TMR1.insert_into(value, bool_to_flag(self.tmr1.load(Ordering::Acquire)));
        value = TMR2.insert_into(value, bool_to_flag(self.tmr2.load(Ordering::Acquire)));
        value = PADMC.insert_into(value, bool_to_flag(self.padmc.load(Ordering::Acquire)));
        value = SIO.insert_into(value, bool_to_flag(self.sio.load(Ordering::Acquire)));
        value = SPU.insert_into(value, bool_to_flag(self.spu.load(Ordering::Acquire)));
        value = PIO.insert_into(value, bool_to_flag(self.pio.load(Ordering::Acquire)));
        value
    }
}

impl B8MemoryMap for Stat {
    fn read_u16(&mut self, _offset: u32) -> ReadResult<u16> {
        Ok(self.value() as u16)
    }
    
    fn write_u16(&mut self, _offset: u32, value: u16) -> WriteResult {
        Ok(self.acknowledge(value as u32))
    }

    fn read_u32(&mut self, _offset: u32) -> ReadResult<u32> {
        Ok(self.value())
    }
    
    fn write_u32(&mut self, _offset: u32, value: u32) -> WriteResult {
        Ok(self.acknowledge(value))
    }
}
