use crate::{
    system::{
        r3000::cp0::constants::*,
        types::State as SystemState,
    },
    types::mips1::register::*,
};
use parking_lot::Mutex;
use std::sync::atomic::{
    AtomicBool,
    Ordering,
};

#[derive(Copy, Clone, Debug)]
pub(crate) enum IrqLine {
    Intc,
}

pub(crate) struct Interrupt {
    intc_pending: AtomicBool,
}

impl Interrupt {
    pub(crate) fn new() -> Interrupt {
        Interrupt {
            intc_pending: AtomicBool::new(false),
        }
    }

    pub(crate) fn assert_line(&self, irq_line: IrqLine) {
        match irq_line {
            IrqLine::Intc => self.intc_pending.store(true, Ordering::Release),
        }
    }

    pub(crate) fn deassert_line(&self, irq_line: IrqLine) {
        match irq_line {
            IrqLine::Intc => self.intc_pending.store(false, Ordering::Release),
        }
    }

    pub(crate) fn line_interrupted(&self, irq_line: IrqLine) -> bool {
        match irq_line {
            IrqLine::Intc => self.intc_pending.load(Ordering::Acquire),
        }
    }
}

pub(crate) struct ControllerState {
    pub(crate) bpc: Register,
    pub(crate) bda: Register,
    pub(crate) jump_dest: Register,
    pub(crate) dcic: Register,
    pub(crate) bdam: Register,
    pub(crate) bpcm: Register,
    pub(crate) status: Register,
    pub(crate) cause: Register,
    pub(crate) epc: Register,
    pub(crate) prid: Register,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
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

pub(crate) struct State {
    pub(crate) interrupt: Interrupt,
    pub(crate) controller_state: Mutex<ControllerState>,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            interrupt: Interrupt::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}

pub(crate) fn initialize(state: &mut SystemState) {
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
