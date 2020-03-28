//use log::debug;
use crate::resources::Resources;
use crate::resources::intc::register::Line;

pub fn handle_vblank_interrupt(resources: &mut Resources) {
    let stat = &resources.intc.stat;
    stat.assert_line(Line::Vblank);
//    debug!("VBLANK interrupt fired");
}
