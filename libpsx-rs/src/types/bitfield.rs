use std::ops::{Shl, Shr, Sub, BitAnd, BitOr, Not};
use num_traits::One;

#[derive(Debug, Copy, Clone)]
pub struct Bitfield {
    pub start: usize,
    pub length: usize,
}

impl Bitfield {
    pub const fn new(start: usize, length: usize) -> Bitfield {
        Bitfield { 
            start: start, 
            length: length,
        }
    }

    pub fn unshifted_mask<T>(&self) -> T 
    where 
        T: Shl<usize, Output=T> + Sub<T, Output=T> + One 
    {
        (T::one() << self.length) - T::one()
    }

    pub fn shifted_mask<T>(&self) -> T 
    where 
        T: Shl<usize, Output=T> + Sub<T, Output=T> + One 
    {
        Self::unshifted_mask::<T>(self) << self.start
    }

    pub fn extract_from<T>(&self, value: T) -> T 
    where 
        T: Shl<usize, Output=T> + Sub<T, Output=T> + One + Shr<usize, Output=T> + BitAnd<T, Output=T>
    {
        (value & self.shifted_mask()) >> self.start
    }
        
    pub fn insert_into<T>(&self, dest: T, source: T) -> T 
    where 
        T: Shl<usize, Output=T> + Sub<T, Output=T> + One + Shr<usize, Output=T> + BitAnd<T, Output=T> + BitOr<T, Output=T> + Not<Output=T>
    {
        let dest_masked =  dest & (!self.shifted_mask::<T>());
        let source_masked = (source & self.unshifted_mask()) << self.start;
        dest_masked | source_masked
    }

    pub fn acknowledge_mask<T>(&self, value: T) -> T 
    where 
        T: Shl<usize, Output=T> + Sub<T, Output=T> + BitAnd<T, Output=T> + Not<Output=T> + One 
    {
        !(value & self.shifted_mask())
    }
}
