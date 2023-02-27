use crate::{chunk::Chunk, opcode};

pub struct Debugger<'a> {
    chunk: &'a Chunk,
}

impl<'a> Debugger<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self { chunk }
    }
    pub fn disassemble(&self, name: &str) -> Vec<String> {
        let mut result = vec![];
        print!("== {} ==", name);
        let mut offset = 0;
        loop {
            if offset >= self.chunk.code().len() {
                break;
            }

            offset = self.disassemble_instruction(&mut result, offset);
        }

        result
    }
    pub fn disassemble_instruction(&self, result: &mut Vec<String>, offset: usize) -> usize {
        let mut opcode_metadata = String::new();
        print!("{:08}", offset);
        if offset > 0 && self.chunk.get_line(offset) == self.chunk.get_line(offset - 1) {
            print!(" {:<4}", "|")
        } else {
            print!(" {:<4}", self.chunk.get_line(offset))
        };

        let op_code = self.chunk.code().get(offset).unwrap().clone();

        let new_offset = match op_code {
            opcode::RETURN => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_RETURN")
            }
            opcode::CONSTANT => {
                self.constant_instruction(&mut opcode_metadata, offset, "OP_CONSTANT")
            }
            opcode::ADD => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_ADD")
            }
            opcode::SUBTRACT => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_SUBTRACT")
            }
            opcode::MULTIPLY => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_MULTIPLY")
            }
            opcode::TRUE => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_TRUE")
            }
            opcode::FALSE => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_FALSE")
            }
            opcode::INVERT => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_INVERT")
            }
            opcode::EQUAL_EQUAL => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_EQUAL_EQUAL")
            }
            opcode::GREATER => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_GREATER")
            }
            opcode::LESS => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_LESS")
            }
            opcode::DEFINE_GLOBAL => self.constant_global_variable_instruction(
                &mut opcode_metadata,
                offset,
                "OP_DEFINE_GLOBAL",
            ),
            opcode::GET_GLOBAL => {
                self.constant_instruction(&mut opcode_metadata, offset, "OP_GET_GLOBAL")
            }
            opcode::SET_GLOBAL => {
                self.constant_instruction(&mut opcode_metadata, offset, "OP_SET_GLOBAL")
            }
            opcode::PRINT => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_Print")
            }
            _ => {
                // this is a unknown opcode
                print!("{:<20}", format!("{}({})", op_code, "unknown").as_str());
                offset + 1
            }
        };

        result.push(opcode_metadata);

        new_offset
    }
    pub fn disassemble_simple_instruction(
        &self,
        _opcode_metadata: &mut String,
        offset: usize,
        name: &str,
    ) -> usize {
        print!(" {:<20}", name);

        offset + 1
    }

    pub fn constant_instruction(&self, _line: &mut String, offset: usize, name: &str) -> usize {
        print!(" {:<20}", name);
        let constant = self.chunk.get_u32(offset + 1);
        let value = self.chunk.constants().get(constant as usize).unwrap();
        print!(" {:08} {:?}", constant, value);

        offset + 5
    }

    pub fn constant_global_variable_instruction(
        &self,
        _line: &mut String,
        offset: usize,
        name: &str,
    ) -> usize {
        print!(" {:<20}", name);
        let constant = self.chunk.get_u32(offset + 1);
        let distance = self.chunk.get_u8(offset + 5);
        let value = self.chunk.constants().get(constant as usize).unwrap();
        print!(" {:08} {:?} peek({})", constant, value, distance);

        offset + 6
    }
}
