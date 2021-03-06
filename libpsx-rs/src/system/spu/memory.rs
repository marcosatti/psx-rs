use crate::system::{
    bus::types::*,
    spu::controllers::dac::voice::*,
    types::State,
};

pub(crate) fn voice_voll_read_u16(state: &State, offset: u32, voice_id: usize) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(get_voll(state, voice_id).read_u16())
}

pub(crate) fn voice_voll_write_u16(state: &State, offset: u32, value: u16, voice_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_voll(state, voice_id).write_u16(value))
}

pub(crate) fn voice_volr_read_u16(state: &State, offset: u32, voice_id: usize) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(get_volr(state, voice_id).read_u16())
}

pub(crate) fn voice_volr_write_u16(state: &State, offset: u32, value: u16, voice_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_volr(state, voice_id).write_u16(value))
}

pub(crate) fn voice_srate_read_u16(state: &State, offset: u32, voice_id: usize) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(get_srate(state, voice_id).read_u16())
}

pub(crate) fn voice_srate_write_u16(state: &State, offset: u32, value: u16, voice_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_srate(state, voice_id).write_u16(value))
}

pub(crate) fn voice_saddr_read_u16(state: &State, offset: u32, voice_id: usize) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(get_saddr(state, voice_id).read_u16())
}

pub(crate) fn voice_saddr_write_u16(state: &State, offset: u32, value: u16, voice_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_saddr(state, voice_id).write_u16(value))
}

pub(crate) fn voice_adsr_read_u16(state: &State, offset: u32, voice_id: usize) -> ReadResult<u16> {
    Ok(get_adsr(state, voice_id).read_u16(offset / 2))
}

pub(crate) fn voice_adsr_write_u16(state: &State, offset: u32, value: u16, voice_id: usize) -> WriteResult {
    Ok(get_adsr(state, voice_id).write_u16(offset / 2, value))
}

pub(crate) fn voice_cvol_read_u16(state: &State, offset: u32, voice_id: usize) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(get_cvol(state, voice_id).read_u16())
}

pub(crate) fn voice_cvol_write_u16(state: &State, offset: u32, value: u16, voice_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_cvol(state, voice_id).write_u16(value))
}

pub(crate) fn voice_raddr_read_u16(state: &State, offset: u32, voice_id: usize) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(get_raddr(state, voice_id).read_u16())
}

pub(crate) fn voice_raddr_write_u16(state: &State, offset: u32, value: u16, voice_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_raddr(state, voice_id).write_u16(value))
}

pub(crate) fn main_volume_left_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.main_volume_left.read_u16())
}

pub(crate) fn main_volume_left_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.main_volume_left.write_u16(value))
}

pub(crate) fn main_volume_right_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.main_volume_right.read_u16())
}

pub(crate) fn main_volume_right_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.main_volume_right.write_u16(value))
}

pub(crate) fn reverb_volume_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.reverb_volume.read_u16(offset / 2))
}

pub(crate) fn reverb_volume_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.reverb_volume.write_u16(offset / 2, value))
}

pub(crate) fn reverb_volume_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.reverb_volume.read_u32())
}

pub(crate) fn reverb_volume_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.reverb_volume.write_u32(value))
}

pub(crate) fn voice_key_on_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    state.spu.voice_key_on.read_u16(offset / 2).map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn voice_key_on_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    state.spu.voice_key_on.write_u16(offset / 2, value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn voice_key_on_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    state.spu.voice_key_on.read_u32().map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn voice_key_on_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    state.spu.voice_key_on.write_u32(value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn voice_key_off_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    state.spu.voice_key_off.read_u16(offset / 2).map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn voice_key_off_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    state.spu.voice_key_off.write_u16(offset / 2, value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn voice_key_off_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    state.spu.voice_key_off.read_u32().map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn voice_key_off_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    state.spu.voice_key_off.write_u32(value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn data_transfer_address_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    state.spu.data_transfer_address.read_u16().map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn data_transfer_address_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    state.spu.data_transfer_address.write_u16(value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn data_fifo_read_u16(_state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    unimplemented!();
}

pub(crate) fn data_fifo_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    state.spu.data_fifo.write_one(value).map_err(|_| WriteErrorKind::Full)
}

pub(crate) fn voice_channel_fm_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.voice_channel_fm.read_u16(offset / 2))
}

pub(crate) fn voice_channel_fm_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.voice_channel_fm.write_u16(offset / 2, value))
}

