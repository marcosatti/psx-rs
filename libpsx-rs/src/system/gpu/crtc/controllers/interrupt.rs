use crate::system::{
    intc::types::Line,
    types::State,
};

pub(crate) fn handle_vblank_interrupt(state: &State) {
    let stat = &state.intc.stat;
    stat.assert_line(Line::Vblank);
}
