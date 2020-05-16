#[derive(Clone, Copy, Debug)]
pub enum ReadErrorKind {
    Empty,
    NotReady,
}

pub type ReadResult<T> = Result<T, ReadErrorKind>;

#[derive(Clone, Copy, Debug)]
pub enum WriteErrorKind {
    Full,
    NotReady,
}

pub type WriteResult = Result<(), WriteErrorKind>;