pub(crate) fn voice_channel_fm_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_channel_fm.read_u32())
}

pub(crate) fn voice_channel_fm_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_channel_fm.write_u32(value))
}

pub(crate) fn voice_channel_noise_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.voice_channel_noise.read_u16(offset / 2))
}

pub(crate) fn voice_channel_noise_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.voice_channel_noise.write_u16(offset / 2, value))
}

pub(crate) fn voice_channel_noise_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_channel_noise.read_u32())
}

pub(crate) fn voice_channel_noise_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_channel_noise.write_u32(value))
}

pub(crate) fn voice_channel_reverb_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.voice_channel_reverb.read_u16(offset / 2))
}

pub(crate) fn voice_channel_reverb_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.voice_channel_reverb.write_u16(offset / 2, value))
}

pub(crate) fn voice_channel_reverb_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_channel_reverb.read_u32())
}

pub(crate) fn voice_channel_reverb_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_channel_reverb.write_u32(value))
}

pub(crate) fn voice_channel_status_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.voice_channel_status.read_u16(offset / 2))
}

pub(crate) fn voice_channel_status_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.voice_channel_status.write_u16(offset / 2, value))
}

pub(crate) fn voice_channel_status_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_channel_status.read_u32())
}

pub(crate) fn voice_channel_status_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_channel_status.write_u32(value))
}

pub(crate) fn unknown_0_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.unknown_0.read_u16())
}

pub(crate) fn unknown_0_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.unknown_0.write_u16(value))
}

pub(crate) fn reverb_start_address_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.reverb_start_address.read_u16())
}

pub(crate) fn reverb_start_address_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.reverb_start_address.write_u16(value))
}

pub(crate) fn irq_address_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.irq_address.read_u16())
}

pub(crate) fn irq_address_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.irq_address.write_u16(value))
}

pub(crate) fn control_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    state.spu.control.read_u16().map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn control_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    state.spu.control.write_u16(value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn data_transfer_control_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.data_transfer_control.read_u16())
}

pub(crate) fn data_transfer_control_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.data_transfer_control.write_u16(value))
}

pub(crate) fn stat_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.stat.read_u16())
}

pub(crate) fn stat_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.stat.write_u16(value))
}

pub(crate) fn cd_volume_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.cd_volume.read_u16(offset / 2))
}

pub(crate) fn cd_volume_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.cd_volume.write_u16(offset / 2, value))
}

pub(crate) fn cd_volume_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.cd_volume.read_u32())
}

pub(crate) fn cd_volume_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.cd_volume.write_u32(value))
}

pub(crate) fn extern_volume_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.extern_volume.read_u16(offset / 2))
}

pub(crate) fn extern_volume_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.extern_volume.write_u16(offset / 2, value))
}

pub(crate) fn extern_volume_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.extern_volume.read_u32())
}

pub(crate) fn extern_volume_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.extern_volume.write_u32(value))
}

pub(crate) fn current_volume_left_read_u16(_state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    log::warn!("Read from current volume left - returning max volume (0x7FFF)");
    Ok(0x7FFF)
}

pub(crate) fn current_volume_left_write_u16(_state: &State, offset: u32, _value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    unimplemented!();
}

pub(crate) fn current_volume_right_read_u16(_state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    log::warn!("Read from current volume right - returning max volume (0x7FFF)");
    Ok(0x7FFF)
}

pub(crate) fn current_volume_right_write_u16(_state: &State, offset: u32, _value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    unimplemented!();
}

pub(crate) fn unknown_1_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.unknown_1.read_u16(offset / 2))
}

pub(crate) fn unknown_1_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.unknown_1.write_u16(offset / 2, value))
}

pub(crate) fn unknown_1_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.unknown_1.read_u32())
}

pub(crate) fn unknown_1_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.unknown_1.write_u32(value))
}

pub(crate) fn dapf1_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.dapf1.read_u16())
}

pub(crate) fn dapf1_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.dapf1.write_u16(value))
}

pub(crate) fn dapf2_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.dapf2.read_u16())
}

pub(crate) fn dapf2_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.dapf2.write_u16(value))
}

pub(crate) fn viir_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.viir.read_u16())
}

pub(crate) fn viir_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.viir.write_u16(value))
}

pub(crate) fn vcomb1_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.vcomb1.read_u16())
}

pub(crate) fn vcomb1_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.vcomb1.write_u16(value))
}

