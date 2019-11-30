pub mod register;
pub mod voice;
pub mod dac;

use crate::types::register::b32_register::B32Register;
use crate::types::register::b16_register::B16Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::memory::b8_memory::B8Memory;
use crate::types::bitfield::Bitfield;
use crate::resources::Resources;
use crate::resources::spu::register::*;
use crate::resources::spu::dac::*;

pub const _CONTROL_CD_AUDIO_ENABLE: Bitfield = Bitfield::new(0, 1);
pub const _CONTROL_EXTERNAL_AUDIO_ENABLE: Bitfield = Bitfield::new(1, 1);
pub const _CONTROL_CD_AUDIO_REVERB: Bitfield = Bitfield::new(2, 1);
pub const _CONTROL_EXTERNAL_AUDIO_REVERB: Bitfield = Bitfield::new(3, 1);
pub const CONTROL_TRANSFER_MODE: Bitfield = Bitfield::new(4, 2);
pub const CONTROL_UNMUTE: Bitfield = Bitfield::new(14, 1);
pub const CONTROL_ENABLE: Bitfield = Bitfield::new(15, 1);

pub const _STAT_CD_AUDIO_ENABLE: Bitfield = Bitfield::new(0, 1);
pub const _STAT_EXTERNAL_AUDIO_ENABLE: Bitfield = Bitfield::new(1, 1);
pub const _STAT_CD_AUDIO_REVERB: Bitfield = Bitfield::new(2, 1);
pub const _STAT_EXTERNAL_AUDIO_REVERB: Bitfield = Bitfield::new(3, 1);
pub const STAT_TRANSFER_MODE: Bitfield = Bitfield::new(4, 2);
pub const _STAT_IRQ_FLAG: Bitfield = Bitfield::new(6, 1);
pub const _STAT_DMA_RW_REQUEST: Bitfield = Bitfield::new(7, 1);
pub const _STAT_DMA_W_REQUEST: Bitfield = Bitfield::new(8, 1);
pub const _STAT_DMA_R_REQUEST: Bitfield = Bitfield::new(9, 1);
pub const STAT_DATA_BUSY_FLAG: Bitfield = Bitfield::new(10, 1);
pub const _STAT_WRITING_BUFFER_HALF: Bitfield = Bitfield::new(11, 1);

pub struct Spu {
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

    pub dac: Dac,
}

