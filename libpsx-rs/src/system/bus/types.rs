#[derive(Clone, Copy, Debug)]
pub(crate) enum ReadErrorKind {
    Empty,
    NotReady,
}

pub(crate) type ReadResult<T> = Result<T, ReadErrorKind>;

#[derive(Clone, Copy, Debug)]
pub(crate) enum WriteErrorKind {
    Full,
    NotReady,
}

pub(crate) type WriteResult = Result<(), WriteErrorKind>;
