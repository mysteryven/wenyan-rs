use crate::{chunk::Chunk, tokenize::scanner::Scanner};

pub struct Parser<'a> {
    scanner: Scanner,
    buf: &'a str,
    chunk: &'a mut Chunk,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a str, chunk: &'a mut Chunk) -> Self {
        let scanner = Scanner::new(buf);
        Self {
            scanner,
            buf,
            chunk,
        }
    }
    pub fn compile(&mut self) {}
    fn binary_statement() {}
    fn advance(&mut self) {}
}
