use crate::system::{
    intc::types::Line,
    types::State,
};

pub(crate) fn handle_vblank_interrupt(state: &State) {
    state.intc.stat.assert_line(Line::Vblank);
}
