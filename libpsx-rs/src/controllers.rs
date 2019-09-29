pub mod r3000;
pub mod gpu;
pub mod intc;
pub mod dmac;
pub mod spu;
pub mod cdrom;
pub mod padmc;

use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Time(Duration),
}
