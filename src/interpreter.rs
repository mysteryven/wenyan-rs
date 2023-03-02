use crate::{
    chunk::Chunk,
    compiler::Parser,
    interner::Interner,
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

    if !compiler.compile() {
        let mut vm = VM::new(&chunk, chunk.code().as_ptr(), &mut runtime);
        vm.run(mode);

        vm.free();
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

pub struct Runtime {
    interner: Interner,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            interner: Interner::new(),
        }
    }
    pub fn interner(&self) -> &Interner {
        &self.interner
    }
    pub fn interner_mut(&mut self) -> &mut Interner {
        &mut self.interner
    }
    pub fn free(&mut self) {
        self.interner.free()
    }
}
