use euclid::{TypedPoint2D, TypedSize2D};

// Units
pub struct Normalized;
pub struct Pixel;

pub type Point2D<Type, Unit> = TypedPoint2D<Type, Unit>;
pub type Size2D<Type, Unit> = TypedSize2D<Type, Unit>;
