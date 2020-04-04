use crate::types::bitfield::Bitfield;
use std::time::Duration;

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

pub const CLOCK_SPEED: f64 = 33.8688 * 1e6; // 33.8688 MHz
pub const _SAMPLE_RATE: usize = 44100;
pub const SAMPLE_RATE_PERIOD: Duration = Duration::from_nanos(22676); // 1 / 44100th of a second
pub const BUFFER_SIZE: usize = 2048;
pub const VOICES_COUNT: usize = 24;
