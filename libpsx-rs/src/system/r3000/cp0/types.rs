use crate::{
    system::{
        r3000::cp0::constants::*,
        types::State as SystemState,
    },
    types::mips1::register::*,
    utilities::bool_to_flag,
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

pub struct Cause {
    pub register: Register,
    intc_pending: AtomicBool,
}

impl Cause {
    pub fn new() -> Cause {
        Cause {
            register: Register::new(),
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

    pub fn update_ip_field(&mut self) {
        self.register.write_bitfield(CAUSE_IP_INTC, bool_to_flag(self.intc_pending.load(Ordering::Acquire)));
    }

    pub fn clear_ip_field(&mut self) {
        self.intc_pending.store(false, Ordering::Release);
        self.register.write_bitfield(CAUSE_IP, 0);
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
    pub cause: Cause,
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
            cause: Cause::new(),
            epc: Register::new(),
            prid: Register::new(),
        }
    }
}

pub struct State {
    controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
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
