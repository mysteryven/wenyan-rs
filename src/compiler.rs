use crate::{
    chunk::Chunk,
    convert::hanzi2num::hanzi2num,
    interpreter::Runtime,
    opcode,
    statements::{
        binary_if_statement, binary_statement, expression_statement, normal_declaration,
        print_statement, unary_statement,
    },
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
    runtime: &'a mut Runtime,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a str, chunk: &'a mut Chunk, runtime: &'a mut Runtime) -> Self {
        let scanner = Scanner::new(buf);

        Self {
            scanner,
            buf,
            compiling_chunk: chunk,
            current: None,
            previous: None,
            has_error: false,
            runtime,
        }
    }
    pub fn current_chunk_mut(&mut self) -> &mut Chunk {
        self.compiling_chunk
    }
    pub fn compile(&mut self) -> bool {
        self.has_error = false;

        self.advance();

        while !self.is_match(Token::Eof) {
            self.declaration();
        }

        self.consume(Token::Eof, "Expect end of expression");

        self.end_compiler();
        return self.has_error;
    }
    fn declaration(&mut self) {
        if self.is_match(Token::Decl) {
            self.normal_declaration();
        } else if self.is_match(Token::DeclShort) {
            self.short_declaration()
        } else {
            self.statement();
        }

        if self.has_error {
            self.synchronize();
        }
    }
    fn short_declaration(&mut self) {
        todo!()
    }
    fn normal_declaration(&mut self) {
        normal_declaration(self, self.buf)
    }
    fn statement(&mut self) {
        let current = self.current.as_ref().unwrap().get_value().clone();

        match current {
            Token::Plus | Token::Minus | Token::Star => binary_statement(self, &current),
            Token::Invert => unary_statement(self, &current),
            Token::Print => print_statement(self),
            Token::BangEqual
            | Token::EqualEqual
            | Token::BangGreater
            | Token::BangLess
            | Token::Less
            | Token::Greater => binary_if_statement(self, &current),
            _ => expression_statement(self),
        }
    }
    fn end_compiler(&mut self) {
        self.emit_return();
    }
    fn synchronize(&mut self) {
        unimplemented!()
    }
    pub fn emit_u8(&mut self, byte: u8) {
        let line_number = self.previous().get_line();
        self.current_chunk_mut().add_u8(byte, line_number);
    }
    pub fn emit_u32(&mut self, byte: u32) {
        let line_number = self.previous().get_line();
        self.current_chunk_mut().add_u32(byte, line_number);
    }
    fn emit_return(&mut self) {
        self.emit_u8(opcode::RETURN);
    }
    pub fn emit_constant(&mut self, value: Value) {
        self.emit_u8(opcode::CONSTANT);
        if let Some(num) = self.make_constant(value) {
            self.emit_u32(num);
        }
    }
    pub fn make_constant(&mut self, value: Value) -> Option<u32> {
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
    pub fn advance(&mut self) {
        self.previous = self.current.take();

        let token = self.scanner.scan_token();

        match token.get_value() {
            Token::Error(msg) => self.error_at_current(msg),
            _ => {}
        }

        self.current = Some(token);
    }
    pub fn consume(&mut self, token: Token, msg: &str) {
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

        print!(": {}\n", msg)
    }
    fn is_kind_of(&self, t: &WithSpan<Token>, target: Token) -> bool {
        *t.get_value() == target
    }
    pub fn pick_str(&self, token: &WithSpan<Token>) -> &str {
        let start = token.get_start();
        let end = token.get_end();

        &self.buf[start..end]
    }
    fn number(&mut self) {
        let s = self.pick_str(&self.previous());
        let num_str = hanzi2num(s);
        match num_str.map(|s| s.parse::<f64>()) {
            Some(res) => match res {
                Ok(value) => {
                    self.emit_constant(Value::Number(value));
                }
                Err(_) => self.error("not a valid number"),
            },
            None => self.error("not a valid number"),
        }
    }
    pub fn str_to_value(&mut self) -> Value {
        let start = self.previous().get_start();
        let end = self.previous().get_end();
        let s = &self.buf[start..end];
        Value::String(self.runtime.interner_mut().intern(s))
    }
    pub fn expression(&mut self) {
        self.advance();
        match *self.previous().get_value() {
            Token::Number => self.number(),
            Token::True => self.emit_u8(opcode::TRUE),
            Token::False => self.emit_u8(opcode::FALSE),
            Token::String => {
                let value = self.str_to_value();
                self.emit_constant(value)
            }
            _ => self.error("Expect expression"),
        }
    }
}
