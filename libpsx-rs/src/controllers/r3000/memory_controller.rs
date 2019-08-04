use crate::controllers::r3000::*;

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

pub unsafe fn read_u8(state: &State, physical_address: u32) -> Result<u8, Hazard> {
    let resources = &mut *state.resources;

    if resources.bus_locked {
        return Err(Hazard::MemoryRead);
    }

    resources.r3000.memory_mapper.read_u8(physical_address).map_err(|_| Hazard::MemoryRead)
}

pub unsafe fn write_u8(state: &State, physical_address: u32, value: u8) -> Result<(), Hazard> {
    let resources = &mut *state.resources;

    if resources.bus_locked {
        return Err(Hazard::MemoryWrite);
    }

    resources.r3000.memory_mapper.write_u8(physical_address, value).map_err(|_| Hazard::MemoryWrite)
}

pub unsafe fn read_u16(state: &State, physical_address: u32) -> Result<u16, Hazard> {
    let resources = &mut *state.resources;
    
    if resources.bus_locked {
        return Err(Hazard::MemoryRead);
    }

    resources.r3000.memory_mapper.read_u16(physical_address).map_err(|_| Hazard::MemoryRead)
}

pub unsafe fn write_u16(state: &State, physical_address: u32, value: u16) -> Result<(), Hazard> {
    let resources = &mut *state.resources;

    if resources.bus_locked {
        return Err(Hazard::MemoryWrite);
    }

    resources.r3000.memory_mapper.write_u16(physical_address, value).map_err(|_| Hazard::MemoryWrite)
}

pub unsafe fn read_u32(state: &State, physical_address: u32) -> Result<u32, Hazard> {
    let resources = &mut *state.resources;
    
    if resources.bus_locked {
        return Err(Hazard::MemoryRead);
    }

    resources.r3000.memory_mapper.read_u32(physical_address).map_err(|_| Hazard::MemoryRead)
}

pub unsafe fn write_u32(state: &State, physical_address: u32, value: u32) -> Result<(), Hazard> {
    let resources = &mut *state.resources;

    if resources.bus_locked {
        return Err(Hazard::MemoryWrite);
    }

    resources.r3000.memory_mapper.write_u32(physical_address, value).map_err(|_| Hazard::MemoryWrite)
}
