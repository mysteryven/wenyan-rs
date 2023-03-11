use crate::chunk::Chunk;

pub type FunId = u32;
pub type ClosureId = u32;

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
    pub fn arity(&self) -> usize {
        self.arity
    }
}

pub struct UpValue {
    pub index: usize,
    pub is_local: bool,
}

pub struct Closure {
    function: Function,
    up_values: Vec<UpValue>,
}

impl Closure {
    pub fn new(function: Function) -> Self {
        Self {
            function,
            up_values: Vec::new(),
        }
    }
    pub fn function(&self) -> &Function {
        &self.function
    }
    pub fn up_values(&self) -> &Vec<UpValue> {
        &self.up_values
    }
    pub fn up_values_mut(&mut self) -> &mut Vec<UpValue> {
        &mut self.up_values
    }
}
