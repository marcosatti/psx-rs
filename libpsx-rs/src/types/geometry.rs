pub(crate) use euclid::{
    Point2D,
    Size2D,
    Rect,
    UnknownUnit,
};
use smallvec::SmallVec;
use crate::types::array::AsFlattened;

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
