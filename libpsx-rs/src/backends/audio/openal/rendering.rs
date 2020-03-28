use openal_sys::*;
use crate::system::spu::constants::VOICES_COUNT;

pub static mut SOURCES: [ALuint; VOICES_COUNT] = [0; VOICES_COUNT];

// Double buffering for each SPU voice, swapping between the Nth and Nth + 1 buffers.
pub static mut BUFFERS: [ALuint; VOICES_COUNT * 2] = [0; VOICES_COUNT * 2];
pub static mut RENDERING_ODD_BUFFER: [bool; VOICES_COUNT] = [false; VOICES_COUNT];
