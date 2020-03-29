use crate::system::types::State;

pub fn handle_current_volume(state: &mut State) {
    let main_volume_left = &mut state.spu.main_volume_left;
    let main_volume_right = &mut state.spu.main_volume_right;
    let current_volume_left = &mut state.spu.current_volume_left;
    let current_volume_right = &mut state.spu.current_volume_right;

    current_volume_left.write_u16(main_volume_left.read_u16());
    current_volume_right.write_u16(main_volume_right.read_u16());
}
