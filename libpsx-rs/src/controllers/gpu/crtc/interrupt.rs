//use log::debug;
use crate::resources::Resources;
use crate::resources::intc::VBLANK;

pub fn handle_vblank_interrupt(resources: &mut Resources) {
    resources.intc.stat.set_irq(VBLANK);
    //debug!("VBLANK interrupt fired");
}
