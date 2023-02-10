use crate::{chunk::Chunk, compiler::Parser, vm::VM};

pub enum InterpretStatus {
    CompilerError,
    RuntimeError,
    Success,
}

pub fn interpret(buf: &str) {
    let mut chunk = Chunk::new();

    let compiler = Parser::new(buf, &mut chunk);

    compiler.compile();

    let vm = VM::new(&chunk, chunk.code());
    vm.run();
}
