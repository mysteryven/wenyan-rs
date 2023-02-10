use crate::{chunk::Chunk, interpreter::InterpretStatus};

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: &'a Vec<u8>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk, ip: &'a Vec<u8>) -> Self {
        Self { chunk, ip }
    }
    pub fn run(&mut self) -> InterpretStatus {
        InterpretStatus::Success
    }
}
