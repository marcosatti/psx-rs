use num_traits::One;
use std::ops::{BitAnd, BitOr, Not, Shl, Shr, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Bitfield {
    pub start: usize,
    pub length: usize,
}

impl Bitfield {
    #[must_use]
    pub const fn new(start: usize, length: usize) -> Bitfield {
        Bitfield {
            start: start,
            length: length,
        }
    }

    #[must_use]
    pub fn unshifted_mask<T>(&self) -> T
    where
        T: Shl<usize, Output = T> + Sub<T, Output = T> + One,
    {
        (T::one() << self.length) - T::one()
    }

    #[must_use]
    pub fn shifted_mask<T>(&self) -> T
    where
        T: Shl<usize, Output = T> + Sub<T, Output = T> + One,
    {
        Self::unshifted_mask::<T>(self) << self.start
    }

    #[must_use]
    pub fn extract_from<T>(&self, value: T) -> T
    where
        T: Shl<usize, Output = T>
            + Sub<T, Output = T>
            + One
            + Shr<usize, Output = T>
            + BitAnd<T, Output = T>,
    {
        (value & self.shifted_mask()) >> self.start
    }

    #[must_use]
    pub fn insert_into<T>(&self, destination: T, source: T) -> T
    where
        T: Shl<usize, Output = T>
            + Sub<T, Output = T>
            + One
            + Shr<usize, Output = T>
            + BitAnd<T, Output = T>
            + BitOr<T, Output = T>
            + Not<Output = T>,
    {
        let destination_masked = destination & (!self.shifted_mask::<T>());
        let source_masked = (source & self.unshifted_mask()) << self.start;
        destination_masked | source_masked
    }

    /// Example:
    ///    self.shifted_mask() == 0b1111_0000           // Example: register interrupt mask.
    ///    value == 0b0110_0000                         // Incoming value acknowledging IRQ5 and IRQ6.
    ///    self.acknowledge_mask(value) == 0b1001_1111  // New mask to apply onto register.
    #[must_use]
    pub fn acknowledge_mask<T>(&self, value: T) -> T
    where
        T: Shl<usize, Output = T>
            + Sub<T, Output = T>
            + BitAnd<T, Output = T>
            + Not<Output = T>
            + One,
    {
        !(value & self.shifted_mask())
    }

    #[must_use]
    pub fn acknowledge<T>(&self, value: T, acknowledge_value: T) -> T
    where
        T: Shl<usize, Output = T>
            + Sub<T, Output = T>
            + BitAnd<T, Output = T>
            + Not<Output = T>
            + One,
    {
        value & self.acknowledge_mask(acknowledge_value)
    }

    #[must_use]
    pub fn copy<T>(&self, destination: T, source: T) -> T
    where
        T: Shl<usize, Output = T>
            + Sub<T, Output = T>
            + One
            + Shr<usize, Output = T>
            + BitAnd<T, Output = T>
            + BitOr<T, Output = T>
            + Not<Output = T>,
    {
        self.insert_into(destination, self.extract_from(source))
    }
}
