//use log::debug;
use crate::system::types::State;
use crate::system::intc::types::Line;

pub fn handle_vblank_interrupt(state: &mut State) {
    let stat = &resources.intc.stat;
    stat.assert_line(Line::Vblank);
//    debug!("VBLANK interrupt fired");
}
