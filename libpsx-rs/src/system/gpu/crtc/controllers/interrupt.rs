use crate::system::{
    intc::types::Line,
    types::State,
};

pub fn handle_vblank_interrupt(state: &mut State) {
    let stat = &state.intc.stat;
    stat.assert_line(Line::Vblank);
}
