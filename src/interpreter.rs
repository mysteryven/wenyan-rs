use crate::{
    chunk::Chunk,
    compiler::Parser,
    interner::{self, Interner},
    vm::{VMMode, VM},
};

pub enum InterpretStatus {
    CompilerError,
    RuntimeError,
    Ok,
}

pub fn interpret(buf: &str, mode: VMMode) {
    let mut chunk = Chunk::new();
    let mut runtime = Runtime::new();

    let mut compiler = Parser::new(buf, &mut chunk, &mut runtime);

    compiler.compile();

    let mut vm = VM::new(&chunk, chunk.code().as_ptr(), &runtime);
    vm.run(mode);
}

pub struct Runtime {
    interner: Interner,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            interner: Interner::with_capacity(100),
        }
    }
    pub fn interner(&self) -> &Interner {
        &self.interner
    }
    pub fn interner_mut(&mut self) -> &mut Interner {
        &mut self.interner
    }
}
