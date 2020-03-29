use crate::{
    system::{
        r3000::cp0::constants::*,
        types::State as SystemState,
    },
    types::register::b32_register::B32Register,
    utilities::bool_to_flag,
};
use std::{
    ptr::NonNull,
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
};

#[derive(Copy, Clone, Debug)]
pub enum IrqLine {
    Intc,
}

pub struct State {
    pub bpc: B32Register,
    pub bda: B32Register,
    pub jump_dest: B32Register,
    pub dcic: B32Register,
    pub bdam: B32Register,
    pub bpcm: B32Register,
    pub status: B32Register,
    pub cause: Cause,
    pub epc: B32Register,
    pub prid: B32Register,
    pub register: [Option<NonNull<B32Register>>; 64],
}

impl State {
    pub fn new() -> State {
        State {
            bpc: B32Register::new(),
            bda: B32Register::new(),
            jump_dest: B32Register::new(),
            dcic: B32Register::new(),
            bdam: B32Register::new(),
            bpcm: B32Register::new(),
            status: B32Register::new(),
            cause: Cause::new(),
            epc: B32Register::new(),
            prid: B32Register::new(),
            register: [None; 64],
        }
    }
}

pub struct Cause {
    pub register: B32Register,
    intc_pending: AtomicBool,
}

impl Cause {
    pub fn new() -> Cause {
        Cause {
            register: B32Register::new(),
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

pub fn initialize(state: &mut SystemState) {
    state.r3000.cp0.prid.write_u32(initialize_prid());

    state.r3000.cp0.status.write_bitfield(STATUS_KUC, 0);
    state.r3000.cp0.status.write_bitfield(STATUS_IEC, 0);
    state.r3000.cp0.status.write_bitfield(STATUS_BEV, 1);
    state.r3000.cp0.status.write_bitfield(STATUS_TS, 1);

    state.r3000.cp0.register[3] = Some(NonNull::new(&mut state.r3000.cp0.bpc as *mut B32Register).unwrap());
    state.r3000.cp0.register[5] = Some(NonNull::new(&mut state.r3000.cp0.bda as *mut B32Register).unwrap());
    state.r3000.cp0.register[6] = Some(NonNull::new(&mut state.r3000.cp0.jump_dest as *mut B32Register).unwrap());
    state.r3000.cp0.register[7] = Some(NonNull::new(&mut state.r3000.cp0.dcic as *mut B32Register).unwrap());
    state.r3000.cp0.register[9] = Some(NonNull::new(&mut state.r3000.cp0.bdam as *mut B32Register).unwrap());
    state.r3000.cp0.register[11] = Some(NonNull::new(&mut state.r3000.cp0.bpcm as *mut B32Register).unwrap());
    state.r3000.cp0.register[12] = Some(NonNull::new(&mut state.r3000.cp0.status as *mut B32Register).unwrap());
    state.r3000.cp0.register[13] = Some(NonNull::new(&mut state.r3000.cp0.cause.register as *mut B32Register).unwrap());
    state.r3000.cp0.register[14] = Some(NonNull::new(&mut state.r3000.cp0.epc as *mut B32Register).unwrap());
    state.r3000.cp0.register[15] = Some(NonNull::new(&mut state.r3000.cp0.prid as *mut B32Register).unwrap());
}

fn initialize_prid() -> u32 {
    let mut value: u32 = 0;
    value = PRID_REVISION.insert_into(value, 2);
    value = PRID_IMPLEMENTATION.insert_into(value, 0);
    value
}