impl Spu {
    pub fn new() -> Spu {
        Spu {
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
            dac: Dac::new(),
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.r3000.memory_mapper.map(0x1F80_1D80, 2, &mut resources.spu.main_volume_left as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D82, 2, &mut resources.spu.main_volume_right as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D84, 4, &mut resources.spu.reverb_volume as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D88, 4, &mut resources.spu.voice_key_on as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D8C, 4, &mut resources.spu.voice_key_off as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D90, 4, &mut resources.spu.voice_channel_fm as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D94, 4, &mut resources.spu.voice_channel_noise as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D98, 4, &mut resources.spu.voice_channel_reverb as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D9C, 4, &mut resources.spu.voice_channel_status as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DA0, 2, &mut resources.spu.unknown_0 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DA2, 2, &mut resources.spu.reverb_start_address as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DA4, 2, &mut resources.spu.irq_address as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DA6, 2, &mut resources.spu.data_transfer_address as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DA8, 2, &mut resources.spu.data_fifo as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DAA, 2, &mut resources.spu.control as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DAC, 2, &mut resources.spu.data_transfer_control as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DAE, 2, &mut resources.spu.stat as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DB0, 4, &mut resources.spu.cd_volume as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DB4, 4, &mut resources.spu.extern_volume as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DB8, 2, &mut resources.spu.current_volume_left as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DBA, 2, &mut resources.spu.current_volume_right as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DBC, 4, &mut resources.spu.unknown_1 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DC0, 2, &mut resources.spu.dapf1 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DC2, 2, &mut resources.spu.dapf2 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DC4, 2, &mut resources.spu.viir as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DC6, 2, &mut resources.spu.vcomb1 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DC8, 2, &mut resources.spu.vcomb2 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DCA, 2, &mut resources.spu.vcomb3 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DCC, 2, &mut resources.spu.vcomb4 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DCE, 2, &mut resources.spu.vwall as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DD0, 2, &mut resources.spu.vapf1 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DD2, 2, &mut resources.spu.vapf2 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DD4, 4, &mut resources.spu.msame as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DD8, 4, &mut resources.spu.mcomb1 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DDC, 4, &mut resources.spu.mcomb2 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DE0, 4, &mut resources.spu.dsame as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DE4, 4, &mut resources.spu.mdiff as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DE8, 4, &mut resources.spu.mcomb3 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DEC, 4, &mut resources.spu.mcomb4 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DF0, 4, &mut resources.spu.ddiff as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DF4, 4, &mut resources.spu.mapf1 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DF8, 4, &mut resources.spu.mapf2 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1DFC, 4, &mut resources.spu.vin as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C00, 2, &mut resources.spu.voice0_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C02, 2, &mut resources.spu.voice0_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C04, 2, &mut resources.spu.voice0_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C06, 2, &mut resources.spu.voice0_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C08, 4, &mut resources.spu.voice0_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C0C, 2, &mut resources.spu.voice0_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C0E, 2, &mut resources.spu.voice0_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C10, 2, &mut resources.spu.voice1_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C12, 2, &mut resources.spu.voice1_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C14, 2, &mut resources.spu.voice1_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C16, 2, &mut resources.spu.voice1_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C18, 4, &mut resources.spu.voice1_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C1C, 2, &mut resources.spu.voice1_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C1E, 2, &mut resources.spu.voice1_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C20, 2, &mut resources.spu.voice2_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C22, 2, &mut resources.spu.voice2_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C24, 2, &mut resources.spu.voice2_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C26, 2, &mut resources.spu.voice2_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C28, 4, &mut resources.spu.voice2_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C2C, 2, &mut resources.spu.voice2_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C2E, 2, &mut resources.spu.voice2_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C30, 2, &mut resources.spu.voice3_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C32, 2, &mut resources.spu.voice3_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C34, 2, &mut resources.spu.voice3_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C36, 2, &mut resources.spu.voice3_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C38, 4, &mut resources.spu.voice3_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C3C, 2, &mut resources.spu.voice3_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C3E, 2, &mut resources.spu.voice3_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C40, 2, &mut resources.spu.voice4_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C42, 2, &mut resources.spu.voice4_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C44, 2, &mut resources.spu.voice4_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C46, 2, &mut resources.spu.voice4_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C48, 4, &mut resources.spu.voice4_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C4C, 2, &mut resources.spu.voice4_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C4E, 2, &mut resources.spu.voice4_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C50, 2, &mut resources.spu.voice5_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C52, 2, &mut resources.spu.voice5_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C54, 2, &mut resources.spu.voice5_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C56, 2, &mut resources.spu.voice5_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C58, 4, &mut resources.spu.voice5_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C5C, 2, &mut resources.spu.voice5_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C5E, 2, &mut resources.spu.voice5_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C60, 2, &mut resources.spu.voice6_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C62, 2, &mut resources.spu.voice6_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C64, 2, &mut resources.spu.voice6_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C66, 2, &mut resources.spu.voice6_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C68, 4, &mut resources.spu.voice6_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C6C, 2, &mut resources.spu.voice6_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C6E, 2, &mut resources.spu.voice6_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C70, 2, &mut resources.spu.voice7_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C72, 2, &mut resources.spu.voice7_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C74, 2, &mut resources.spu.voice7_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C76, 2, &mut resources.spu.voice7_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C78, 4, &mut resources.spu.voice7_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C7C, 2, &mut resources.spu.voice7_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C7E, 2, &mut resources.spu.voice7_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C80, 2, &mut resources.spu.voice8_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C82, 2, &mut resources.spu.voice8_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C84, 2, &mut resources.spu.voice8_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C86, 2, &mut resources.spu.voice8_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C88, 4, &mut resources.spu.voice8_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C8C, 2, &mut resources.spu.voice8_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C8E, 2, &mut resources.spu.voice8_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C90, 2, &mut resources.spu.voice9_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C92, 2, &mut resources.spu.voice9_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C94, 2, &mut resources.spu.voice9_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C96, 2, &mut resources.spu.voice9_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C98, 4, &mut resources.spu.voice9_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C9C, 2, &mut resources.spu.voice9_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1C9E, 2, &mut resources.spu.voice9_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CA0, 2, &mut resources.spu.voice10_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CA2, 2, &mut resources.spu.voice10_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CA4, 2, &mut resources.spu.voice10_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CA6, 2, &mut resources.spu.voice10_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CA8, 4, &mut resources.spu.voice10_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CAC, 2, &mut resources.spu.voice10_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CAE, 2, &mut resources.spu.voice10_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CB0, 2, &mut resources.spu.voice11_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CB2, 2, &mut resources.spu.voice11_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CB4, 2, &mut resources.spu.voice11_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CB6, 2, &mut resources.spu.voice11_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CB8, 4, &mut resources.spu.voice11_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CBC, 2, &mut resources.spu.voice11_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CBE, 2, &mut resources.spu.voice11_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CC0, 2, &mut resources.spu.voice12_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CC2, 2, &mut resources.spu.voice12_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CC4, 2, &mut resources.spu.voice12_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CC6, 2, &mut resources.spu.voice12_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CC8, 4, &mut resources.spu.voice12_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CCC, 2, &mut resources.spu.voice12_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CCE, 2, &mut resources.spu.voice12_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CD0, 2, &mut resources.spu.voice13_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CD2, 2, &mut resources.spu.voice13_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CD4, 2, &mut resources.spu.voice13_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CD6, 2, &mut resources.spu.voice13_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CD8, 4, &mut resources.spu.voice13_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CDC, 2, &mut resources.spu.voice13_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CDE, 2, &mut resources.spu.voice13_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CE0, 2, &mut resources.spu.voice14_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CE2, 2, &mut resources.spu.voice14_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CE4, 2, &mut resources.spu.voice14_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CE6, 2, &mut resources.spu.voice14_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CE8, 4, &mut resources.spu.voice14_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CEC, 2, &mut resources.spu.voice14_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CEE, 2, &mut resources.spu.voice14_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CF0, 2, &mut resources.spu.voice15_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CF2, 2, &mut resources.spu.voice15_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CF4, 2, &mut resources.spu.voice15_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CF6, 2, &mut resources.spu.voice15_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CF8, 4, &mut resources.spu.voice15_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CFC, 2, &mut resources.spu.voice15_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1CFE, 2, &mut resources.spu.voice15_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D00, 2, &mut resources.spu.voice16_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D02, 2, &mut resources.spu.voice16_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D04, 2, &mut resources.spu.voice16_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D06, 2, &mut resources.spu.voice16_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D08, 4, &mut resources.spu.voice16_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D0C, 2, &mut resources.spu.voice16_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D0E, 2, &mut resources.spu.voice16_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D10, 2, &mut resources.spu.voice17_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D12, 2, &mut resources.spu.voice17_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D14, 2, &mut resources.spu.voice17_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D16, 2, &mut resources.spu.voice17_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D18, 4, &mut resources.spu.voice17_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D1C, 2, &mut resources.spu.voice17_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D1E, 2, &mut resources.spu.voice17_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D20, 2, &mut resources.spu.voice18_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D22, 2, &mut resources.spu.voice18_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D24, 2, &mut resources.spu.voice18_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D26, 2, &mut resources.spu.voice18_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D28, 4, &mut resources.spu.voice18_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D2C, 2, &mut resources.spu.voice18_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D2E, 2, &mut resources.spu.voice18_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D30, 2, &mut resources.spu.voice19_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D32, 2, &mut resources.spu.voice19_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D34, 2, &mut resources.spu.voice19_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D36, 2, &mut resources.spu.voice19_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D38, 4, &mut resources.spu.voice19_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D3C, 2, &mut resources.spu.voice19_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D3E, 2, &mut resources.spu.voice19_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D40, 2, &mut resources.spu.voice20_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D42, 2, &mut resources.spu.voice20_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D44, 2, &mut resources.spu.voice20_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D46, 2, &mut resources.spu.voice20_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D48, 4, &mut resources.spu.voice20_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D4C, 2, &mut resources.spu.voice20_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D4E, 2, &mut resources.spu.voice20_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D50, 2, &mut resources.spu.voice21_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D52, 2, &mut resources.spu.voice21_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D54, 2, &mut resources.spu.voice21_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D56, 2, &mut resources.spu.voice21_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D58, 4, &mut resources.spu.voice21_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D5C, 2, &mut resources.spu.voice21_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D5E, 2, &mut resources.spu.voice21_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D60, 2, &mut resources.spu.voice22_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D62, 2, &mut resources.spu.voice22_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D64, 2, &mut resources.spu.voice22_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D66, 2, &mut resources.spu.voice22_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D68, 4, &mut resources.spu.voice22_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D6C, 2, &mut resources.spu.voice22_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D6E, 2, &mut resources.spu.voice22_raddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D70, 2, &mut resources.spu.voice23_voll as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D72, 2, &mut resources.spu.voice23_volr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D74, 2, &mut resources.spu.voice23_srate as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D76, 2, &mut resources.spu.voice23_saddr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D78, 4, &mut resources.spu.voice23_adsr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D7C, 2, &mut resources.spu.voice23_cvol as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1D7E, 2, &mut resources.spu.voice23_raddr as *mut dyn B8MemoryMap);
}
