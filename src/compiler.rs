use std::fmt::Debug;

use crate::{
    chunk::Chunk,
    convert::hanzi2num::hanzi2num,
    debug::Debugger,
    interpreter::Runtime,
    object::Function,
    opcode,
    statements::{
        assign_statement, binary_statement, boolean_algebra_statement, break_statement,
        expression_statement, for_statement, for_while_statement, if_statement, name_is_statement,
        normal_declaration, print_statement, unary_statement,
    },
    tokenize::{position::WithSpan, scanner::Scanner, token::Token},
    value::Value,
};

pub struct Local {
    name: String,
    depth: Depth,
}

impl Default for Local {
    fn default() -> Self {
        Self {
            name: String::default(),
            depth: 0,
        }
    }
}

type Depth = i8;

pub enum FunctionType {
    Script,
    Function,
}

pub struct Compiler {
    locals: Vec<Local>,
    scope_depth: Depth,
    function: Function,
    fun_kind: FunctionType,
    enclosing: Option<Box<Compiler>>,
}

impl Compiler {
    pub fn init(fun_kind: FunctionType) -> Box<Self> {
        let local = Local::default();
        let locals = vec![local];

        Box::new(Self {
            locals,
            scope_depth: 0,
            function: Function::new(),
            fun_kind,
        })
    }
    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }
    pub fn scope_depth(&self) -> Depth {
        self.scope_depth
    }
    pub fn add_local(&mut self, name: String) {
        self.locals.push(Local {
            name,
            depth: self.scope_depth,
        });
    }
    pub fn resolve_local(&mut self, name: String) -> Option<u32> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name.as_str() == name {
                return Some(i as u32);
            }
        }

        None
    }
}

