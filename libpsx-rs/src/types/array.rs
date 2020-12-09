use smallvec::SmallVec;

pub(crate) trait AsFlattened {
    type Output;

    fn as_flattened(&self) -> SmallVec<[Self::Output; 16]>;
}
