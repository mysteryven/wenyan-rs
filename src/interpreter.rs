use crate::{chunk::Chunk, compiler::Parser, vm::VM};

pub enum InterpretStatus {
    CompilerError,
    RuntimeError,
    Ok,
}

pub fn interpret(buf: &str) {
    let mut chunk = Chunk::new();

    let mut compiler = Parser::new(buf, &mut chunk);

    compiler.compile();

    let mut vm = VM::new(&chunk, chunk.code().as_ptr());
    vm.run();
}

#[cfg(test)]
mod test {
    use super::interpret;

    #[test]
    fn test_binary_add() {
        interpret("加一以二")
    }

    #[test]
    fn test_bool() {
        interpret("陰");
    }

    #[test]
    fn test_invert() {
        interpret("變陰")
    }
}
