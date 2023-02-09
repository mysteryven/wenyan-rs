use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WithSpan<T> {
    value: T,
    span: Span,
}

impl<T> WithSpan<T> {
    pub fn new(value: T, span: Span) -> Self {
        WithSpan { value, span }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}

#[derive(Debug, Serialize)]
pub struct BytePos(pub usize);

#[derive(Debug, Serialize)]
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
