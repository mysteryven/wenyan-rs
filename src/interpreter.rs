use std::collections::HashMap;

use crate::{
    chunk::Chunk,
    compiler::Parser,
    interner::Interner,
    object::Function,
    vm::{VMMode, VM},
};

pub enum InterpretStatus {
    CompilerError,
    RuntimeError,
    Ok,
}

pub fn interpret(buf: &str, mode: VMMode) {
    let chunk = Chunk::new();
    let mut runtime = Runtime::new();

    let mut compiler = Parser::new(buf, &mut runtime);

    if !compiler.compile() {
        let mut vm = VM::new(&chunk, chunk.code().as_ptr(), &mut runtime);
        vm.run(mode);

        vm.free();
        std::process::exit(0);
    } else {
        // std::process::exit(1);
    }
}

pub struct Runtime {
    interner: Interner,
    functions: HashMap<u32, Function>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            interner: Interner::new(),
            functions: HashMap::new(),
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
    pub fn add_function(&mut self, func: Function) -> u32 {
        let id = self.functions.len() as u32;
        self.functions.insert(id, func);
        id
    }
    pub fn get_function(&self, id: &u32) -> &Function {
        self.functions.get(id).expect("Function not found.")
    }
}
