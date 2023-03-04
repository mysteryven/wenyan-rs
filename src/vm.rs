use std::collections::HashMap;

use crate::{
    chunk::Chunk,
    interner::StrId,
    interpreter::{CallFrame, InterpretStatus, Runtime},
    memory::free_object,
    object::Function,
    opcode,
    value::{is_falsy, is_less, value_equal, Value},
};

#[derive(Clone, Copy, PartialEq)]
pub enum VMMode {
    Debug,
    Run,
}

pub struct VM<'a> {
    stack: Vec<Value>,
    local_stack: Vec<Value>,
    runtime: &'a mut Runtime,
    globals: HashMap<String, Value>,
    break_points: Vec<*const u8>,
}

impl<'a> VM<'a> {
    pub fn new(runtime: &'a mut Runtime) -> Self {
        Self {
            stack: vec![],
            local_stack: vec![],
            runtime,
            globals: HashMap::new(),
            break_points: vec![],
        }
    }
    pub fn frame_mut(&mut self) -> &mut CallFrame {
        self.runtime.current_frame_mut()
    }
    pub fn frame(&self) -> &CallFrame {
        self.runtime.current_frame()
    }
    pub fn begin_frame(&mut self, ip: *const u8, slot_begin: usize, function: Function) {
        let fun_idx = self.runtime.begin_frame(ip, slot_begin, function);
        self.local_stack.push(Value::Function(fun_idx));
    }
    pub fn ip(&self) -> *const u8 {
        self.frame().ip()
    }
    pub fn set_ip(&mut self, ip: *const u8) {
        self.frame_mut().set_ip(ip)
    }
    pub fn add_ip(&mut self, offset: usize) {
        self.frame_mut().add_ip(offset);
    }
    pub fn sub_ip(&mut self, offset: usize) {
        self.frame_mut().sub_ip(offset);
    }
    pub fn chunk(&self) -> &Chunk {
        self.runtime.chunk()
    }
    pub fn normalize_slot(&self, slot: usize) -> usize {
        self.frame().slot_begin() + slot
    }
    pub fn offset(&self) -> usize {
        unsafe { self.ip().offset_from(self.chunk().code().as_ptr()) as usize }
    }
    pub fn stack(&self) -> &Vec<Value> {
        &self.stack
    }
    pub fn local_stack(&self) -> &Vec<Value> {
        &self.local_stack
    }
    pub fn push_local(&mut self, value: Value) {
        self.local_stack.push(value);
    }
    pub fn peek(&self, distance: usize) -> Option<&Value> {
        self.stack.get(self.stack.len() - 1 - distance)
    }
    pub fn show_stack(&self) {
        println!("  ");
        println!("  ");
        print!("Stack->");
        if self.stack.len() == 0 {
            print!("[]")
        }
        for val in self.stack.iter() {
            print!("[{:?}]", val);
        }

        println!("  ");
        print!("LocalStack->");
        if self.local_stack.len() == 0 {
            print!("[]")
        }
        for val in self.local_stack.iter() {
            print!("[{:?}]", val);
        }
        println!("  ")
    }
    pub fn run(&mut self) -> InterpretStatus {
        loop {
            let byte = self.read_byte();
            match byte {
                opcode::RETURN => {
                    return InterpretStatus::Ok;
                }
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
                opcode::POP => {
                    self.stack.pop();
                }
                opcode::ADD => self.binary_op("+"),
                opcode::SUBTRACT => self.binary_op("-"),
                opcode::MULTIPLY => self.binary_op("*"),
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
                opcode::DEFINE_GLOBAL => {
                    let str = self.read_string();
                    let offset = self.read_byte() as usize;
                    let value = self.peek(offset);
                    if let (Some(value), Some(str)) = (value, str) {
                        self.globals.insert(str, value.clone());
                    }
                }
                opcode::GET_GLOBAL => {
                    let str_id = self.read_str().expect("a valid str id");
                    let str = self.runtime.interner().lookup(str_id);
                    if let Some(value) = self.globals.get(str) {
                        self.stack.push(value.clone());
                    } else {
                        self.runtime_error(format!("undefined variable {}", str).as_str());
                        return InterpretStatus::RuntimeError;
                    }
                }
                opcode::SET_GLOBAL => {
                    let str_id = self.read_str().expect("a valid str id");
                    let str = self.runtime.interner().lookup(str_id);
                    let value = self.stack.pop();
                    if let Some(value) = value {
                        self.globals.insert(str.to_owned(), value);
                    } else {
                        self.runtime_error(format!("undefined variable {}", str).as_str());
                        return InterpretStatus::RuntimeError;
                    }
                }
                opcode::DEFINE_LOCAL => {
                    let offset = self.read_byte() as usize;
                    let value = self.peek(offset);
                    if let Some(value) = value {
                        self.local_stack.push(value.clone());
                    }
                }
                opcode::GET_LOCAL => {
                    let slot = self.read_u32() as usize;
                    let value = self.local_stack.get(self.normalize_slot(slot));
                    if let Some(value) = value {
                        self.stack.push(value.clone());
                    }
                }
                opcode::SET_LOCAL => {
                    let mut slot = self.read_u32() as usize;
                    slot = self.normalize_slot(slot);

                    self.local_stack.get_mut(slot).map(|x| {
                        let value = self.stack.pop();
                        if let Some(value) = value {
                            *x = value;
                        }
                    });
                }
                opcode::POP_LOCAL => {
                    self.local_stack.pop();
                }
                opcode::JUMP_IF_FALSE => {
                    let offset = self.read_u32();
                    let value = self.stack.last();
                    if let Some(value) = value {
                        if is_falsy(value) {
                            self.skip(offset, true)
                        }
                    }
                }
                opcode::JUMP => {
                    let offset = self.read_u32();
                    self.skip(offset, true);
                }
                opcode::AND => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();

                    if let (Some(a), Some(b)) = (a, b) {
                        let boolean = if !is_falsy(&a) && !is_falsy(&b) {
                            true
                        } else {
                            false
                        };

                        self.stack.push(Value::Bool(boolean))
                    }
                }
                opcode::OR => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();

                    if let (Some(a), Some(b)) = (a, b) {
                        let boolean = if !is_falsy(&a) || !is_falsy(&b) {
                            true
                        } else {
                            false
                        };

                        self.stack.push(Value::Bool(boolean))
                    }
                }
                opcode::LOOP => {
                    let offset = self.read_u32();
                    self.skip(offset, false);
                }
                opcode::BREAK => {
                    if let Some(ip) = self.break_points.last() {
                        self.set_ip(ip.clone());
                    } else {
                        self.runtime_error("no loop to break.");
                        return InterpretStatus::RuntimeError;
                    }
                }
                opcode::DISCARD_BREAK => {
                    self.break_points.pop();
                }
                opcode::RECORD_BREAK => {
                    let offset = self.read_u32();
                    let ip = unsafe { self.ip().add(offset as usize) };
                    self.break_points.push(ip);
                }
                _ => {}
            }
        }
    }
    pub fn free(&mut self) {
        self.stack.clear();
        free_object(self.runtime)
    }
    fn runtime_error(&mut self, msg: &str) {
        eprintln!(
            "[line {}] error: {}",
            self.chunk().get_line(self.offset()),
            msg
        );

        self.stack.clear();
    }
    pub fn read_string(&mut self) -> Option<String> {
        let idx = self.read_constant().map(|x| x.clone());

        match idx {
            Some(Value::String(i)) => Some(self.runtime.interner().lookup(i.clone()).to_owned()),
            _ => {
                self.runtime_error("not find this string in interner.");
                None
            }
        }
    }
    pub fn read_str(&mut self) -> Option<StrId> {
        let idx = self.read_constant().map(|x| x.clone());

        match idx {
            Some(Value::String(i)) => Some(i),
            _ => {
                self.runtime_error("not find this string in interner.");
                None
            }
        }
    }
    fn read_byte(&mut self) -> u8 {
        unsafe {
            let value = std::ptr::read(self.ip());
            self.add_ip(1);
            value
        }
    }
    fn read_u32(&mut self) -> u32 {
        unsafe {
            let slice = &*std::ptr::slice_from_raw_parts(self.ip(), 4);
            let value = u32::from_le_bytes(slice.try_into().unwrap());
            self.add_ip(4);
            value
        }
    }
    fn skip(&mut self, offset: u32, is_add: bool) {
        unsafe {
            if is_add {
                self.add_ip(offset as usize);
            } else {
                self.sub_ip((offset + 1) as usize) // self.ip is point to next opcode
            }
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
            Value::String(str) => {
                format!("{}", self.runtime.interner().lookup(*str))
            }
            Value::Function(idx) => {
                let name = self.runtime.get_function(idx).name();
                format!(
                    "<fn> {}",
                    if name == "" { "<global context>" } else { name }
                )
            }
        }
    }
    fn read_constant(&mut self) -> Option<&Value> {
        let id = self.read_u32() as usize;
        self.chunk().constants().get(id)
    }
    fn binary_op(&mut self, op: &str) {
        let slice_start = self.stack.len() - 2;
        let op_code = self.read_byte();

        let compute = |a, b| match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            _ => panic!("unreachable"),
        };

        match &self.stack[slice_start..] {
            [Value::Number(a), Value::Number(b)] => {
                let num = match op_code {
                    opcode::PREPOSITION_LEFT => compute(*b, *a),
                    opcode::PREPOSITION_RIGHT => compute(*a, *b),
                    _ => panic!("unreachable"),
                };
                self.stack.pop();
                self.stack.pop();
                self.stack.push(Value::Number(num));
            }
            [Value::String(a), Value::String(b)] => {
                if op == "+" {
                    let str = format!(
                        "{}{}",
                        self.runtime.interner().lookup(*a),
                        self.runtime.interner().lookup(*b)
                    );
                    self.stack.pop();
                    self.stack.pop();
                    let str_id = self.runtime.interner_mut().intern(&str);
                    self.stack.push(Value::String(str_id));
                } else {
                    self.runtime_error("two string can only be added");
                }
            }
            _ => {
                self.runtime_error("Operands must be numbers.");
            }
        }
    }
}
