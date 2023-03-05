use std::collections::HashMap;

use crate::{
    chunk::Chunk,
    compiler::Parser,
    interner::Interner,
    object::{FunId, Function},
    vm::{VMMode, VM},
};

pub enum InterpretStatus {
    CompilerError,
    RuntimeError,
    Ok,
}

pub fn interpret(buf: &str, mode: VMMode) -> InterpretStatus {
    let mut runtime = Runtime::new();
    let mut compiler = Parser::new(buf, &mut runtime);

    if let Some(function) = compiler.compile() {
        let mut vm = VM::new(&mut runtime);
        let idx = vm.add_function(function);
        vm.setup_first_frame(idx);
        let ok = vm.run(mode);
        vm.free();

        ok
    } else {
        return InterpretStatus::CompilerError;
    }
}

pub struct Runtime {
    interner: Interner,
    functions: HashMap<u32, Function>,
    frames: Vec<CallFrame>,
    current_frame: *mut CallFrame,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            interner: Interner::new(),
            functions: HashMap::new(),
            frames: vec![],
            current_frame: std::ptr::null_mut(),
        }
    }
    pub fn frames(&self) -> &Vec<CallFrame> {
        &self.frames
    }
    pub fn current_frame(&self) -> &CallFrame {
        unsafe { &*self.current_frame }
    }
    pub fn current_frame_mut(&self) -> &mut CallFrame {
        unsafe { &mut *self.current_frame }
    }
    pub fn chunk(&self) -> &Chunk {
        self.get_function(&self.current_frame().fun_id()).chunk()
    }
    pub fn current_chunk(&self) -> &Chunk {
        self.get_function(&self.current_frame().fun_id()).chunk()
    }
    pub fn begin_frame(
        &mut self,
        fun_idx: FunId,
        slot_begin: usize,
        local_slot_begin: usize,
    ) -> u32 {
        let function = self.get_function(&fun_idx);
        let ip = function.chunk().code().as_ptr();
        let frame = CallFrame::new(ip, fun_idx, slot_begin, local_slot_begin);

        self.frames.push(frame);

        unsafe {
            self.current_frame = self.frames.as_mut_ptr().add(self.frames.len() - 1);
        }
        fun_idx
    }
    pub fn exit_frame(&mut self) {
        if self.frames.pop().is_some() {
            unsafe {
                self.current_frame = self.current_frame.offset(-1);
            }
        } else {
            eprint!("not frame to exit.");
        }
    }

    pub fn add_function(&mut self, func: Function) -> u32 {
        let id = self.functions.len() as u32;
        self.functions.insert(id, func);
        id
    }
    pub fn get_function(&self, id: &u32) -> &Function {
        self.functions.get(id).expect("Function not found.")
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
}

pub struct CallFrame {
    ip: *const u8,
    fun_id: FunId,
    slot_begin: usize,
    local_slot_begin: usize,
}

impl CallFrame {
    pub fn new(ip: *const u8, fun_id: FunId, slot_begin: usize, local_slot_begin: usize) -> Self {
        Self {
            ip,
            fun_id,
            slot_begin,
            local_slot_begin,
        }
    }
    pub fn set_ip(&mut self, ip: *const u8) {
        self.ip = ip;
    }
    pub fn add_ip(&mut self, offset: usize) {
        unsafe { self.ip = self.ip.add(offset) }
    }
    pub fn sub_ip(&mut self, offset: usize) {
        unsafe { self.ip = self.ip.sub(offset) }
    }
    pub fn ip(&self) -> *const u8 {
        self.ip
    }
    pub fn fun_id(&self) -> FunId {
        self.fun_id
    }
    pub fn slot_begin(&self) -> usize {
        self.slot_begin
    }
    pub fn local_slot_begin(&self) -> usize {
        self.local_slot_begin
    }
}
