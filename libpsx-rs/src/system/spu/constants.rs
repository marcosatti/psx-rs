use crate::types::bitfield::Bitfield;
use std::time::Duration;

pub(crate) const _CONTROL_CD_AUDIO_ENABLE: Bitfield = Bitfield::new(0, 1);
pub(crate) const _CONTROL_EXTERNAL_AUDIO_ENABLE: Bitfield = Bitfield::new(1, 1);
pub(crate) const _CONTROL_CD_AUDIO_REVERB: Bitfield = Bitfield::new(2, 1);
pub(crate) const _CONTROL_EXTERNAL_AUDIO_REVERB: Bitfield = Bitfield::new(3, 1);
pub(crate) const CONTROL_TRANSFER_MODE: Bitfield = Bitfield::new(4, 2);
pub(crate) const CONTROL_UNMUTE: Bitfield = Bitfield::new(14, 1);
pub(crate) const CONTROL_ENABLE: Bitfield = Bitfield::new(15, 1);

pub(crate) const _STAT_CD_AUDIO_ENABLE: Bitfield = Bitfield::new(0, 1);
pub(crate) const _STAT_EXTERNAL_AUDIO_ENABLE: Bitfield = Bitfield::new(1, 1);
pub(crate) const _STAT_CD_AUDIO_REVERB: Bitfield = Bitfield::new(2, 1);
pub(crate) const _STAT_EXTERNAL_AUDIO_REVERB: Bitfield = Bitfield::new(3, 1);
pub(crate) const _STAT_IRQ_FLAG: Bitfield = Bitfield::new(6, 1);
pub(crate) const _STAT_DMA_RW_REQUEST: Bitfield = Bitfield::new(7, 1);
pub(crate) const _STAT_DMA_W_REQUEST: Bitfield = Bitfield::new(8, 1);
pub(crate) const _STAT_DMA_R_REQUEST: Bitfield = Bitfield::new(9, 1);
pub(crate) const STAT_DATA_BUSY_FLAG: Bitfield = Bitfield::new(10, 1);
pub(crate) const _STAT_WRITING_BUFFER_HALF: Bitfield = Bitfield::new(11, 1);

pub(crate) const VOLUME_MODE: Bitfield = Bitfield::new(15, 1);

pub(crate) const SWEEP_STEP: Bitfield = Bitfield::new(0, 2);
pub(crate) const SWEEP_SHIFT: Bitfield = Bitfield::new(2, 5);
pub(crate) const SWEEP_PHASE: Bitfield = Bitfield::new(12, 1);
pub(crate) const SWEEP_DIRECTION: Bitfield = Bitfield::new(13, 1);
pub(crate) const SWEEP_MODE: Bitfield = Bitfield::new(14, 1);

pub(crate) const ADPCM_SHIFT: Bitfield = Bitfield::new(0, 4);
pub(crate) const ADPCM_FILTER: Bitfield = Bitfield::new(4, 4);
pub(crate) const ADPCM_LOOP_END: Bitfield = Bitfield::new(0, 1);
pub(crate) const ADPCM_LOOP_REPEAT: Bitfield = Bitfield::new(1, 1);
pub(crate) const ADPCM_LOOP_START: Bitfield = Bitfield::new(2, 1);

pub(crate) const ADSR_SUSTAIN_LEVEL: Bitfield = Bitfield::new(0, 4);
pub(crate) const ADSR_DECAY_SHIFT: Bitfield = Bitfield::new(4, 4);
pub(crate) const ADSR_ATTACK_STEP: Bitfield = Bitfield::new(8, 2);
pub(crate) const ADSR_ATTACK_SHIFT: Bitfield = Bitfield::new(10, 5);
pub(crate) const ADSR_ATTACK_MODE: Bitfield = Bitfield::new(15, 1);
pub(crate) const ADSR_RELEASE_SHIFT: Bitfield = Bitfield::new(16, 5);
pub(crate) const ADSR_RELEASE_MODE: Bitfield = Bitfield::new(21, 1);
pub(crate) const ADSR_SUSTAIN_STEP: Bitfield = Bitfield::new(22, 2);
pub(crate) const ADSR_SUSTAIN_SHIFT: Bitfield = Bitfield::new(24, 5);
pub(crate) const ADSR_SUSTAIN_DIRECTION: Bitfield = Bitfield::new(30, 1);
pub(crate) const ADSR_SUSTAIN_MODE: Bitfield = Bitfield::new(31, 1);

pub(crate) const CLOCK_SPEED: f64 = 33.8688 * 1e6; // 33.8688 MHz
pub(crate) const SAMPLE_RATE: f64 = 44100.0; // 44.1 kHz
pub(crate) const _SAMPLE_RATE_PERIOD: Duration = Duration::from_nanos(22676); // 1 / 44100th of a second
pub(crate) const BUFFER_SIZE: usize = 2048;
pub(crate) const VOICES_COUNT: usize = 24;
