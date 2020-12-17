use crate::types::bitfield::Bitfield;

pub(crate) const CLOCK_SPEED: f32 = 33.8688 * 1e6; // 33.8688 MHz
pub(crate) const CLOCK_SPEED_PERIOD: f32 = 1.0 / CLOCK_SPEED;

pub(crate) const STAT_TXRDY_1: Bitfield = Bitfield::new(0, 1);
pub(crate) const STAT_RXFIFO_READY: Bitfield = Bitfield::new(1, 1);
pub(crate) const STAT_TXRDY_2: Bitfield = Bitfield::new(2, 1);
pub(crate) const _STAT_RXERR_PARITY: Bitfield = Bitfield::new(3, 1);
pub(crate) const _STAT_ACK_LEVEL: Bitfield = Bitfield::new(7, 1);
pub(crate) const _STAT_IRQ: Bitfield = Bitfield::new(9, 1);
pub(crate) const _STAT_TIMER: Bitfield = Bitfield::new(11, 21);

pub(crate) const _MODE_RATE_RELOADF: Bitfield = Bitfield::new(0, 2);
pub(crate) const _MODE_CHAR_LENGTH: Bitfield = Bitfield::new(2, 2);
pub(crate) const _MODE_PARITY_ENABLE: Bitfield = Bitfield::new(4, 1);
pub(crate) const _MODE_PARITY_TYPE: Bitfield = Bitfield::new(5, 1);
pub(crate) const _MODE_CLKOUT_POLARITY: Bitfield = Bitfield::new(8, 1);

pub(crate) const CTRL_TXEN: Bitfield = Bitfield::new(0, 1);
pub(crate) const CTRL_JOYN_OUTPUT: Bitfield = Bitfield::new(1, 1);
pub(crate) const _CTRL_RXEN: Bitfield = Bitfield::new(2, 1);
pub(crate) const CTRL_ACK: Bitfield = Bitfield::new(4, 1);
pub(crate) const CTRL_RESET: Bitfield = Bitfield::new(6, 1);
pub(crate) const _CTRL_RXINT_MODE: Bitfield = Bitfield::new(8, 2);
pub(crate) const _CTRL_TXINT_ENABLE: Bitfield = Bitfield::new(10, 1);
pub(crate) const _CTRL_RXINT_ENABLE: Bitfield = Bitfield::new(11, 1);
pub(crate) const CTRL_ACKINT_ENABLE: Bitfield = Bitfield::new(12, 1);
pub(crate) const CTRL_JOY_SLOT: Bitfield = Bitfield::new(13, 1);
