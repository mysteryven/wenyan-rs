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
    has_error: bool,
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
            has_error: false,
        }
    }
    pub fn compile(&mut self) {}
    fn binary_statement() {}
    fn advance(&mut self) {
        self.previous = self.current.take();

        loop {
            let token = self.scanner.scan_token();

            if self.is_kind_of(&token, Token::Eof) {
                break;
            }

            match token.get_value() {
                Token::Error(msg) => self.error_at_current(msg),
                _ => {}
            }
        }
    }
    fn consume(&mut self, token: Token, msg: &str) {
        if self.is_kind_of(self.current.as_ref().unwrap(), token) {
            self.advance();
            return;
        }

        self.error_at_current(msg);
    }

    fn error_at_current(&mut self, msg: &str) {
        self.has_error = true;

        self.error_at(self.current.as_ref().unwrap(), msg)
    }
    fn error(&mut self, msg: &str) {
        self.has_error = true;
        self.error_at(self.previous.as_ref().unwrap(), msg)
    }
    fn error_at(&self, token: &WithSpan<Token>, msg: &str) {
        print!("[line {}] error", token.get_line());

        if self.is_kind_of(token, Token::Eof) {
            print!(" at end")
        }

        let is_match_error = match token.get_value() {
            Token::Error(_) => true,
            _ => false,
        };

        if !is_match_error {
            print!(" at {} to {}", token.get_start(), token.get_end())
        }

        print!(": {}\n", msg)
    }
    fn is_kind_of(&self, t: &WithSpan<Token>, target: Token) -> bool {
        *t.get_value() == target
    }
}
