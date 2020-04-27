use crate::{
    system::{
        r3000::cp0::constants::*,
        types::State as SystemState,
    },
    types::mips1::register::*,
};
use std::{
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
};
use parking_lot::Mutex;

#[derive(Copy, Clone, Debug)]
pub enum IrqLine {
    Intc,
}

pub struct Interrupt {
    intc_pending: AtomicBool,
}

impl Interrupt {
    pub fn new() -> Interrupt {
        Interrupt {
            intc_pending: AtomicBool::new(false),
        }
    }

    pub fn assert_line(&self, irq_line: IrqLine) {
        match irq_line {
            IrqLine::Intc => self.intc_pending.store(true, Ordering::Release),
        }
    }

    pub fn deassert_line(&self, irq_line: IrqLine) {
        match irq_line {
            IrqLine::Intc => self.intc_pending.store(false, Ordering::Release),
        }
    }

    pub fn line_interrupted(&self, irq_line: IrqLine) -> bool {
        match irq_line {
            IrqLine::Intc => self.intc_pending.load(Ordering::Acquire),
        }
    }
}

pub struct ControllerState {
    pub bpc: Register,
    pub bda: Register,
    pub jump_dest: Register,
    pub dcic: Register,
    pub bdam: Register,
    pub bpcm: Register,
    pub status: Register,
    pub cause: Register,
    pub epc: Register,
    pub prid: Register,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
            bpc: Register::new(),
            bda: Register::new(),
            jump_dest: Register::new(),
            dcic: Register::new(),
            bdam: Register::new(),
            bpcm: Register::new(),
            status: Register::new(),
            cause: Register::new(),
            epc: Register::new(),
            prid: Register::new(),
        }
    }
}

pub struct State {
    pub interrupt: Interrupt,
    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            interrupt: Interrupt::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}

pub fn initialize(state: &mut SystemState) {
    state.r3000.cp0.controller_state.get_mut().prid.write_u32(initialize_prid());
    state.r3000.cp0.controller_state.get_mut().status.write_bitfield(STATUS_KUC, 0);
    state.r3000.cp0.controller_state.get_mut().status.write_bitfield(STATUS_IEC, 0);
    state.r3000.cp0.controller_state.get_mut().status.write_bitfield(STATUS_BEV, 1);
    state.r3000.cp0.controller_state.get_mut().status.write_bitfield(STATUS_TS, 1);
}

fn initialize_prid() -> u32 {
    let mut value: u32 = 0;
    value = PRID_REVISION.insert_into(value, 2);
    value = PRID_IMPLEMENTATION.insert_into(value, 0);
    value
}
