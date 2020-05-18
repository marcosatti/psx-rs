pub(crate) trait U32Primitive {
    #[must_use]
    fn extract_u8_le(value: Self, offset: usize) -> u8;
    #[must_use]
    fn insert_u8_le(value: Self, offset: usize, other: u8) -> Self;
    #[must_use]
    fn extract_u16_le(value: Self, offset: usize) -> u16;
    #[must_use]
    fn insert_u16_le(value: Self, offset: usize, other: u16) -> Self;
}

impl U32Primitive for u32 {
    fn extract_u8_le(value: Self, offset: usize) -> u8 {
        value.to_le_bytes()[offset]
    }

    fn insert_u8_le(value: Self, offset: usize, other: u8) -> Self {
        let mut bytes = value.to_le_bytes();
        bytes[offset as usize] = other;
        u32::from_le_bytes(bytes)
    }

    fn extract_u16_le(value: Self, offset: usize) -> u16 {
        let bytes = value.to_le_bytes();
        let part = [bytes[offset * 2], bytes[offset * 2 + 1]];
        u16::from_le_bytes(part)
    }

    fn insert_u16_le(value: Self, offset: usize, other: u16) -> Self {
        let other = other.to_le_bytes();
        let mut bytes = value.to_le_bytes();
        for i in 0..2 {
            bytes[offset * 2 + i] = other[i];
        }
        u32::from_le_bytes(bytes)
    }
}

pub(crate) trait U16Primitive {
    #[must_use]
    fn extract_u8_le(value: Self, offset: usize) -> u8;
    #[must_use]
    fn insert_u8_le(value: Self, offset: usize, other: u8) -> Self;
}

impl U16Primitive for u16 {
    fn extract_u8_le(value: Self, offset: usize) -> u8 {
        value.to_le_bytes()[offset]
    }

    fn insert_u8_le(value: Self, offset: usize, other: u8) -> Self {
        let mut bytes = value.to_le_bytes();
        bytes[offset as usize] = other;
        u16::from_le_bytes(bytes)
    }
}
