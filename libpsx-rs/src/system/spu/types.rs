use crate::{
    system::types::State as SystemState,
    types::{
        b8_memory_mapper::{
            B8MemoryMap,
            *,
        },
        bitfield::Bitfield,
        fifo::{
            debug::DebugState,
            Fifo,
        },
        memory::b8_memory::B8Memory,
        register::{
            b16_register::B16Register,
            b32_register::B32Register,
        },
        stereo::*,
    },
};
use parking_lot::Mutex;
use std::{
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
    time::Duration,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TransferMode {
    Stop,
    ManualWrite,
    DmaWrite,
    DmaRead,
}

#[derive(Debug)]
pub enum AdsrMode {
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct DacState {
    pub current_duration: Duration,
    pub voice0_state: PlayState,
    pub voice1_state: PlayState,
    pub voice2_state: PlayState,
    pub voice3_state: PlayState,
    pub voice4_state: PlayState,
    pub voice5_state: PlayState,
    pub voice6_state: PlayState,
    pub voice7_state: PlayState,
    pub voice8_state: PlayState,
    pub voice9_state: PlayState,
    pub voice10_state: PlayState,
    pub voice11_state: PlayState,
    pub voice12_state: PlayState,
    pub voice13_state: PlayState,
    pub voice14_state: PlayState,
    pub voice15_state: PlayState,
    pub voice16_state: PlayState,
    pub voice17_state: PlayState,
    pub voice18_state: PlayState,
    pub voice19_state: PlayState,
    pub voice20_state: PlayState,
    pub voice21_state: PlayState,
    pub voice22_state: PlayState,
    pub voice23_state: PlayState,
}

impl DacState {
    pub fn new() -> DacState {
        DacState {
            current_duration: Duration::from_secs(0),
            voice0_state: PlayState::new(),
            voice1_state: PlayState::new(),
            voice2_state: PlayState::new(),
            voice3_state: PlayState::new(),
            voice4_state: PlayState::new(),
            voice5_state: PlayState::new(),
            voice6_state: PlayState::new(),
            voice7_state: PlayState::new(),
            voice8_state: PlayState::new(),
            voice9_state: PlayState::new(),
            voice10_state: PlayState::new(),
            voice11_state: PlayState::new(),
            voice12_state: PlayState::new(),
            voice13_state: PlayState::new(),
            voice14_state: PlayState::new(),
            voice15_state: PlayState::new(),
            voice16_state: PlayState::new(),
            voice17_state: PlayState::new(),
            voice18_state: PlayState::new(),
            voice19_state: PlayState::new(),
            voice20_state: PlayState::new(),
            voice21_state: PlayState::new(),
            voice22_state: PlayState::new(),
            voice23_state: PlayState::new(),
        }
    }
}

pub struct DataFifo {
    pub fifo: Fifo<u16>,
}

impl DataFifo {
    pub fn new() -> DataFifo {
        DataFifo {
            fifo: Fifo::new(64, Some(DebugState::new("SPU FIFO", false, false))),
        }
    }
}

impl B8MemoryMap for DataFifo {
    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        assert!(offset == 0, "Invalid offset");
        self.fifo.write_one(value).map_err(|_| WriteError::Full)
    }
}

pub struct TransferAddress {
    pub register: B16Register,
    pub write_latch: AtomicBool,
}

impl TransferAddress {
    pub fn new() -> TransferAddress {
        TransferAddress {
            register: B16Register::new(),
            write_latch: AtomicBool::new(false),
        }
    }
}

impl B8MemoryMap for TransferAddress {
    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }

    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        B8MemoryMap::write_u16(&mut self.register, offset, value)
    }
}

pub struct VoiceKey {
    pub register: B32Register,
    pub write_latch: [bool; 32],
    pub mutex: Mutex<()>,
}

impl VoiceKey {
    pub fn new() -> VoiceKey {
        VoiceKey {
            register: B32Register::new(),
            write_latch: [false; 32],
            mutex: Mutex::new(()),
        }
    }
}

