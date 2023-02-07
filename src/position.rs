pub struct WithSpan<T> {
    pub value: T,
    pub span: Span,
}

pub struct BytePos(pub usize);

pub struct Span {
    pub start: BytePos,
    pub end: BytePos,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: BytePos(start),
            end: BytePos(end),
        }
    }
}
