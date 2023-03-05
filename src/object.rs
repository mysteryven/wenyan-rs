use crate::chunk::Chunk;

pub type FunId = u32;

pub struct Function {
    arity: usize,
    chunk: Chunk,
    name: String,
}

impl Function {
    pub fn new() -> Self {
        Self {
            arity: usize::default(),
            chunk: Chunk::new(),
            name: String::default(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }
    pub fn chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
    }
    pub fn add_arity(&mut self, num: usize) {
        self.arity = self.arity + num;
    }
}
