use crate::{
    chunk::Chunk,
    opcode,
    tokenize::{position::WithSpan, scanner::Scanner, token::Token},
    value::Value,
};

pub struct Parser<'a> {
    scanner: Scanner,
    buf: &'a str,
    compiling_chunk: &'a mut Chunk,
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
            compiling_chunk: chunk,
            current: None,
            previous: None,
            has_error: false,
        }
    }
    pub fn current_chunk_mut(&mut self) -> &mut Chunk {
        self.compiling_chunk
    }
    pub fn compile(&mut self) -> bool {
        self.has_error = false;

        self.advance();
        self.expression();

        self.consume(Token::Eof, "Expect end of expression");

        self.end_compiler();
        return self.has_error;
    }
    fn end_compiler(&mut self) {
        self.emit_return();
    }
    pub fn emit_u8(&mut self, byte: u8) {
        let line_number = self.previous().get_line();
        self.current_chunk_mut().add_u8(byte, line_number);
    }
    fn emit_u32(&mut self, byte: u32) {
        let line_number = self.previous().get_line();
        self.current_chunk_mut().add_u32(byte, line_number);
    }
    fn emit_return(&mut self) {
        self.emit_u8(opcode::RETURN);
    }
    fn emit_constant(&mut self, value: Value) {
        self.emit_u8(opcode::CONSTANT);
        if let Some(num) = self.make_constant(value) {
            self.emit_u32(num);
        }
    }
    fn make_constant(&mut self, value: Value) -> Option<u32> {
        let constant = self.current_chunk_mut().add_constant(value);

        match u32::try_from(constant) {
            Ok(num) => Some(num),
            Err(_) => {
                self.error("Too many constants in one chunk.");
                None
            }
        }
    }

    pub fn previous(&self) -> &WithSpan<Token> {
        self.previous.as_ref().unwrap()
    }
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

    pub fn is_match(&mut self, token: Token) -> bool {
        match self.is_kind_of(self.current.as_ref().unwrap(), token) {
            true => {
                self.advance();
                true
            }
            false => false,
        }
    }

    pub fn error_at_current(&mut self, msg: &str) {
        self.has_error = true;

        self.error_at(self.current.as_ref().unwrap(), msg)
    }
    pub fn error(&mut self, msg: &str) {
        self.has_error = true;
        self.error_at(self.previous.as_ref().unwrap(), msg)
    }
    pub fn error_at(&self, token: &WithSpan<Token>, msg: &str) {
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
    fn pick_str(&self, token: &WithSpan<Token>) -> &str {
        let start = token.get_start();
        let end = token.get_end();

        &self.buf[start..end]
    }
    fn number(&mut self) {
        let _s = self.pick_str(&self.previous());
        let value = Value::Number(111.0);
        self.emit_constant(value);
    }
    pub fn expression(&mut self) {}
}