pub(crate) fn vcomb2_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.vcomb2.read_u16())
}

pub(crate) fn vcomb2_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.vcomb2.write_u16(value))
}

pub(crate) fn vcomb3_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.vcomb3.read_u16())
}

pub(crate) fn vcomb3_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.vcomb3.write_u16(value))
}

pub(crate) fn vcomb4_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.vcomb4.read_u16())
}

pub(crate) fn vcomb4_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.vcomb4.write_u16(value))
}

pub(crate) fn vwall_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.vwall.read_u16())
}

pub(crate) fn vwall_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.vwall.write_u16(value))
}

pub(crate) fn vapf1_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.vapf1.read_u16())
}

pub(crate) fn vapf1_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.vapf1.write_u16(value))
}

pub(crate) fn vapf2_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.vapf2.read_u16())
}

pub(crate) fn vapf2_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.vapf2.write_u16(value))
}

pub(crate) fn msame_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.msame.read_u16(offset / 2))
}

pub(crate) fn msame_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.msame.write_u16(offset / 2, value))
}

pub(crate) fn msame_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.msame.read_u32())
}

pub(crate) fn msame_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.msame.write_u32(value))
}

pub(crate) fn mcomb1_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.mcomb1.read_u16(offset / 2))
}

pub(crate) fn mcomb1_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.mcomb1.write_u16(offset / 2, value))
}

pub(crate) fn mcomb1_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.mcomb1.read_u32())
}

pub(crate) fn mcomb1_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.mcomb1.write_u32(value))
}

pub(crate) fn mcomb2_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.mcomb2.read_u16(offset / 2))
}

pub(crate) fn mcomb2_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.mcomb2.write_u16(offset / 2, value))
}

pub(crate) fn mcomb2_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.mcomb2.read_u32())
}

pub(crate) fn mcomb2_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.mcomb2.write_u32(value))
}

pub(crate) fn dsame_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.dsame.read_u16(offset / 2))
}

pub(crate) fn dsame_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.dsame.write_u16(offset / 2, value))
}

pub(crate) fn dsame_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.dsame.read_u32())
}

pub(crate) fn dsame_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.dsame.write_u32(value))
}

pub(crate) fn mdiff_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.mdiff.read_u16(offset / 2))
}

pub(crate) fn mdiff_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.mdiff.write_u16(offset / 2, value))
}

pub(crate) fn mdiff_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.mdiff.read_u32())
}

pub(crate) fn mdiff_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.mdiff.write_u32(value))
}

pub(crate) fn mcomb3_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.mcomb3.read_u16(offset / 2))
}

pub(crate) fn mcomb3_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.mcomb3.write_u16(offset / 2, value))
}

pub(crate) fn mcomb3_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.mcomb3.read_u32())
}

pub(crate) fn mcomb3_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.mcomb3.write_u32(value))
}

pub(crate) fn mcomb4_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.mcomb4.read_u16(offset / 2))
}

pub(crate) fn mcomb4_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.mcomb4.write_u16(offset / 2, value))
}

pub(crate) fn mcomb4_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.mcomb4.read_u32())
}

pub(crate) fn mcomb4_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.mcomb4.write_u32(value))
}

pub(crate) fn ddiff_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.ddiff.read_u16(offset / 2))
}

pub(crate) fn ddiff_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.ddiff.write_u16(offset / 2, value))
}

pub(crate) fn ddiff_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.ddiff.read_u32())
}

pub(crate) fn ddiff_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.ddiff.write_u32(value))
}

pub(crate) fn mapf1_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.mapf1.read_u16(offset / 2))
}

pub(crate) fn mapf1_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.mapf1.write_u16(offset / 2, value))
}

pub(crate) fn mapf1_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.mapf1.read_u32())
}

pub(crate) fn mapf1_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.mapf1.write_u32(value))
}

pub(crate) fn mapf2_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.mapf2.read_u16(offset / 2))
}

pub(crate) fn mapf2_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.mapf2.write_u16(offset / 2, value))
}

pub(crate) fn mapf2_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.mapf2.read_u32())
}

pub(crate) fn mapf2_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.mapf2.write_u32(value))
}

pub(crate) fn vin_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.vin.read_u16(offset / 2))
}

pub(crate) fn vin_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.vin.write_u16(offset / 2, value))
}

pub(crate) fn vin_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.vin.read_u32())
}

pub(crate) fn vin_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.vin.write_u32(value))
}