impl B8MemoryMap for VoiceKey {
    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }

    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        let _lock = self.mutex.lock();
        B8MemoryMap::write_u16(&mut self.register, offset, value).unwrap();
        for i in 0..16 {
            self.write_latch[((offset * 8) + (i as u32)) as usize] = Bitfield::new(i, 1).extract_from(value) != 0;
        }
        Ok(())
    }

    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.register, offset)
    }

    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        let _lock = self.mutex.lock();
        B8MemoryMap::write_u32(&mut self.register, offset, value).unwrap();
        for i in 0..32 {
            self.write_latch[i] = Bitfield::new(i, 1).extract_from(value) != 0;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct AdpcmParams {
    pub filter: usize,
    pub shift: usize,
    pub loop_end: bool,
    pub loop_repeat: bool,
    pub loop_start: bool,
}

impl AdpcmParams {
    pub fn new() -> AdpcmParams {
        AdpcmParams {
            filter: 0,
            shift: 0,
            loop_end: false,
            loop_repeat: false,
            loop_start: false,
        }
    }
}

#[derive(Debug)]
pub struct AdpcmState {
    /// Sample memory for decoding
    pub old_sample: i16,
    pub older_sample: i16,
    /// Decoded ADPCM header parameters
    pub params: AdpcmParams,
    /// Decoded raw ADPCM samples
    pub sample_buffer: Option<[i16; 28]>,
}

impl AdpcmState {
    pub fn new() -> AdpcmState {
        AdpcmState {
            old_sample: 0,
            older_sample: 0,
            params: AdpcmParams::new(),
            sample_buffer: None,
        }
    }
}

#[derive(Debug)]
pub struct PlayState {
    /// Voice (ADPCM) decoding address
    pub current_address: usize,
    /// ADPCM decoding state
    pub adpcm_state: AdpcmState,
    /// Voice sample/pitch counter
    /// Explanation (docs were a bit confusing):
    /// This is used as an interpolation counter; the SPU always plays samples at 44100 Hz,
    /// but needs to interpolate between samples when the sample rate register is not 0x1000
    /// (44100 Hz). The upper 12+ bits are used as the ADPCM sample index, while the lower
    /// 0-11 bits are used as an interpolation index. Notice that when the sample rate is
    /// 0x1000 (this is added to the pitch counter on every tick), the ADPCM index is always
    /// incrementing by 1, and the interpolation index by 0 (ie: perfectly in sync with
    /// samples). When the sample rate is, say, 0x800 (22050 Hz), then sample 0 is used on
    /// tick 0 and 1, with interpolation being used on tick 1.
    pub pitch_counter_base: usize,
    pub pitch_counter_interp: usize,
    /// Sample memory
    /// These samples are used in the interpolation process described above. As I understand
    /// it, these are the actual full ("base") samples used on each tick by the 12+ bits part
    /// of the sample counter. This is different to the ADPCM decoding sample memory, which
    /// is only related to the decoding process.
    pub old_sample: i16,
    pub older_sample: i16,
    pub oldest_sample: i16,
    /// PCM sample buffer
    /// This is filled with the output of the SPU after all processing is done.
    pub sample_buffer: Vec<Stereo>,
    /// ADSR mode (attack, decay, sustain, release)
    pub adsr_mode: AdsrMode,
    /// ADSR volume
    /// Internally, it is described as a normalized scaling factor. This is to support delay
    /// cycles without losing accuracy / increasing emulator complexity.
    pub adsr_current_volume: f64,
}

impl PlayState {
    pub fn new() -> PlayState {
        PlayState {
            current_address: 0x1000,
            adpcm_state: AdpcmState::new(),
            pitch_counter_base: 0,
            pitch_counter_interp: 0,
            old_sample: 0,
            older_sample: 0,
            oldest_sample: 0,
            sample_buffer: Vec::new(),
            adsr_mode: AdsrMode::Attack,
            adsr_current_volume: 0.0,
        }
    }

    pub fn reset(&mut self, current_address: usize) {
        self.current_address = current_address;
        self.adpcm_state = AdpcmState::new();
        self.pitch_counter_base = 0;
        self.pitch_counter_interp = 0;
        self.old_sample = 0;
        self.older_sample = 0;
        self.oldest_sample = 0;
        self.sample_buffer.clear();
        self.adsr_mode = AdsrMode::Attack;
        self.adsr_current_volume = 0.0;
    }
}

pub struct State {
    pub main_volume_left: B16Register,
    pub main_volume_right: B16Register,
    pub reverb_volume: B32Register,

    pub voice_key_on: VoiceKey,
    pub voice_key_off: VoiceKey,
    pub voice_channel_fm: B32Register,
    pub voice_channel_noise: B32Register,
    pub voice_channel_reverb: B32Register,
    pub voice_channel_status: B32Register,

    pub unknown_0: B16Register,
    pub reverb_start_address: B16Register,
    pub irq_address: B16Register,
    pub data_transfer_address: TransferAddress,
    pub control: B16Register,
    pub data_transfer_control: B16Register,
    pub stat: B16Register,
    pub cd_volume: B32Register,
    pub extern_volume: B32Register,
    pub current_volume_left: B16Register,
    pub current_volume_right: B16Register,
    pub unknown_1: B32Register,

    pub dapf1: B16Register,
    pub dapf2: B16Register,
    pub viir: B16Register,
    pub vcomb1: B16Register,
    pub vcomb2: B16Register,
    pub vcomb3: B16Register,
    pub vcomb4: B16Register,
    pub vwall: B16Register,
    pub vapf1: B16Register,
    pub vapf2: B16Register,
    pub msame: B32Register,
    pub mcomb1: B32Register,
    pub mcomb2: B32Register,
    pub dsame: B32Register,
    pub mdiff: B32Register,
    pub mcomb3: B32Register,
    pub mcomb4: B32Register,
    pub ddiff: B32Register,
    pub mapf1: B32Register,
    pub mapf2: B32Register,
    pub vin: B32Register,

    pub voice0_voll: B16Register,
    pub voice0_volr: B16Register,
    pub voice0_srate: B16Register,
    pub voice0_saddr: B16Register,
    pub voice0_adsr: B32Register,
    pub voice0_cvol: B16Register,
    pub voice0_raddr: B16Register,

    pub voice1_voll: B16Register,
    pub voice1_volr: B16Register,
    pub voice1_srate: B16Register,
    pub voice1_saddr: B16Register,
    pub voice1_adsr: B32Register,
    pub voice1_cvol: B16Register,
    pub voice1_raddr: B16Register,

    pub voice2_voll: B16Register,
    pub voice2_volr: B16Register,
    pub voice2_srate: B16Register,
    pub voice2_saddr: B16Register,
    pub voice2_adsr: B32Register,
    pub voice2_cvol: B16Register,
    pub voice2_raddr: B16Register,

    pub voice3_voll: B16Register,
    pub voice3_volr: B16Register,
    pub voice3_srate: B16Register,
    pub voice3_saddr: B16Register,
    pub voice3_adsr: B32Register,
    pub voice3_cvol: B16Register,
    pub voice3_raddr: B16Register,

    pub voice4_voll: B16Register,
    pub voice4_volr: B16Register,
    pub voice4_srate: B16Register,
    pub voice4_saddr: B16Register,
    pub voice4_adsr: B32Register,
    pub voice4_cvol: B16Register,
    pub voice4_raddr: B16Register,

    pub voice5_voll: B16Register,
    pub voice5_volr: B16Register,
    pub voice5_srate: B16Register,
    pub voice5_saddr: B16Register,
    pub voice5_adsr: B32Register,
    pub voice5_cvol: B16Register,
    pub voice5_raddr: B16Register,

    pub voice6_voll: B16Register,
    pub voice6_volr: B16Register,
    pub voice6_srate: B16Register,
    pub voice6_saddr: B16Register,
    pub voice6_adsr: B32Register,
    pub voice6_cvol: B16Register,
    pub voice6_raddr: B16Register,

    pub voice7_voll: B16Register,
    pub voice7_volr: B16Register,
    pub voice7_srate: B16Register,
    pub voice7_saddr: B16Register,
    pub voice7_adsr: B32Register,
    pub voice7_cvol: B16Register,
    pub voice7_raddr: B16Register,

    pub voice8_voll: B16Register,
    pub voice8_volr: B16Register,
    pub voice8_srate: B16Register,
    pub voice8_saddr: B16Register,
    pub voice8_adsr: B32Register,
    pub voice8_cvol: B16Register,
    pub voice8_raddr: B16Register,

    pub voice9_voll: B16Register,
    pub voice9_volr: B16Register,
    pub voice9_srate: B16Register,
    pub voice9_saddr: B16Register,
    pub voice9_adsr: B32Register,
    pub voice9_cvol: B16Register,
    pub voice9_raddr: B16Register,

    pub voice10_voll: B16Register,
    pub voice10_volr: B16Register,
    pub voice10_srate: B16Register,
    pub voice10_saddr: B16Register,
    pub voice10_adsr: B32Register,
    pub voice10_cvol: B16Register,
    pub voice10_raddr: B16Register,

    pub voice11_voll: B16Register,
    pub voice11_volr: B16Register,
    pub voice11_srate: B16Register,
    pub voice11_saddr: B16Register,
    pub voice11_adsr: B32Register,
    pub voice11_cvol: B16Register,
    pub voice11_raddr: B16Register,

    pub voice12_voll: B16Register,
    pub voice12_volr: B16Register,
    pub voice12_srate: B16Register,
    pub voice12_saddr: B16Register,
    pub voice12_adsr: B32Register,
    pub voice12_cvol: B16Register,
    pub voice12_raddr: B16Register,

    pub voice13_voll: B16Register,
    pub voice13_volr: B16Register,
    pub voice13_srate: B16Register,
    pub voice13_saddr: B16Register,
    pub voice13_adsr: B32Register,
    pub voice13_cvol: B16Register,
    pub voice13_raddr: B16Register,

    pub voice14_voll: B16Register,
    pub voice14_volr: B16Register,
    pub voice14_srate: B16Register,
    pub voice14_saddr: B16Register,
    pub voice14_adsr: B32Register,
    pub voice14_cvol: B16Register,
    pub voice14_raddr: B16Register,

    pub voice15_voll: B16Register,
    pub voice15_volr: B16Register,
    pub voice15_srate: B16Register,
    pub voice15_saddr: B16Register,
    pub voice15_adsr: B32Register,
    pub voice15_cvol: B16Register,
    pub voice15_raddr: B16Register,

    pub voice16_voll: B16Register,
    pub voice16_volr: B16Register,
    pub voice16_srate: B16Register,
    pub voice16_saddr: B16Register,
    pub voice16_adsr: B32Register,
    pub voice16_cvol: B16Register,
    pub voice16_raddr: B16Register,

    pub voice17_voll: B16Register,
    pub voice17_volr: B16Register,
    pub voice17_srate: B16Register,
    pub voice17_saddr: B16Register,
    pub voice17_adsr: B32Register,
    pub voice17_cvol: B16Register,
    pub voice17_raddr: B16Register,

    pub voice18_voll: B16Register,
    pub voice18_volr: B16Register,
    pub voice18_srate: B16Register,
    pub voice18_saddr: B16Register,
    pub voice18_adsr: B32Register,
    pub voice18_cvol: B16Register,
    pub voice18_raddr: B16Register,

    pub voice19_voll: B16Register,
    pub voice19_volr: B16Register,
    pub voice19_srate: B16Register,
    pub voice19_saddr: B16Register,
    pub voice19_adsr: B32Register,
    pub voice19_cvol: B16Register,
    pub voice19_raddr: B16Register,

    pub voice20_voll: B16Register,
    pub voice20_volr: B16Register,
    pub voice20_srate: B16Register,
    pub voice20_saddr: B16Register,
    pub voice20_adsr: B32Register,
    pub voice20_cvol: B16Register,
    pub voice20_raddr: B16Register,

    pub voice21_voll: B16Register,
    pub voice21_volr: B16Register,
    pub voice21_srate: B16Register,
    pub voice21_saddr: B16Register,
    pub voice21_adsr: B32Register,
    pub voice21_cvol: B16Register,
    pub voice21_raddr: B16Register,

    pub voice22_voll: B16Register,
    pub voice22_volr: B16Register,
    pub voice22_srate: B16Register,
    pub voice22_saddr: B16Register,
    pub voice22_adsr: B32Register,
    pub voice22_cvol: B16Register,
    pub voice22_raddr: B16Register,

    pub voice23_voll: B16Register,
    pub voice23_volr: B16Register,
    pub voice23_srate: B16Register,
    pub voice23_saddr: B16Register,
    pub voice23_adsr: B32Register,
    pub voice23_cvol: B16Register,
    pub voice23_raddr: B16Register,

    pub data_fifo: DataFifo,

    pub memory: B8Memory,

    pub current_transfer_mode: TransferMode,
    pub current_transfer_address: u32,

    pub dac: DacState,
}

impl State {
    pub fn new() -> State {
        State {
            main_volume_left: B16Register::new(),
            main_volume_right: B16Register::new(),
            reverb_volume: B32Register::new(),
            voice_key_on: VoiceKey::new(),
            voice_key_off: VoiceKey::new(),
            voice_channel_fm: B32Register::new(),
            voice_channel_noise: B32Register::new(),
            voice_channel_reverb: B32Register::new(),
            voice_channel_status: B32Register::new(),
            unknown_0: B16Register::new(),
            reverb_start_address: B16Register::new(),
            irq_address: B16Register::new(),
            data_transfer_address: TransferAddress::new(),
            control: B16Register::new(),
            data_transfer_control: B16Register::new(),
            stat: B16Register::new(),
            cd_volume: B32Register::new(),
            extern_volume: B32Register::new(),
            current_volume_left: B16Register::new(),
            current_volume_right: B16Register::new(),
            unknown_1: B32Register::new(),
            dapf1: B16Register::new(),
            dapf2: B16Register::new(),
            viir: B16Register::new(),
            vcomb1: B16Register::new(),
            vcomb2: B16Register::new(),
            vcomb3: B16Register::new(),
            vcomb4: B16Register::new(),
            vwall: B16Register::new(),
            vapf1: B16Register::new(),
            vapf2: B16Register::new(),
            msame: B32Register::new(),
            mcomb1: B32Register::new(),
            mcomb2: B32Register::new(),
            dsame: B32Register::new(),
            mdiff: B32Register::new(),
            mcomb3: B32Register::new(),
            mcomb4: B32Register::new(),
            ddiff: B32Register::new(),
            mapf1: B32Register::new(),
            mapf2: B32Register::new(),
            vin: B32Register::new(),
            voice0_voll: B16Register::new(),
            voice0_volr: B16Register::new(),
            voice0_srate: B16Register::new(),
            voice0_saddr: B16Register::new(),
            voice0_adsr: B32Register::new(),
            voice0_cvol: B16Register::new(),
            voice0_raddr: B16Register::new(),
            voice1_voll: B16Register::new(),
            voice1_volr: B16Register::new(),
            voice1_srate: B16Register::new(),
            voice1_saddr: B16Register::new(),
            voice1_adsr: B32Register::new(),
            voice1_cvol: B16Register::new(),
            voice1_raddr: B16Register::new(),
            voice2_voll: B16Register::new(),
            voice2_volr: B16Register::new(),
            voice2_srate: B16Register::new(),
            voice2_saddr: B16Register::new(),
            voice2_adsr: B32Register::new(),
            voice2_cvol: B16Register::new(),
            voice2_raddr: B16Register::new(),
            voice3_voll: B16Register::new(),
            voice3_volr: B16Register::new(),
            voice3_srate: B16Register::new(),
            voice3_saddr: B16Register::new(),
            voice3_adsr: B32Register::new(),
            voice3_cvol: B16Register::new(),
            voice3_raddr: B16Register::new(),
            voice4_voll: B16Register::new(),
            voice4_volr: B16Register::new(),
            voice4_srate: B16Register::new(),
            voice4_saddr: B16Register::new(),
            voice4_adsr: B32Register::new(),
            voice4_cvol: B16Register::new(),
            voice4_raddr: B16Register::new(),
            voice5_voll: B16Register::new(),
            voice5_volr: B16Register::new(),
            voice5_srate: B16Register::new(),
            voice5_saddr: B16Register::new(),
            voice5_adsr: B32Register::new(),
            voice5_cvol: B16Register::new(),
            voice5_raddr: B16Register::new(),
            voice6_voll: B16Register::new(),
            voice6_volr: B16Register::new(),
            voice6_srate: B16Register::new(),
            voice6_saddr: B16Register::new(),
            voice6_adsr: B32Register::new(),
            voice6_cvol: B16Register::new(),
            voice6_raddr: B16Register::new(),
            voice7_voll: B16Register::new(),
            voice7_volr: B16Register::new(),
            voice7_srate: B16Register::new(),
            voice7_saddr: B16Register::new(),
            voice7_adsr: B32Register::new(),
            voice7_cvol: B16Register::new(),
            voice7_raddr: B16Register::new(),
            voice8_voll: B16Register::new(),
            voice8_volr: B16Register::new(),
            voice8_srate: B16Register::new(),
            voice8_saddr: B16Register::new(),
            voice8_adsr: B32Register::new(),
            voice8_cvol: B16Register::new(),
            voice8_raddr: B16Register::new(),
            voice9_voll: B16Register::new(),
            voice9_volr: B16Register::new(),
            voice9_srate: B16Register::new(),
            voice9_saddr: B16Register::new(),
            voice9_adsr: B32Register::new(),
            voice9_cvol: B16Register::new(),
            voice9_raddr: B16Register::new(),
            voice10_voll: B16Register::new(),
            voice10_volr: B16Register::new(),
            voice10_srate: B16Register::new(),
            voice10_saddr: B16Register::new(),
            voice10_adsr: B32Register::new(),
            voice10_cvol: B16Register::new(),
            voice10_raddr: B16Register::new(),
            voice11_voll: B16Register::new(),
            voice11_volr: B16Register::new(),
            voice11_srate: B16Register::new(),
            voice11_saddr: B16Register::new(),
            voice11_adsr: B32Register::new(),
            voice11_cvol: B16Register::new(),
            voice11_raddr: B16Register::new(),
            voice12_voll: B16Register::new(),
            voice12_volr: B16Register::new(),
            voice12_srate: B16Register::new(),
            voice12_saddr: B16Register::new(),
            voice12_adsr: B32Register::new(),
            voice12_cvol: B16Register::new(),
            voice12_raddr: B16Register::new(),
            voice13_voll: B16Register::new(),
            voice13_volr: B16Register::new(),
            voice13_srate: B16Register::new(),
            voice13_saddr: B16Register::new(),
            voice13_adsr: B32Register::new(),
            voice13_cvol: B16Register::new(),
            voice13_raddr: B16Register::new(),
            voice14_voll: B16Register::new(),
            voice14_volr: B16Register::new(),
            voice14_srate: B16Register::new(),
            voice14_saddr: B16Register::new(),
            voice14_adsr: B32Register::new(),
            voice14_cvol: B16Register::new(),
            voice14_raddr: B16Register::new(),
            voice15_voll: B16Register::new(),
            voice15_volr: B16Register::new(),
            voice15_srate: B16Register::new(),
            voice15_saddr: B16Register::new(),
            voice15_adsr: B32Register::new(),
            voice15_cvol: B16Register::new(),
            voice15_raddr: B16Register::new(),
            voice16_voll: B16Register::new(),
            voice16_volr: B16Register::new(),
            voice16_srate: B16Register::new(),
            voice16_saddr: B16Register::new(),
            voice16_adsr: B32Register::new(),
            voice16_cvol: B16Register::new(),
            voice16_raddr: B16Register::new(),
            voice17_voll: B16Register::new(),
            voice17_volr: B16Register::new(),
            voice17_srate: B16Register::new(),
            voice17_saddr: B16Register::new(),
            voice17_adsr: B32Register::new(),
            voice17_cvol: B16Register::new(),
            voice17_raddr: B16Register::new(),
            voice18_voll: B16Register::new(),
            voice18_volr: B16Register::new(),
            voice18_srate: B16Register::new(),
            voice18_saddr: B16Register::new(),
            voice18_adsr: B32Register::new(),
            voice18_cvol: B16Register::new(),
            voice18_raddr: B16Register::new(),
            voice19_voll: B16Register::new(),
            voice19_volr: B16Register::new(),
            voice19_srate: B16Register::new(),
            voice19_saddr: B16Register::new(),
            voice19_adsr: B32Register::new(),
            voice19_cvol: B16Register::new(),
            voice19_raddr: B16Register::new(),
            voice20_voll: B16Register::new(),
            voice20_volr: B16Register::new(),
            voice20_srate: B16Register::new(),
            voice20_saddr: B16Register::new(),
            voice20_adsr: B32Register::new(),
            voice20_cvol: B16Register::new(),
            voice20_raddr: B16Register::new(),
            voice21_voll: B16Register::new(),
            voice21_volr: B16Register::new(),
            voice21_srate: B16Register::new(),
            voice21_saddr: B16Register::new(),
            voice21_adsr: B32Register::new(),
            voice21_cvol: B16Register::new(),
            voice21_raddr: B16Register::new(),
            voice22_voll: B16Register::new(),
            voice22_volr: B16Register::new(),
            voice22_srate: B16Register::new(),
            voice22_saddr: B16Register::new(),
            voice22_adsr: B32Register::new(),
            voice22_cvol: B16Register::new(),
            voice22_raddr: B16Register::new(),
            voice23_voll: B16Register::new(),
            voice23_volr: B16Register::new(),
            voice23_srate: B16Register::new(),
            voice23_saddr: B16Register::new(),
            voice23_adsr: B32Register::new(),
            voice23_cvol: B16Register::new(),
            voice23_raddr: B16Register::new(),
            data_fifo: DataFifo::new(),
            memory: B8Memory::new(0x80_000),
            current_transfer_mode: TransferMode::Stop,
            current_transfer_address: 0,
            dac: DacState::new(),
        }
    }
}

pub fn initialize(state: &mut SystemState) {
    state.r3000.memory_mapper.map(0x1F80_1D80, 2, &mut state.spu.main_volume_left as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D82, 2, &mut state.spu.main_volume_right as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D84, 4, &mut state.spu.reverb_volume as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D88, 4, &mut state.spu.voice_key_on as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D8C, 4, &mut state.spu.voice_key_off as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D90, 4, &mut state.spu.voice_channel_fm as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D94, 4, &mut state.spu.voice_channel_noise as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D98, 4, &mut state.spu.voice_channel_reverb as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D9C, 4, &mut state.spu.voice_channel_status as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DA0, 2, &mut state.spu.unknown_0 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DA2, 2, &mut state.spu.reverb_start_address as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DA4, 2, &mut state.spu.irq_address as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DA6, 2, &mut state.spu.data_transfer_address as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DA8, 2, &mut state.spu.data_fifo as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DAA, 2, &mut state.spu.control as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DAC, 2, &mut state.spu.data_transfer_control as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DAE, 2, &mut state.spu.stat as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DB0, 4, &mut state.spu.cd_volume as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DB4, 4, &mut state.spu.extern_volume as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DB8, 2, &mut state.spu.current_volume_left as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DBA, 2, &mut state.spu.current_volume_right as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DBC, 4, &mut state.spu.unknown_1 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DC0, 2, &mut state.spu.dapf1 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DC2, 2, &mut state.spu.dapf2 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DC4, 2, &mut state.spu.viir as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DC6, 2, &mut state.spu.vcomb1 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DC8, 2, &mut state.spu.vcomb2 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DCA, 2, &mut state.spu.vcomb3 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DCC, 2, &mut state.spu.vcomb4 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DCE, 2, &mut state.spu.vwall as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DD0, 2, &mut state.spu.vapf1 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DD2, 2, &mut state.spu.vapf2 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DD4, 4, &mut state.spu.msame as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DD8, 4, &mut state.spu.mcomb1 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DDC, 4, &mut state.spu.mcomb2 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DE0, 4, &mut state.spu.dsame as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DE4, 4, &mut state.spu.mdiff as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DE8, 4, &mut state.spu.mcomb3 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DEC, 4, &mut state.spu.mcomb4 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DF0, 4, &mut state.spu.ddiff as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DF4, 4, &mut state.spu.mapf1 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DF8, 4, &mut state.spu.mapf2 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1DFC, 4, &mut state.spu.vin as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C00, 2, &mut state.spu.voice0_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C02, 2, &mut state.spu.voice0_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C04, 2, &mut state.spu.voice0_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C06, 2, &mut state.spu.voice0_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C08, 4, &mut state.spu.voice0_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C0C, 2, &mut state.spu.voice0_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C0E, 2, &mut state.spu.voice0_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C10, 2, &mut state.spu.voice1_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C12, 2, &mut state.spu.voice1_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C14, 2, &mut state.spu.voice1_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C16, 2, &mut state.spu.voice1_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C18, 4, &mut state.spu.voice1_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C1C, 2, &mut state.spu.voice1_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C1E, 2, &mut state.spu.voice1_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C20, 2, &mut state.spu.voice2_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C22, 2, &mut state.spu.voice2_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C24, 2, &mut state.spu.voice2_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C26, 2, &mut state.spu.voice2_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C28, 4, &mut state.spu.voice2_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C2C, 2, &mut state.spu.voice2_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C2E, 2, &mut state.spu.voice2_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C30, 2, &mut state.spu.voice3_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C32, 2, &mut state.spu.voice3_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C34, 2, &mut state.spu.voice3_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C36, 2, &mut state.spu.voice3_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C38, 4, &mut state.spu.voice3_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C3C, 2, &mut state.spu.voice3_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C3E, 2, &mut state.spu.voice3_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C40, 2, &mut state.spu.voice4_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C42, 2, &mut state.spu.voice4_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C44, 2, &mut state.spu.voice4_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C46, 2, &mut state.spu.voice4_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C48, 4, &mut state.spu.voice4_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C4C, 2, &mut state.spu.voice4_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C4E, 2, &mut state.spu.voice4_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C50, 2, &mut state.spu.voice5_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C52, 2, &mut state.spu.voice5_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C54, 2, &mut state.spu.voice5_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C56, 2, &mut state.spu.voice5_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C58, 4, &mut state.spu.voice5_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C5C, 2, &mut state.spu.voice5_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C5E, 2, &mut state.spu.voice5_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C60, 2, &mut state.spu.voice6_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C62, 2, &mut state.spu.voice6_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C64, 2, &mut state.spu.voice6_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C66, 2, &mut state.spu.voice6_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C68, 4, &mut state.spu.voice6_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C6C, 2, &mut state.spu.voice6_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C6E, 2, &mut state.spu.voice6_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C70, 2, &mut state.spu.voice7_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C72, 2, &mut state.spu.voice7_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C74, 2, &mut state.spu.voice7_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C76, 2, &mut state.spu.voice7_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C78, 4, &mut state.spu.voice7_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C7C, 2, &mut state.spu.voice7_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C7E, 2, &mut state.spu.voice7_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C80, 2, &mut state.spu.voice8_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C82, 2, &mut state.spu.voice8_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C84, 2, &mut state.spu.voice8_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C86, 2, &mut state.spu.voice8_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C88, 4, &mut state.spu.voice8_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C8C, 2, &mut state.spu.voice8_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C8E, 2, &mut state.spu.voice8_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C90, 2, &mut state.spu.voice9_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C92, 2, &mut state.spu.voice9_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C94, 2, &mut state.spu.voice9_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C96, 2, &mut state.spu.voice9_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C98, 4, &mut state.spu.voice9_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C9C, 2, &mut state.spu.voice9_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1C9E, 2, &mut state.spu.voice9_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CA0, 2, &mut state.spu.voice10_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CA2, 2, &mut state.spu.voice10_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CA4, 2, &mut state.spu.voice10_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CA6, 2, &mut state.spu.voice10_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CA8, 4, &mut state.spu.voice10_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CAC, 2, &mut state.spu.voice10_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CAE, 2, &mut state.spu.voice10_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CB0, 2, &mut state.spu.voice11_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CB2, 2, &mut state.spu.voice11_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CB4, 2, &mut state.spu.voice11_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CB6, 2, &mut state.spu.voice11_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CB8, 4, &mut state.spu.voice11_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CBC, 2, &mut state.spu.voice11_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CBE, 2, &mut state.spu.voice11_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CC0, 2, &mut state.spu.voice12_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CC2, 2, &mut state.spu.voice12_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CC4, 2, &mut state.spu.voice12_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CC6, 2, &mut state.spu.voice12_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CC8, 4, &mut state.spu.voice12_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CCC, 2, &mut state.spu.voice12_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CCE, 2, &mut state.spu.voice12_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CD0, 2, &mut state.spu.voice13_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CD2, 2, &mut state.spu.voice13_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CD4, 2, &mut state.spu.voice13_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CD6, 2, &mut state.spu.voice13_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CD8, 4, &mut state.spu.voice13_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CDC, 2, &mut state.spu.voice13_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CDE, 2, &mut state.spu.voice13_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CE0, 2, &mut state.spu.voice14_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CE2, 2, &mut state.spu.voice14_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CE4, 2, &mut state.spu.voice14_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CE6, 2, &mut state.spu.voice14_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CE8, 4, &mut state.spu.voice14_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CEC, 2, &mut state.spu.voice14_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CEE, 2, &mut state.spu.voice14_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CF0, 2, &mut state.spu.voice15_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CF2, 2, &mut state.spu.voice15_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CF4, 2, &mut state.spu.voice15_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CF6, 2, &mut state.spu.voice15_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CF8, 4, &mut state.spu.voice15_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CFC, 2, &mut state.spu.voice15_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1CFE, 2, &mut state.spu.voice15_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D00, 2, &mut state.spu.voice16_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D02, 2, &mut state.spu.voice16_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D04, 2, &mut state.spu.voice16_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D06, 2, &mut state.spu.voice16_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D08, 4, &mut state.spu.voice16_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D0C, 2, &mut state.spu.voice16_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D0E, 2, &mut state.spu.voice16_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D10, 2, &mut state.spu.voice17_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D12, 2, &mut state.spu.voice17_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D14, 2, &mut state.spu.voice17_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D16, 2, &mut state.spu.voice17_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D18, 4, &mut state.spu.voice17_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D1C, 2, &mut state.spu.voice17_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D1E, 2, &mut state.spu.voice17_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D20, 2, &mut state.spu.voice18_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D22, 2, &mut state.spu.voice18_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D24, 2, &mut state.spu.voice18_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D26, 2, &mut state.spu.voice18_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D28, 4, &mut state.spu.voice18_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D2C, 2, &mut state.spu.voice18_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D2E, 2, &mut state.spu.voice18_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D30, 2, &mut state.spu.voice19_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D32, 2, &mut state.spu.voice19_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D34, 2, &mut state.spu.voice19_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D36, 2, &mut state.spu.voice19_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D38, 4, &mut state.spu.voice19_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D3C, 2, &mut state.spu.voice19_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D3E, 2, &mut state.spu.voice19_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D40, 2, &mut state.spu.voice20_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D42, 2, &mut state.spu.voice20_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D44, 2, &mut state.spu.voice20_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D46, 2, &mut state.spu.voice20_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D48, 4, &mut state.spu.voice20_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D4C, 2, &mut state.spu.voice20_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D4E, 2, &mut state.spu.voice20_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D50, 2, &mut state.spu.voice21_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D52, 2, &mut state.spu.voice21_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D54, 2, &mut state.spu.voice21_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D56, 2, &mut state.spu.voice21_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D58, 4, &mut state.spu.voice21_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D5C, 2, &mut state.spu.voice21_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D5E, 2, &mut state.spu.voice21_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D60, 2, &mut state.spu.voice22_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D62, 2, &mut state.spu.voice22_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D64, 2, &mut state.spu.voice22_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D66, 2, &mut state.spu.voice22_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D68, 4, &mut state.spu.voice22_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D6C, 2, &mut state.spu.voice22_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D6E, 2, &mut state.spu.voice22_raddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D70, 2, &mut state.spu.voice23_voll as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D72, 2, &mut state.spu.voice23_volr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D74, 2, &mut state.spu.voice23_srate as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D76, 2, &mut state.spu.voice23_saddr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D78, 4, &mut state.spu.voice23_adsr as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D7C, 2, &mut state.spu.voice23_cvol as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1D7E, 2, &mut state.spu.voice23_raddr as *mut dyn B8MemoryMap);
}