pub struct Parser<'a> {
    scanner: Scanner,
    buf: &'a str,
    current: Option<WithSpan<Token>>,
    previous: Option<WithSpan<Token>>,
    has_error: bool,
    panic_mode: bool,
    runtime: &'a mut Runtime,
    current_compiler: Box<Compiler>,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a str, runtime: &'a mut Runtime) -> Self {
        let scanner = Scanner::new(buf);

        Self {
            scanner,
            buf,
            current: None,
            previous: None,
            has_error: false,
            panic_mode: false,
            runtime,
            current_compiler: Compiler::init(FunctionType::Script),
        }
    }
    pub fn current_code_len(&self) -> usize {
        self.current_compiler.function.chunk().code().len()
    }
    pub fn current_chunk(&self) -> &Chunk {
        self.current_compiler.function.chunk()
    }
    pub fn current_chunk_mut(&mut self) -> &mut Chunk {
        self.current_compiler.function.chunk_mut()
    }
    pub fn compile(&mut self) -> Option<Function> {
        self.has_error = false;
        self.panic_mode = false;

        self.advance();

        while !self.is_match(Token::Eof) {
            self.declaration();
        }

        self.consume(Token::Eof, "Expect end of expression");

        self.end_compiler()
    }
    pub fn end_compiler(&mut self) -> Option<Function> {
        self.emit_return();

        let fun = self.get_compiled_fn();

        if self.has_error {
            None
        } else {
            fun
        }
    }
    pub fn get_compiled_fn(&self) -> Option<Function> {
        let fun = if let Some(compiler) = Some(self.current_compiler) {
            Some(compiler.function)
        } else {
            None
        };
        if let Some(fun) = fun {
            let debug = Debugger::new(fun.chunk());

            let name = if fun.name() != "" {
                fun.name()
            } else {
                "<global context>"
            };

            debug.disassemble(name);

            Some(fun)
        } else {
            None
        }
    }
    pub fn declaration(&mut self) {
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
    pub fn statement(&mut self) {
        let current = self.current.as_ref().unwrap().get_value().clone();

        match current {
            Token::Plus | Token::Minus | Token::Star => binary_statement(self, &current),
            Token::Invert => unary_statement(self, &current),
            Token::Print => print_statement(self),

            Token::AssignFrom => assign_statement(self),
            Token::NameIs => name_is_statement(self),
            Token::If => if_statement(self),
            Token::Fu => {
                self.advance();
                self.expression();
                if self.is_kind_of(self.current(), Token::Identifier)
                    || self.is_kind_of(self.current(), Token::True)
                    || self.is_kind_of(self.current(), Token::False)
                    || self.is_kind_of(self.current(), Token::String)
                    || self.is_kind_of(self.current(), Token::Number)
                {
                    boolean_algebra_statement(self)
                } else {
                    todo!("reference statement")
                }
            }
            Token::Loop => {
                for_while_statement(self);
            }
            Token::For => for_statement(self),
            Token::Break => break_statement(self),
            _ => expression_statement(self),
        }
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;
        loop {
            let current = self.current.as_ref().unwrap().get_value().clone();

            match current {
                Token::Decl
                | Token::DeclShort
                | Token::Print
                | Token::If
                | Token::Fu
                | Token::Loop
                | Token::For
                | Token::Break
                | Token::AssignFrom
                | Token::Plus
                | Token::Minus
                | Token::Star
                | Token::Invert
                | Token::Eof => return,
                _ => {
                    self.advance();
                }
            }
        }
    }
    pub fn emit_u8(&mut self, byte: u8) {
        let line_number = self.previous().get_line();
        self.current_chunk_mut().add_u8(byte, line_number);
    }
    pub fn emit_u32(&mut self, byte: u32) {
        let line_number = self.previous().get_line();
        self.current_chunk_mut().add_u32(byte, line_number);
    }
    pub fn set_u32(&mut self, index: usize, byte: u32) {
        self.current_chunk_mut().overwrite_u32(index, byte);
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
    pub fn identifier_constant(self: &mut Self) -> Option<u32> {
        let value = self.str_to_value();

        self.make_constant(value)
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
    pub fn current(&self) -> &WithSpan<Token> {
        self.current.as_ref().unwrap()
    }

    pub fn previous(&self) -> &WithSpan<Token> {
        self.previous.as_ref().unwrap()
    }
    pub fn advance(&mut self) {
        self.previous = self.current.take();

        let token = self.scanner.scan_token();

        let msg = match token.get_value() {
            Token::Error(msg) => Some(msg.to_owned()),
            _ => None,
        };

        self.current = Some(token);

        if let Some(msg) = msg {
            self.error_at_current(msg.as_str())
        }
    }
    pub fn consume(&mut self, token: Token, msg: &str) {
        if self.is_kind_of(self.current.as_ref().unwrap(), token) {
            self.advance();
            return;
        }

        self.error_at_current(msg);
    }

    fn is_kind_of(&self, t: &WithSpan<Token>, target: Token) -> bool {
        *t.get_value() == target
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
    pub fn check(&self, token: Token) -> bool {
        self.is_kind_of(self.current.as_ref().unwrap(), token)
    }

    pub fn check_not_in_vec(&self, tokens: &[Token]) -> bool {
        tokens.iter().all(|t| !self.check(t.clone()))
    }
    pub fn check_in_vec(&self, tokens: &[Token]) -> bool {
        tokens.iter().any(|t| self.check(t.clone()))
    }

    pub fn error_at_current(&mut self, msg: &str) {
        self.has_error = true;
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.error_at(self.current.as_ref().unwrap(), msg)
    }
    pub fn error(&mut self, msg: &str) {
        self.has_error = true;
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        self.error_at(self.previous.as_ref().unwrap(), msg)
    }
    pub fn error_at(&self, token: &WithSpan<Token>, msg: &str) {
        eprint!("[line {}] compiler error", token.get_line());

        if self.is_kind_of(token, Token::Eof) {
            eprint!(" at end")
        }

        eprint!(": {}\n", msg)
    }

    pub fn pick_str(&self, token: &WithSpan<Token>) -> &str {
        let start = token.get_start();
        let end = token.get_end();

        &self.buf[start..end]
    }
    pub fn str_to_value(&mut self) -> Value {
        let start = self.previous().get_start();
        let end = self.previous().get_end();
        let s = &self.buf[start..end];
        Value::String(self.runtime.interner_mut().intern(s))
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
    pub fn variable(&mut self) {
        self.named_variable();
    }

    pub fn named_variable(&mut self) {
        let name = self.get_prev_token_string();
        let arg = self.resolve_local(name);

        let (x, y) = match arg {
            Some(arg) => (opcode::GET_LOCAL, arg),
            None => (opcode::GET_GLOBAL, self.identifier_constant().unwrap()),
        };

        self.emit_u8(x);
        self.emit_u32(y);
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
            Token::Identifier => self.variable(),
            Token::Prev => {} // do nothing
            _ => self.error("Expect expression"),
        }
    }
    pub fn begin_scope(&mut self) {
        self.current_compiler.begin_scope()
    }
    pub fn get_scope(&mut self) -> i8 {
        self.current_compiler.scope_depth()
    }
    pub fn resolve_local(&mut self, name: String) -> Option<u32> {
        self.current_compiler.resolve_local(name)
    }
    pub fn end_scope(&mut self) {
        self.current_compiler.scope_depth -= 1;
        while !self.current_compiler.locals.is_empty()
            && self.current_compiler.locals.last().unwrap().depth
                > self.current_compiler.scope_depth
        {
            self.current_compiler.locals.pop();
            self.emit_u8(opcode::POP_LOCAL)
        }
    }
    pub fn add_local(&mut self, name: String) {
        self.current_compiler.add_local(name);
    }
    pub fn emit_jump(&mut self, opcode: u8) -> usize {
        self.emit_u8(opcode);
        self.emit_u32(0);
        self.current_chunk().len() - 4
    }
    pub fn patch_jump(&mut self, patch_index: usize) {
        let jump = self.current_chunk().len() - patch_index - 4;

        let jump = match u32::try_from(jump) {
            Ok(jump) => jump,
            Err(_) => {
                self.error("Too many jumps in one chunk.");
                0
            }
        };

        self.set_u32(patch_index, jump);
    }
    pub fn emit_loop(&mut self, loop_start: usize) {
        let offset = self.current_chunk().len() - loop_start + 4;

        let offset = match u32::try_from(offset) {
            Ok(offset) => offset,
            Err(_) => {
                self.error("Too many jumps in one chunk.");
                0
            }
        };

        self.emit_u8(opcode::LOOP);
        self.emit_u32(offset);
    }
    pub fn get_prev_token_string(&self) -> String {
        let token = self.previous();
        String::from(self.pick_str(token))
    }
    pub fn define_local_variable(&mut self, name: &str) {
        self.add_local(name.to_string());
        self.emit_u8(opcode::DEFINE_LOCAL);
        self.emit_u8(0);
        self.emit_u8(opcode::POP);
    }
}
