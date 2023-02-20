use crate::{
    chunk::Chunk,
    compiler::Parser,
    vm::{VMMode, VM},
};

pub enum InterpretStatus {
    CompilerError,
    RuntimeError,
    Ok,
}

pub fn interpret(buf: &str, mode: VMMode) {
    let mut chunk = Chunk::new();

    let mut compiler = Parser::new(buf, &mut chunk);

    compiler.compile();

    let mut vm = VM::new(&chunk, chunk.code().as_ptr());
    vm.run(mode);
}
