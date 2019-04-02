use num_traits::clamp;
use crate::types::bitfield::Bitfield;
use crate::resources::spu::voice::*;

pub fn decode_header(data: [u8; 2]) -> AdpcmParams {
    AdpcmParams {
        shift: Bitfield::new(0, 4).extract_from(data[0]) as usize,
        filter: Bitfield::new(4, 4).extract_from(data[0]) as usize,
        loop_end: Bitfield::new(0, 1).extract_from(data[1]) != 0,
        loop_repeat: Bitfield::new(1, 1).extract_from(data[1]) != 0,
        loop_start: Bitfield::new(2, 1).extract_from(data[1]) != 0,
    }
}

pub fn decode_frame(data: u8, params: &AdpcmParams, old_sample: &mut i16, older_sample: &mut i16) -> [i16; 2]{
    let mut samples = [
        Bitfield::new(0, 4).extract_from(data) as i16,
        Bitfield::new(4, 4).extract_from(data) as i16,
    ];

    let pos_filter_value = [0, 60, 115, 98, 122][params.filter];
    let neg_filter_value = [0, 0, -52, -55, -60][params.filter];

    for i in 0..2 {
        let mut shifted_sample = ((samples[i] << 12) as i32) >> params.shift;
        // TODO: change formula to the no cash psx docs way - the pcsxr way is wrong? Only this way for debugging.
        // Right shift != division (pow 2) when negatives...
        shifted_sample += ((*old_sample as i32 * pos_filter_value) >> 6) + ((*older_sample as i32 * neg_filter_value) >> 6);

        samples[i] = clamp(shifted_sample, std::i16::MIN as i32, std::i16::MAX as i32) as i16; 

        *older_sample = *old_sample;
        *old_sample = samples[i];
    }

    samples
}
