#[derive(Clone, Copy, Debug)]
pub enum ReadErrorKind {
    Empty,
}

pub type ReadResult<T> = Result<T, ReadErrorKind>;

#[derive(Clone, Copy, Debug)]
pub enum WriteErrorKind {
    Full,
}

pub type WriteResult = Result<(), WriteErrorKind>;
