use crate::{
    chunk::Chunk,
    debug::Debugger,
    interpreter::InterpretStatus,
    opcode,
    value::{is_less, value_equal, Value},
};

#[derive(Clone, Copy, PartialEq)]
pub enum VMMode {
    Debug,
    Run,
}

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
        println!("  ");
        for val in self.stack.iter() {
            print!("[{:?}]", val);
        }
        println!("  ")
    }
    pub fn run(&mut self, mode: VMMode) -> InterpretStatus {
        let debugger = Debugger::new(self.chunk);
        let mut result = vec![];

        loop {
            if mode == VMMode::Debug {
                self.show_stack();
                debugger.disassemble_instruction(&mut result, self.offset());
            }
            let byte = self.read_byte();
            match byte {
                opcode::CONSTANT => {
                    if let Some(value) = self.read_constant().map(|x| x.clone()) {
                        self.stack.push(value);
                    }
                }
                opcode::PRINT => {
                    let vec_str = self
                        .stack
                        .iter()
                        .map(|x| self.format_value(x))
                        .collect::<Vec<String>>();

                    println!("{}", vec_str.join(" "));
                    self.stack.clear();
                }
                opcode::ADD => binary_op!(self, +),
                opcode::SUBTRACT => binary_op!(self, -),
                opcode::MULTIPLY => binary_op!(self, *),
                opcode::RETURN => return InterpretStatus::Ok,
                opcode::INVERT => {
                    let val = match self.stack.pop() {
                        Some(Value::Bool(false)) => Some(true),
                        None => {
                            self.runtime_error("not match expression after 變");
                            None
                        }
                        _ => Some(false),
                    };
                    if let Some(b) = val {
                        self.stack.push(Value::Bool(b))
                    }
                }
                opcode::TRUE => self.stack.push(Value::Bool(true)),
                opcode::FALSE => self.stack.push(Value::Bool(false)),
                opcode::EQUAL_EQUAL => {
                    let right_operand = self.stack.pop();
                    let left_operand = self.stack.pop();
                    if let (Some(right_operand), Some(left_operand)) = (right_operand, left_operand)
                    {
                        self.stack
                            .push(Value::Bool(value_equal(left_operand, right_operand)))
                    }
                }
                opcode::LESS => {
                    let right_operand = self.stack.pop();
                    let left_operand = self.stack.pop();
                    if let (Some(right_operand), Some(left_operand)) = (right_operand, left_operand)
                    {
                        self.stack
                            .push(Value::Bool(is_less(left_operand, right_operand)))
                    }
                }
                opcode::GREATER => {
                    let right_operand = self.stack.pop();
                    let left_operand = self.stack.pop();
                    if let (Some(right_operand), Some(left_operand)) = (right_operand, left_operand)
                    {
                        self.stack
                            .push(Value::Bool(is_less(right_operand, left_operand)))
                    }
                }
                _ => {}
            }
        }
    }
    fn runtime_error(&mut self, msg: &str) {
        eprintln!(
            "[line {}]errors: {}",
            self.chunk.get_line(self.offset()),
            msg
        );

        self.stack.clear();
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
    fn format_value(&self, value: &Value) -> String {
        match value {
            Value::Bool(boolean) => {
                format!("{}", boolean)
            }
            Value::Number(num) => {
                format!("{}", num)
            }
        }
    }
    fn read_constant(&mut self) -> Option<&Value> {
        self.chunk.constants().get(self.read_u32() as usize)
    }
    fn binary_op(&mut self, op: BinaryOp) -> bool {
        let slice_start = self.stack.len() - 2;
        let op_code = self.read_byte();

        match &self.stack[slice_start..] {
            [Value::Number(a), Value::Number(b)] => {
                let num = match op_code {
                    opcode::PREPOSITION_LEFT => op(*b, *a),
                    opcode::PREPOSITION_RIGHT => op(*a, *b),
                    _ => panic!("unreachable"),
                };
                self.stack.pop();
                self.stack.pop();
                self.stack.push(Value::Number(num));
                true
            }
            _ => {
                self.runtime_error("Operands must be numbers.");
                false
            }
        }
    }
}
