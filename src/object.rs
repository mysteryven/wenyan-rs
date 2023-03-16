use std::rc::Rc;

use crate::{chunk::Chunk, value::Value};

pub type FunId = u32;
pub type ClosureId = u32;

pub struct Function {
    arity: usize,
    chunk: Chunk,
    name: String,
    upvalues_count: u32,
}

impl Function {
    pub fn new() -> Self {
        Self {
            arity: usize::default(),
            chunk: Chunk::new(),
            name: String::default(),
            upvalues_count: u32::default(),
        }
    }
    pub fn upvalues_count(&self) -> u32 {
        self.upvalues_count
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
    pub fn up_values_count(&self) -> u32 {
        self.upvalues_count
    }
    pub fn plus_upvalues_count(&mut self) {
        self.upvalues_count = self.upvalues_count + 1;
    }
}

#[derive(Debug, Clone)]
pub struct ObjUpValue {
    location: Rc<Value>,
}

impl ObjUpValue {
    pub fn new(location: Rc<Value>) -> Self {
        Self { location }
    }
    pub fn location(&self) -> &Rc<Value> {
        &self.location
    }
    pub fn location_mut(&mut self) -> &mut Rc<Value> {
        &mut self.location
    }
}

impl Default for ObjUpValue {
    fn default() -> Self {
        Self {
            location: Rc::new(Value::Nil),
        }
    }
}

pub struct Closure {
    function: Function,
    up_values: Vec<ObjUpValue>,
    up_values_count: u32,
}

impl Closure {
    pub fn new(function: Function) -> Self {
        let up_values = vec![];

        for i in 0..function.upvalues_count() {
            up_values.push(ObjUpValue::default());
        }

        Self {
            function,
            up_values,
            up_values_count: function.upvalues_count(),
        }
    }
    pub fn function(&self) -> &Function {
        &self.function
    }
    pub fn up_values_count(&self) -> u32 {
        self.up_values_count
    }
    pub fn set_up_value(&mut self, i: usize, up_value: ObjUpValue) {
        self.up_values[i] = up_value;
    }
    pub fn get_up_values(&self, i: usize) -> ObjUpValue {
        self.up_values[i].clone()
    }
}
