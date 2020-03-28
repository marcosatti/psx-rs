use std::sync::atomic::Ordering;
use crate::system::Resources;
use crate::controllers::r3000::*;
use crate::controllers::r3000::debug;

pub fn translate_address(va: u32) -> u32 {
    match va {
        // kuseg.
        // The PSX doesn't have a TLB, but it also uses a special mapping that
        // differs from the standard MIPS documentation.
        0x0000_0000..=0x7FFF_FFFF => {
            va
        },
        // kseg0.
        0x8000_0000..=0x9FFF_FFFF => {
            va - 0x8000_0000
        },
        // kseg1.
        0xA000_0000..=0xBFFF_FFFF => {
            va - 0xA000_0000
        },
        // kseg2.
        0xC000_0000..=0xFFFD_FFFF => {
            unimplemented!("Address translation reached kseg2 - unimplemented")
        },
        // Cache control i/o ports (PSX specific).
        0xFFFE_0000..=0xFFFF_FFFF => {
            va
        },
    }
}

pub fn read_u8(resources: &mut Resources, physical_address: u32) -> Result<u8, Hazard> {
    let result = {
        if resources.bus_locked.load(Ordering::Acquire) {
            return Err(Hazard::BusLockedMemoryRead(physical_address));
        }

        debug::track_memory_read_pending::<u8>(resources, physical_address);
        resources.r3000.memory_mapper.read_u8(physical_address).map_err(|_| Hazard::MemoryRead(physical_address))
    };

    if result.is_ok() {
        debug::track_memory_read(resources, physical_address, result.unwrap());
    }

    result
}

pub fn write_u8(resources: &mut Resources, physical_address: u32, value: u8) -> Result<(), Hazard> {
    let result = {
        if resources.bus_locked.load(Ordering::Acquire) {
            return Err(Hazard::BusLockedMemoryWrite(physical_address));
        }

        debug::track_memory_write_pending(resources, physical_address, value);
        resources.r3000.memory_mapper.write_u8(physical_address, value).map_err(|_| Hazard::MemoryWrite(physical_address))
    };

    if result.is_ok() {
        debug::track_memory_write(resources, physical_address, value);
    }

    result
}

pub fn read_u16(resources: &mut Resources, physical_address: u32) -> Result<u16, Hazard> {
    let result = {
        if resources.bus_locked.load(Ordering::Acquire) {
            return Err(Hazard::BusLockedMemoryRead(physical_address));
        }

        debug::track_memory_read_pending::<u16>(resources, physical_address);
        resources.r3000.memory_mapper.read_u16(physical_address).map_err(|_| Hazard::MemoryRead(physical_address))
    };

    if result.is_ok() {
        debug::track_memory_read(resources, physical_address, result.unwrap());
    }

    result
}

pub fn write_u16(resources: &mut Resources, physical_address: u32, value: u16) -> Result<(), Hazard> {
    let result = {
        if resources.bus_locked.load(Ordering::Acquire) {
            return Err(Hazard::BusLockedMemoryWrite(physical_address));
        }

        debug::track_memory_write_pending(resources, physical_address, value);
        resources.r3000.memory_mapper.write_u16(physical_address, value).map_err(|_| Hazard::MemoryWrite(physical_address))
    };
    
    if result.is_ok() {
        debug::track_memory_write(resources, physical_address, value);
    }

    result
}

pub fn read_u32(resources: &mut Resources, physical_address: u32) -> Result<u32, Hazard> {
    let result = {
        if resources.bus_locked.load(Ordering::Acquire) {
            return Err(Hazard::BusLockedMemoryRead(physical_address));
        }

        debug::track_memory_read_pending::<u32>(resources, physical_address);
        resources.r3000.memory_mapper.read_u32(physical_address).map_err(|_| Hazard::MemoryRead(physical_address))
    };

    if result.is_ok() {
        debug::track_memory_read(resources, physical_address, result.unwrap());
    }

    result
}

pub fn write_u32(resources: &mut Resources, physical_address: u32, value: u32) -> Result<(), Hazard> {
    let result = {
        if resources.bus_locked.load(Ordering::Acquire) {
            return Err(Hazard::BusLockedMemoryWrite(physical_address));
        }

        debug::track_memory_write_pending(resources, physical_address, value);
        resources.r3000.memory_mapper.write_u32(physical_address, value).map_err(|_| Hazard::MemoryWrite(physical_address))    
    };

    if result.is_ok() {
        debug::track_memory_write(resources, physical_address, value);
    }

    result
}