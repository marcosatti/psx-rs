use crate::system::{
    bus::types::*,
    dmac::controllers::channel::*,
    types::State,
};

pub(crate) fn madr_read_u32(state: &State, offset: u32, channel_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(get_madr(state, channel_id).read_u32())
}

pub(crate) fn madr_write_u32(state: &State, offset: u32, value: u32, channel_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_madr(state, channel_id).write_u32(value))
}

pub(crate) fn bcr_read_u32(state: &State, offset: u32, channel_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(get_bcr(state, channel_id).read_u32())
}

pub(crate) fn bcr_write_u32(state: &State, offset: u32, value: u32, channel_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_bcr(state, channel_id).write_u32(value))
}

pub(crate) fn chcr_read_u32(state: &State, offset: u32, channel_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    get_chcr(state, channel_id).read_u32().map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn chcr_write_u32(state: &State, offset: u32, value: u32, channel_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    get_chcr(state, channel_id).write_u32(value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn dpcr_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    state.dmac.dpcr.read_u32().map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn dpcr_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    state.dmac.dpcr.write_u32(value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn dicr_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    state.dmac.dicr.read_u32().map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn dicr_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    state.dmac.dicr.write_u32(value).map_err(|_| WriteErrorKind::NotReady)
}
