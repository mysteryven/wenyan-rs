use crate::{
    chunk::Chunk,
    tokenize::{position::WithSpan, scanner::Scanner, token::Token},
};

pub struct Parser<'a> {
    scanner: Scanner,
    buf: &'a str,
    chunk: &'a mut Chunk,
    current: Option<WithSpan<Token>>,
    previous: Option<WithSpan<Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a str, chunk: &'a mut Chunk) -> Self {
        let scanner = Scanner::new(buf);

        Self {
            scanner,
            buf,
            chunk,
            current: None,
            previous: None,
        }
    }
    pub fn compile(&mut self) {}
    fn binary_statement() {}
    fn advance(&mut self) {
        self.previous = self.current.take();

        loop {
            let token = self.scanner.scan_token();
        }
    }
}
