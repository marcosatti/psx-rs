#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Stereo {
    pub left: i16,
    pub right: i16,
}

impl Stereo {
    pub fn new(left: i16, right: i16) -> Stereo {
        Stereo {
            left,
            right,
        }
    }
}
