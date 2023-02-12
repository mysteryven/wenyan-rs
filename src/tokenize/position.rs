use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WithSpan<T> {
    value: T,
    span: Span,
    line: usize,
}

impl<T> WithSpan<T> {
    pub fn new(value: T, span: Span, line: usize) -> Self {
        WithSpan { value, span, line }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
    pub fn get_line(&self) -> usize {
        self.line
    }
    pub fn get_start(&self) -> usize {
        self.span.start.0
    }
    pub fn get_end(&self) -> usize {
        self.span.end.0
    }
}

#[derive(Debug, Serialize, Default)]
pub struct BytePos(pub usize);

impl BytePos {
    pub fn shift(&mut self, ch: char) {
        self.0 += ch.len_utf8()
    }
}

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
    pub fn from(start: BytePos, end: BytePos) -> Self {
        Self { start, end }
    }
}
