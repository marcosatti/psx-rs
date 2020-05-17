#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct Stereo {
    pub(crate) left: i16,
    pub(crate) right: i16,
}

impl Stereo {
    pub(crate) fn new(left: i16, right: i16) -> Stereo {
        Stereo {
            left,
            right,
        }
    }
}
