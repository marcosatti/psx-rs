use crate::{
    system::cdrom::constants::*,
    types::{
        memory::*,
        bitfield::Bitfield,
        fifo::{
            debug::DebugState,
            Fifo,
        },
    },
};
use std::{
    collections::VecDeque,
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
};
use parking_lot::Mutex;

pub struct Command {
    pub register: B8Register,
    pub write_latch: AtomicBool,
}

impl Command {
    pub fn new() -> Command {
        Command {
            register: B8Register::new(),
            write_latch: AtomicBool::new(false),
        }
    }

    pub fn read_u8(&self) -> u8 {
        self.register.read_u8()
    }

    pub fn write_u8(&self, value: u8) {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        self.register.write_u8(value);
    }
}

pub struct IntEnable {
    pub register: B8Register,
    pub write_latch: AtomicBool,
}

impl IntEnable {
    pub fn new() -> IntEnable {
        IntEnable {
            register: B8Register::new(),
            write_latch: AtomicBool::new(false),
        }
    }

    pub fn read_u8(&self) -> u8 {
        self.register.read_u8()
    }

    pub fn write_u8(&self, value: u8) {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still pending");
        let mut register_value = self.register.read_u8();
        register_value = INTERRUPT_FLAGS.insert_into(register_value, value);
        self.register.write_u8(register_value);
    }
}

pub struct Request {
    pub register: B8Register,
    pub write_latch: AtomicBool,
}

impl Request {
    pub fn new() -> Request {
        Request {
            register: B8Register::new(),
            write_latch: AtomicBool::new(false),
        }
    }

    pub fn read_u8(&self) -> u8 {
        self.register.read_u8()
    }

    pub fn write_u8(&self, value: u8) {
        // assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still pending");
        self.write_latch.store(true, Ordering::Release);
        self.register.write_u8(value);
    }
}

pub struct IntFlag {
    pub register: B8Register,
    pub write_latch: AtomicBool,
    pub parameter_reset: AtomicBool,
}

impl IntFlag {
    pub fn new() -> IntFlag {
        IntFlag {
            register: B8Register::new(),
            write_latch: AtomicBool::new(false),
            parameter_reset: AtomicBool::new(false),
        }
    }

    pub fn set_interrupt(&self, line: usize) {
        assert!(line <= 10, "Invalid interrupt index");
        let value = self.register.read_u8() | (line as u8);
        self.register.write_u8(value);
    }

    pub fn read_u8(&self) -> u8 {
        self.register.read_u8()
    }

    pub fn write_u8(&mut self, value: u8) {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still pending");
        assert!(!self.parameter_reset.load(Ordering::Acquire), "Parameter FIFO reset still pending");
        self.write_latch.store(true, Ordering::Release);

        if INT_FLAG_CLRPRM.extract_from(value) != 0 {
            self.parameter_reset.store(true, Ordering::Release);
        }

        let mut register_value = self.register.read_u8();
        register_value = INTERRUPT_FLAGS.acknowledge(register_value, value);
        register_value = Bitfield::new(5, 1).insert_into(register_value, 1);
        register_value = Bitfield::new(6, 1).insert_into(register_value, 1);
        register_value = Bitfield::new(7, 1).insert_into(register_value, 1);

        self.register.write_u8(register_value);
    }
}

pub struct ControllerState {
    /// Command state.
    pub command_index: Option<u8>,
    pub command_iteration: usize,

    /// Pausing status.
    pub pausing: bool,
    /// Seeking status.
    pub seeking: bool,
    /// Reading status.
    pub reading: bool,
    pub read_buffer: VecDeque<u8>,
    /// Current MSF address.
    /// Base is stored in BCD format.
    pub msf_address_base: (u8, u8, u8),
    pub msf_address_offset: usize,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
            command_index: None,
            command_iteration: 0,
            pausing: false,
            seeking: false,
            reading: false,
            read_buffer: VecDeque::with_capacity(2048),
            msf_address_base: (0, 0, 0),
            msf_address_offset: 0,
        }
    }
}

pub struct State {
    pub status: B8Register,
    pub command: Command,
    pub response: Fifo<u8>,
    pub parameter: Fifo<u8>,
    pub data: Fifo<u8>,
    pub int_enable: IntEnable,
    pub int_flag: IntFlag,
    pub request: Request,
    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            status: B8Register::new(),
            command: Command::new(),
            response: Fifo::new(16, Some(DebugState::new("CDROM RESPONSE", true, true))),
            parameter: Fifo::new(16, Some(DebugState::new("CDROM PARAMETER", true, true))),
            data: Fifo::new(64, Some(DebugState::new("CDROM DATA", true, true))),
            int_enable: IntEnable::new(),
            int_flag: IntFlag::new(),
            request: Request::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
