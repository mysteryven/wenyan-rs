use crate::{chunk::Chunk, debug::Debugger, interpreter::InterpretStatus, opcode, value::Value};

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: *const u8,
    stack: Vec<Value>,
}

type BinaryOp = fn(f64, f64) -> f64;

// TODO optimize binary_op not use self.binary_op
macro_rules! binary_op {
    ($self:ident, $op:tt) => {
        {
            $self.binary_op(|a, b| a $op b);
        }
    }
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk, ip: *const u8) -> Self {
        Self {
            chunk,
            ip,
            stack: vec![],
        }
    }
    pub fn offset(&self) -> usize {
        unsafe { self.ip.offset_from(self.chunk.code().as_ptr()) as usize }
    }
    pub fn show_stack(&self) {
        println!("   ");
        for val in self.stack.iter() {
            print!("[{:?}]", val);
        }
        println!("")
    }
    pub fn run(&mut self) -> InterpretStatus {
        let debugger = Debugger::new(self.chunk);
        let mut result = vec![];

        loop {
            self.show_stack();
            debugger.disassemble_instruction(&mut result, self.offset());
            match self.read_byte() {
                opcode::CONSTANT => {
                    if let Some(value) = self.read_constant() {
                        self.stack.push(value.clone())
                    }
                }
                opcode::ADD => binary_op!(self, +),
                opcode::SUBTRACT => binary_op!(self, -),
                opcode::MULTIPLY => binary_op!(self, *),
                opcode::RETURN => return InterpretStatus::Ok,
            }
        }
    }
    fn read_byte(&mut self) -> u8 {
        unsafe {
            let value = std::ptr::read(self.ip);
            self.ip = self.ip.add(1);
            value
        }
    }
    fn read_u32(&mut self) -> u32 {
        unsafe {
            let slice = &*std::ptr::slice_from_raw_parts(self.ip, 4);
            let value = u32::from_le_bytes(slice.try_into().unwrap());
            self.ip = self.ip.add(4);
            value
        }
    }
    fn print_value(&self, value: &Value) {}
    fn read_constant(&mut self) -> Option<&Value> {
        self.chunk.constants().get(self.read_u32() as usize)
    }
    fn binary_op(&mut self, op: BinaryOp) -> bool {
        let slice_start = self.stack.len() - 2;

        match &self.stack[slice_start..] {
            [Value::Number(a), Value::Number(b)] => {
                let value = op(*a, *b);
                self.stack.pop();
                self.stack.pop();
                self.stack.push(Value::Number(value));
                true
            }
            _ => false,
        }
    }
}
