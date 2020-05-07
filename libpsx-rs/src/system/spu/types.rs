use crate::types::{
    bitfield::Bitfield,
    fifo::{
        debug::DebugState,
        Fifo,
    },
    memory::*,
    stereo::*,
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

pub struct DataTransferAddress {
    pub register: B16LevelRegister,
    pub write_latch: AtomicBool,
}

impl DataTransferAddress {
    pub fn new() -> DataTransferAddress {
        DataTransferAddress {
            register: B16LevelRegister::new(),
            write_latch: AtomicBool::new(false),
        }
    }

    pub fn read_u16(&self) -> u16 {
        self.register.read_u16()
    }

    pub fn write_u16(&self, value: u16) {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        self.register.write_u16(value)
    }
}

pub struct VoiceKey {
    pub register: B32LevelRegister,
    pub write_latch: Mutex<[bool; 32]>,
}

impl VoiceKey {
    pub fn new() -> VoiceKey {
        VoiceKey {
            register: B32LevelRegister::new(),
            write_latch: Mutex::new([false; 32]),
        }
    }

    pub fn read_u16(&self, offset: u32) -> u16 {
        self.register.read_u16(offset)
    }

    pub fn write_u16(&self, offset: u32, value: u16) {
        let write_latch = &mut self.write_latch.lock();
        self.register.write_u16(offset, value);
        for i in 0..16 {
            let write_latch_offset = ((offset * 8) + (i as u32)) as usize;
            write_latch[write_latch_offset] = Bitfield::new(i, 1).extract_from(value) != 0;
        }
    }

    pub fn read_u32(&self) -> u32 {
        self.register.read_u32()
    }

    pub fn write_u32(&self, value: u32) {
        let write_latch = &mut self.write_latch.lock();
        self.register.write_u32(value);
        for i in 0..32 {
            write_latch[i] = Bitfield::new(i, 1).extract_from(value) != 0;
        }
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

pub struct ControllerState {
    pub memory: Vec<u8>,
    pub current_transfer_mode: TransferMode,
    pub current_transfer_address: u32,
    pub dac: DacState,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
            memory: vec![0; 0x8_0000],
            current_transfer_mode: TransferMode::Stop,
            current_transfer_address: 0,
            dac: DacState::new(),
        }
    }
}

pub struct State {
    pub main_volume_left: B16LevelRegister,
    pub main_volume_right: B16LevelRegister,
    pub reverb_volume: B32LevelRegister,

    pub voice_key_on: VoiceKey,
    pub voice_key_off: VoiceKey,
    pub voice_channel_fm: B32LevelRegister,
    pub voice_channel_noise: B32LevelRegister,
    pub voice_channel_reverb: B32LevelRegister,
    pub voice_channel_status: B32LevelRegister,

    pub unknown_0: B16LevelRegister,
    pub reverb_start_address: B16LevelRegister,
    pub irq_address: B16LevelRegister,
    pub data_transfer_address: DataTransferAddress,
    pub control: B16LevelRegister,
    pub data_transfer_control: B16LevelRegister,
    pub stat: B16LevelRegister,
    pub cd_volume: B32LevelRegister,
    pub extern_volume: B32LevelRegister,
    pub current_volume_left: B16LevelRegister,
    pub current_volume_right: B16LevelRegister,
    pub unknown_1: B32LevelRegister,

    pub dapf1: B16LevelRegister,
    pub dapf2: B16LevelRegister,
    pub viir: B16LevelRegister,
    pub vcomb1: B16LevelRegister,
    pub vcomb2: B16LevelRegister,
    pub vcomb3: B16LevelRegister,
    pub vcomb4: B16LevelRegister,
    pub vwall: B16LevelRegister,
    pub vapf1: B16LevelRegister,
    pub vapf2: B16LevelRegister,
    pub msame: B32LevelRegister,
    pub mcomb1: B32LevelRegister,
    pub mcomb2: B32LevelRegister,
    pub dsame: B32LevelRegister,
    pub mdiff: B32LevelRegister,
    pub mcomb3: B32LevelRegister,
    pub mcomb4: B32LevelRegister,
    pub ddiff: B32LevelRegister,
    pub mapf1: B32LevelRegister,
    pub mapf2: B32LevelRegister,
    pub vin: B32LevelRegister,

    pub voice0_voll: B16LevelRegister,
    pub voice0_volr: B16LevelRegister,
    pub voice0_srate: B16LevelRegister,
    pub voice0_saddr: B16LevelRegister,
    pub voice0_adsr: B32LevelRegister,
    pub voice0_cvol: B16LevelRegister,
    pub voice0_raddr: B16LevelRegister,

    pub voice1_voll: B16LevelRegister,
    pub voice1_volr: B16LevelRegister,
    pub voice1_srate: B16LevelRegister,
    pub voice1_saddr: B16LevelRegister,
    pub voice1_adsr: B32LevelRegister,
    pub voice1_cvol: B16LevelRegister,
    pub voice1_raddr: B16LevelRegister,

    pub voice2_voll: B16LevelRegister,
    pub voice2_volr: B16LevelRegister,
    pub voice2_srate: B16LevelRegister,
    pub voice2_saddr: B16LevelRegister,
    pub voice2_adsr: B32LevelRegister,
    pub voice2_cvol: B16LevelRegister,
    pub voice2_raddr: B16LevelRegister,

    pub voice3_voll: B16LevelRegister,
    pub voice3_volr: B16LevelRegister,
    pub voice3_srate: B16LevelRegister,
    pub voice3_saddr: B16LevelRegister,
    pub voice3_adsr: B32LevelRegister,
    pub voice3_cvol: B16LevelRegister,
    pub voice3_raddr: B16LevelRegister,

    pub voice4_voll: B16LevelRegister,
    pub voice4_volr: B16LevelRegister,
    pub voice4_srate: B16LevelRegister,
    pub voice4_saddr: B16LevelRegister,
    pub voice4_adsr: B32LevelRegister,
    pub voice4_cvol: B16LevelRegister,
    pub voice4_raddr: B16LevelRegister,

    pub voice5_voll: B16LevelRegister,
    pub voice5_volr: B16LevelRegister,
    pub voice5_srate: B16LevelRegister,
    pub voice5_saddr: B16LevelRegister,
    pub voice5_adsr: B32LevelRegister,
    pub voice5_cvol: B16LevelRegister,
    pub voice5_raddr: B16LevelRegister,

    pub voice6_voll: B16LevelRegister,
    pub voice6_volr: B16LevelRegister,
    pub voice6_srate: B16LevelRegister,
    pub voice6_saddr: B16LevelRegister,
    pub voice6_adsr: B32LevelRegister,
    pub voice6_cvol: B16LevelRegister,
    pub voice6_raddr: B16LevelRegister,

    pub voice7_voll: B16LevelRegister,
    pub voice7_volr: B16LevelRegister,
    pub voice7_srate: B16LevelRegister,
    pub voice7_saddr: B16LevelRegister,
    pub voice7_adsr: B32LevelRegister,
    pub voice7_cvol: B16LevelRegister,
    pub voice7_raddr: B16LevelRegister,

    pub voice8_voll: B16LevelRegister,
    pub voice8_volr: B16LevelRegister,
    pub voice8_srate: B16LevelRegister,
    pub voice8_saddr: B16LevelRegister,
    pub voice8_adsr: B32LevelRegister,
    pub voice8_cvol: B16LevelRegister,
    pub voice8_raddr: B16LevelRegister,

    pub voice9_voll: B16LevelRegister,
    pub voice9_volr: B16LevelRegister,
    pub voice9_srate: B16LevelRegister,
    pub voice9_saddr: B16LevelRegister,
    pub voice9_adsr: B32LevelRegister,
    pub voice9_cvol: B16LevelRegister,
    pub voice9_raddr: B16LevelRegister,

    pub voice10_voll: B16LevelRegister,
    pub voice10_volr: B16LevelRegister,
    pub voice10_srate: B16LevelRegister,
    pub voice10_saddr: B16LevelRegister,
    pub voice10_adsr: B32LevelRegister,
    pub voice10_cvol: B16LevelRegister,
    pub voice10_raddr: B16LevelRegister,

    pub voice11_voll: B16LevelRegister,
    pub voice11_volr: B16LevelRegister,
    pub voice11_srate: B16LevelRegister,
    pub voice11_saddr: B16LevelRegister,
    pub voice11_adsr: B32LevelRegister,
    pub voice11_cvol: B16LevelRegister,
    pub voice11_raddr: B16LevelRegister,

    pub voice12_voll: B16LevelRegister,
    pub voice12_volr: B16LevelRegister,
    pub voice12_srate: B16LevelRegister,
    pub voice12_saddr: B16LevelRegister,
    pub voice12_adsr: B32LevelRegister,
    pub voice12_cvol: B16LevelRegister,
    pub voice12_raddr: B16LevelRegister,

    pub voice13_voll: B16LevelRegister,
    pub voice13_volr: B16LevelRegister,
    pub voice13_srate: B16LevelRegister,
    pub voice13_saddr: B16LevelRegister,
    pub voice13_adsr: B32LevelRegister,
    pub voice13_cvol: B16LevelRegister,
    pub voice13_raddr: B16LevelRegister,

    pub voice14_voll: B16LevelRegister,
    pub voice14_volr: B16LevelRegister,
    pub voice14_srate: B16LevelRegister,
    pub voice14_saddr: B16LevelRegister,
    pub voice14_adsr: B32LevelRegister,
    pub voice14_cvol: B16LevelRegister,
    pub voice14_raddr: B16LevelRegister,

    pub voice15_voll: B16LevelRegister,
    pub voice15_volr: B16LevelRegister,
    pub voice15_srate: B16LevelRegister,
    pub voice15_saddr: B16LevelRegister,
    pub voice15_adsr: B32LevelRegister,
    pub voice15_cvol: B16LevelRegister,
    pub voice15_raddr: B16LevelRegister,

    pub voice16_voll: B16LevelRegister,
    pub voice16_volr: B16LevelRegister,
    pub voice16_srate: B16LevelRegister,
    pub voice16_saddr: B16LevelRegister,
    pub voice16_adsr: B32LevelRegister,
    pub voice16_cvol: B16LevelRegister,
    pub voice16_raddr: B16LevelRegister,

    pub voice17_voll: B16LevelRegister,
    pub voice17_volr: B16LevelRegister,
    pub voice17_srate: B16LevelRegister,
    pub voice17_saddr: B16LevelRegister,
    pub voice17_adsr: B32LevelRegister,
    pub voice17_cvol: B16LevelRegister,
    pub voice17_raddr: B16LevelRegister,

    pub voice18_voll: B16LevelRegister,
    pub voice18_volr: B16LevelRegister,
    pub voice18_srate: B16LevelRegister,
    pub voice18_saddr: B16LevelRegister,
    pub voice18_adsr: B32LevelRegister,
    pub voice18_cvol: B16LevelRegister,
    pub voice18_raddr: B16LevelRegister,

    pub voice19_voll: B16LevelRegister,
    pub voice19_volr: B16LevelRegister,
    pub voice19_srate: B16LevelRegister,
    pub voice19_saddr: B16LevelRegister,
    pub voice19_adsr: B32LevelRegister,
    pub voice19_cvol: B16LevelRegister,
    pub voice19_raddr: B16LevelRegister,

    pub voice20_voll: B16LevelRegister,
    pub voice20_volr: B16LevelRegister,
    pub voice20_srate: B16LevelRegister,
    pub voice20_saddr: B16LevelRegister,
    pub voice20_adsr: B32LevelRegister,
    pub voice20_cvol: B16LevelRegister,
    pub voice20_raddr: B16LevelRegister,

    pub voice21_voll: B16LevelRegister,
    pub voice21_volr: B16LevelRegister,
    pub voice21_srate: B16LevelRegister,
    pub voice21_saddr: B16LevelRegister,
    pub voice21_adsr: B32LevelRegister,
    pub voice21_cvol: B16LevelRegister,
    pub voice21_raddr: B16LevelRegister,

    pub voice22_voll: B16LevelRegister,
    pub voice22_volr: B16LevelRegister,
    pub voice22_srate: B16LevelRegister,
    pub voice22_saddr: B16LevelRegister,
    pub voice22_adsr: B32LevelRegister,
    pub voice22_cvol: B16LevelRegister,
    pub voice22_raddr: B16LevelRegister,

    pub voice23_voll: B16LevelRegister,
    pub voice23_volr: B16LevelRegister,
    pub voice23_srate: B16LevelRegister,
    pub voice23_saddr: B16LevelRegister,
    pub voice23_adsr: B32LevelRegister,
    pub voice23_cvol: B16LevelRegister,
    pub voice23_raddr: B16LevelRegister,

    pub data_fifo: Fifo<u16>,

    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            main_volume_left: B16LevelRegister::new(),
            main_volume_right: B16LevelRegister::new(),
            reverb_volume: B32LevelRegister::new(),
            voice_key_on: VoiceKey::new(),
            voice_key_off: VoiceKey::new(),
            voice_channel_fm: B32LevelRegister::new(),
            voice_channel_noise: B32LevelRegister::new(),
            voice_channel_reverb: B32LevelRegister::new(),
            voice_channel_status: B32LevelRegister::new(),
            unknown_0: B16LevelRegister::new(),
            reverb_start_address: B16LevelRegister::new(),
            irq_address: B16LevelRegister::new(),
            data_transfer_address: DataTransferAddress::new(),
            control: B16LevelRegister::new(),
            data_transfer_control: B16LevelRegister::new(),
            stat: B16LevelRegister::new(),
            cd_volume: B32LevelRegister::new(),
            extern_volume: B32LevelRegister::new(),
            current_volume_left: B16LevelRegister::new(),
            current_volume_right: B16LevelRegister::new(),
            unknown_1: B32LevelRegister::new(),
            dapf1: B16LevelRegister::new(),
            dapf2: B16LevelRegister::new(),
            viir: B16LevelRegister::new(),
            vcomb1: B16LevelRegister::new(),
            vcomb2: B16LevelRegister::new(),
            vcomb3: B16LevelRegister::new(),
            vcomb4: B16LevelRegister::new(),
            vwall: B16LevelRegister::new(),
            vapf1: B16LevelRegister::new(),
            vapf2: B16LevelRegister::new(),
            msame: B32LevelRegister::new(),
            mcomb1: B32LevelRegister::new(),
            mcomb2: B32LevelRegister::new(),
            dsame: B32LevelRegister::new(),
            mdiff: B32LevelRegister::new(),
            mcomb3: B32LevelRegister::new(),
            mcomb4: B32LevelRegister::new(),
            ddiff: B32LevelRegister::new(),
            mapf1: B32LevelRegister::new(),
            mapf2: B32LevelRegister::new(),
            vin: B32LevelRegister::new(),
            voice0_voll: B16LevelRegister::new(),
            voice0_volr: B16LevelRegister::new(),
            voice0_srate: B16LevelRegister::new(),
            voice0_saddr: B16LevelRegister::new(),
            voice0_adsr: B32LevelRegister::new(),
            voice0_cvol: B16LevelRegister::new(),
            voice0_raddr: B16LevelRegister::new(),
            voice1_voll: B16LevelRegister::new(),
            voice1_volr: B16LevelRegister::new(),
            voice1_srate: B16LevelRegister::new(),
            voice1_saddr: B16LevelRegister::new(),
            voice1_adsr: B32LevelRegister::new(),
            voice1_cvol: B16LevelRegister::new(),
            voice1_raddr: B16LevelRegister::new(),
            voice2_voll: B16LevelRegister::new(),
            voice2_volr: B16LevelRegister::new(),
            voice2_srate: B16LevelRegister::new(),
            voice2_saddr: B16LevelRegister::new(),
            voice2_adsr: B32LevelRegister::new(),
            voice2_cvol: B16LevelRegister::new(),
            voice2_raddr: B16LevelRegister::new(),
            voice3_voll: B16LevelRegister::new(),
            voice3_volr: B16LevelRegister::new(),
            voice3_srate: B16LevelRegister::new(),
            voice3_saddr: B16LevelRegister::new(),
            voice3_adsr: B32LevelRegister::new(),
            voice3_cvol: B16LevelRegister::new(),
            voice3_raddr: B16LevelRegister::new(),
            voice4_voll: B16LevelRegister::new(),
            voice4_volr: B16LevelRegister::new(),
            voice4_srate: B16LevelRegister::new(),
            voice4_saddr: B16LevelRegister::new(),
            voice4_adsr: B32LevelRegister::new(),
            voice4_cvol: B16LevelRegister::new(),
            voice4_raddr: B16LevelRegister::new(),
            voice5_voll: B16LevelRegister::new(),
            voice5_volr: B16LevelRegister::new(),
            voice5_srate: B16LevelRegister::new(),
            voice5_saddr: B16LevelRegister::new(),
            voice5_adsr: B32LevelRegister::new(),
            voice5_cvol: B16LevelRegister::new(),
            voice5_raddr: B16LevelRegister::new(),
            voice6_voll: B16LevelRegister::new(),
            voice6_volr: B16LevelRegister::new(),
            voice6_srate: B16LevelRegister::new(),
            voice6_saddr: B16LevelRegister::new(),
            voice6_adsr: B32LevelRegister::new(),
            voice6_cvol: B16LevelRegister::new(),
            voice6_raddr: B16LevelRegister::new(),
            voice7_voll: B16LevelRegister::new(),
            voice7_volr: B16LevelRegister::new(),
            voice7_srate: B16LevelRegister::new(),
            voice7_saddr: B16LevelRegister::new(),
            voice7_adsr: B32LevelRegister::new(),
            voice7_cvol: B16LevelRegister::new(),
            voice7_raddr: B16LevelRegister::new(),
            voice8_voll: B16LevelRegister::new(),
            voice8_volr: B16LevelRegister::new(),
            voice8_srate: B16LevelRegister::new(),
            voice8_saddr: B16LevelRegister::new(),
            voice8_adsr: B32LevelRegister::new(),
            voice8_cvol: B16LevelRegister::new(),
            voice8_raddr: B16LevelRegister::new(),
            voice9_voll: B16LevelRegister::new(),
            voice9_volr: B16LevelRegister::new(),
            voice9_srate: B16LevelRegister::new(),
            voice9_saddr: B16LevelRegister::new(),
            voice9_adsr: B32LevelRegister::new(),
            voice9_cvol: B16LevelRegister::new(),
            voice9_raddr: B16LevelRegister::new(),
            voice10_voll: B16LevelRegister::new(),
            voice10_volr: B16LevelRegister::new(),
            voice10_srate: B16LevelRegister::new(),
            voice10_saddr: B16LevelRegister::new(),
            voice10_adsr: B32LevelRegister::new(),
            voice10_cvol: B16LevelRegister::new(),
            voice10_raddr: B16LevelRegister::new(),
            voice11_voll: B16LevelRegister::new(),
            voice11_volr: B16LevelRegister::new(),
            voice11_srate: B16LevelRegister::new(),
            voice11_saddr: B16LevelRegister::new(),
            voice11_adsr: B32LevelRegister::new(),
            voice11_cvol: B16LevelRegister::new(),
            voice11_raddr: B16LevelRegister::new(),
            voice12_voll: B16LevelRegister::new(),
            voice12_volr: B16LevelRegister::new(),
            voice12_srate: B16LevelRegister::new(),
            voice12_saddr: B16LevelRegister::new(),
            voice12_adsr: B32LevelRegister::new(),
            voice12_cvol: B16LevelRegister::new(),
            voice12_raddr: B16LevelRegister::new(),
            voice13_voll: B16LevelRegister::new(),
            voice13_volr: B16LevelRegister::new(),
            voice13_srate: B16LevelRegister::new(),
            voice13_saddr: B16LevelRegister::new(),
            voice13_adsr: B32LevelRegister::new(),
            voice13_cvol: B16LevelRegister::new(),
            voice13_raddr: B16LevelRegister::new(),
            voice14_voll: B16LevelRegister::new(),
            voice14_volr: B16LevelRegister::new(),
            voice14_srate: B16LevelRegister::new(),
            voice14_saddr: B16LevelRegister::new(),
            voice14_adsr: B32LevelRegister::new(),
            voice14_cvol: B16LevelRegister::new(),
            voice14_raddr: B16LevelRegister::new(),
            voice15_voll: B16LevelRegister::new(),
            voice15_volr: B16LevelRegister::new(),
            voice15_srate: B16LevelRegister::new(),
            voice15_saddr: B16LevelRegister::new(),
            voice15_adsr: B32LevelRegister::new(),
            voice15_cvol: B16LevelRegister::new(),
            voice15_raddr: B16LevelRegister::new(),
            voice16_voll: B16LevelRegister::new(),
            voice16_volr: B16LevelRegister::new(),
            voice16_srate: B16LevelRegister::new(),
            voice16_saddr: B16LevelRegister::new(),
            voice16_adsr: B32LevelRegister::new(),
            voice16_cvol: B16LevelRegister::new(),
            voice16_raddr: B16LevelRegister::new(),
            voice17_voll: B16LevelRegister::new(),
            voice17_volr: B16LevelRegister::new(),
            voice17_srate: B16LevelRegister::new(),
            voice17_saddr: B16LevelRegister::new(),
            voice17_adsr: B32LevelRegister::new(),
            voice17_cvol: B16LevelRegister::new(),
            voice17_raddr: B16LevelRegister::new(),
            voice18_voll: B16LevelRegister::new(),
            voice18_volr: B16LevelRegister::new(),
            voice18_srate: B16LevelRegister::new(),
            voice18_saddr: B16LevelRegister::new(),
            voice18_adsr: B32LevelRegister::new(),
            voice18_cvol: B16LevelRegister::new(),
            voice18_raddr: B16LevelRegister::new(),
            voice19_voll: B16LevelRegister::new(),
            voice19_volr: B16LevelRegister::new(),
            voice19_srate: B16LevelRegister::new(),
            voice19_saddr: B16LevelRegister::new(),
            voice19_adsr: B32LevelRegister::new(),
            voice19_cvol: B16LevelRegister::new(),
            voice19_raddr: B16LevelRegister::new(),
            voice20_voll: B16LevelRegister::new(),
            voice20_volr: B16LevelRegister::new(),
            voice20_srate: B16LevelRegister::new(),
            voice20_saddr: B16LevelRegister::new(),
            voice20_adsr: B32LevelRegister::new(),
            voice20_cvol: B16LevelRegister::new(),
            voice20_raddr: B16LevelRegister::new(),
            voice21_voll: B16LevelRegister::new(),
            voice21_volr: B16LevelRegister::new(),
            voice21_srate: B16LevelRegister::new(),
            voice21_saddr: B16LevelRegister::new(),
            voice21_adsr: B32LevelRegister::new(),
            voice21_cvol: B16LevelRegister::new(),
            voice21_raddr: B16LevelRegister::new(),
            voice22_voll: B16LevelRegister::new(),
            voice22_volr: B16LevelRegister::new(),
            voice22_srate: B16LevelRegister::new(),
            voice22_saddr: B16LevelRegister::new(),
            voice22_adsr: B32LevelRegister::new(),
            voice22_cvol: B16LevelRegister::new(),
            voice22_raddr: B16LevelRegister::new(),
            voice23_voll: B16LevelRegister::new(),
            voice23_volr: B16LevelRegister::new(),
            voice23_srate: B16LevelRegister::new(),
            voice23_saddr: B16LevelRegister::new(),
            voice23_adsr: B32LevelRegister::new(),
            voice23_cvol: B16LevelRegister::new(),
            voice23_raddr: B16LevelRegister::new(),
            data_fifo: Fifo::new(64, Some(DebugState::new("SPU FIFO", false, false))),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
