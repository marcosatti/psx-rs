use crate::types::array::AsFlattened;
pub(crate) use euclid::{
    Point2D,
    Rect,
    Size2D,
};
use smallvec::SmallVec;

// Units
pub(crate) struct Normalized;
pub(crate) struct NormalizedTexcoord;
pub(crate) struct Pixel;

impl<BaseType: Copy, Unit> AsFlattened for [Point2D<BaseType, Unit>] {
    type Output = BaseType;

    fn as_flattened(&self) -> SmallVec<[Self::Output; 16]> {
        let mut buffer = SmallVec::new();

        for item in self.iter() {
            buffer.push(item.x);
            buffer.push(item.y);
        }

        buffer
    }
}

impl<BaseType: Copy, Unit> AsFlattened for [Size2D<BaseType, Unit>] {
    type Output = BaseType;

    fn as_flattened(&self) -> SmallVec<[Self::Output; 16]> {
        let mut buffer = SmallVec::new();

        for item in self.iter() {
            buffer.push(item.width);
            buffer.push(item.height);
        }

        buffer
    }
}

pub(crate) trait ToUsizeChecked {
    type Output;

    fn to_usize_checked(&self) -> Self::Output;
}

impl ToUsizeChecked for Point2D<isize, Pixel> {
    type Output = Point2D<usize, Pixel>;

    fn to_usize_checked(&self) -> Self::Output {
        assert!(self.x >= 0, "X coordinate is not positive: {}", self.x);
        assert!(self.y >= 0, "Y coordinate is not positive: {}", self.y);
        self.cast()
    }
}

impl ToUsizeChecked for Size2D<isize, Pixel> {
    type Output = Size2D<usize, Pixel>;

    fn to_usize_checked(&self) -> Self::Output {
        assert!(self.width >= 0, "Width is not positive: {}", self.width);
        assert!(self.height >= 0, "Height is not positive: {}", self.height);
        self.cast()
    }
}
