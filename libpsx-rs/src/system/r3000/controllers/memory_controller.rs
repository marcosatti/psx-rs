use crate::system::{
    bus::memory::*,
    r3000::{
        controllers::debug,
        types::*,
    },
    types::{
        ControllerResult,
        State,
    },
};

pub(crate) fn translate_address(va: u32) -> u32 {
    match va {
        // kuseg.
        // The PSX doesn't have a TLB, but it also uses a special mapping that differs from the standard MIPS documentation.
        0x0000_0000..=0x7FFF_FFFF => va,
        // kseg0.
        0x8000_0000..=0x9FFF_FFFF => va - 0x8000_0000,
        // kseg1.
        0xA000_0000..=0xBFFF_FFFF => va - 0xA000_0000,
        // kseg2.
        0xC000_0000..=0xFFFD_FFFF => unimplemented!("Address translation reached kseg2 - unimplemented"),
        // Cache control i/o ports (PSX specific).
        0xFFFE_0000..=0xFFFF_FFFF => va,
    }
}

pub(crate) fn read_u8(state: &State, r3000_state: &ControllerState, physical_address: u32) -> ControllerResult<Result<u8, Hazard>> {
    let result = {
        if state.bus_locked.load() {
            return Ok(Err(Hazard::BusLockedMemoryRead(physical_address)));
        }

        debug::track_memory_read_pending::<u8>(r3000_state, physical_address);
        bus_read_u8(state, physical_address).map_err(|_| Hazard::MemoryRead(physical_address))
    };

    if result.is_ok() {
        debug::track_memory_read(r3000_state, physical_address, result.unwrap());
    }

    Ok(result)
}

pub(crate) fn write_u8(state: &State, r3000_state: &ControllerState, physical_address: u32, value: u8) -> ControllerResult<Result<(), Hazard>> {
    let result = {
        if state.bus_locked.load() {
            return Ok(Err(Hazard::BusLockedMemoryWrite(physical_address)));
        }

        debug::track_memory_write_pending(r3000_state, physical_address, value);
        bus_write_u8(state, physical_address, value).map_err(|_| Hazard::MemoryWrite(physical_address))
    };

    if result.is_ok() {
        debug::track_memory_write(r3000_state, physical_address, value);
    }

    Ok(result)
}

pub(crate) fn read_u16(state: &State, r3000_state: &ControllerState, physical_address: u32) -> ControllerResult<Result<u16, Hazard>> {
    let result = {
        if state.bus_locked.load() {
            return Ok(Err(Hazard::BusLockedMemoryRead(physical_address)));
        }

        debug::track_memory_read_pending::<u16>(r3000_state, physical_address);
        bus_read_u16(state, physical_address).map_err(|_| Hazard::MemoryRead(physical_address))
    };

    if result.is_ok() {
        debug::track_memory_read(r3000_state, physical_address, result.unwrap());
    }

    Ok(result)
}

pub(crate) fn write_u16(state: &State, r3000_state: &ControllerState, physical_address: u32, value: u16) -> ControllerResult<Result<(), Hazard>> {
    let result = {
        if state.bus_locked.load() {
            return Ok(Err(Hazard::BusLockedMemoryWrite(physical_address)));
        }

        debug::track_memory_write_pending(r3000_state, physical_address, value);
        bus_write_u16(state, physical_address, value).map_err(|_| Hazard::MemoryWrite(physical_address))
    };

    if result.is_ok() {
        debug::track_memory_write(r3000_state, physical_address, value);
    }

    Ok(result)
}

pub(crate) fn read_u32(state: &State, r3000_state: &ControllerState, physical_address: u32) -> ControllerResult<Result<u32, Hazard>> {
    let result = {
        if state.bus_locked.load() {
            return Ok(Err(Hazard::BusLockedMemoryRead(physical_address)));
        }

        debug::track_memory_read_pending::<u32>(r3000_state, physical_address);
        bus_read_u32(state, physical_address).map_err(|_| Hazard::MemoryRead(physical_address))
    };

    if result.is_ok() {
        debug::track_memory_read(r3000_state, physical_address, result.unwrap());
    }

    Ok(result)
}

pub(crate) fn write_u32(state: &State, r3000_state: &ControllerState, physical_address: u32, value: u32) -> ControllerResult<Result<(), Hazard>> {
    let result = {
        if state.bus_locked.load() {
            return Ok(Err(Hazard::BusLockedMemoryWrite(physical_address)));
        }

        debug::track_memory_write_pending(r3000_state, physical_address, value);
        bus_write_u32(state, physical_address, value).map_err(|_| Hazard::MemoryWrite(physical_address))
    };

    if result.is_ok() {
        debug::track_memory_write(r3000_state, physical_address, value);
    }

    Ok(result)
}
