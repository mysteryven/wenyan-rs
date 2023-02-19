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
        result.push(format!("== {} ==", name));
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
        opcode_metadata.push_str(format!("{:08}", offset).as_str());
        if offset > 0 && self.chunk.get_line(offset) == self.chunk.get_line(offset - 1) {
            opcode_metadata.push_str(format!(" {:<4}", "|").as_str())
        } else {
            opcode_metadata.push_str(format!(" {:<4}", self.chunk.get_line(offset)).as_str())
        };

        let op_code = self.chunk.code().get(offset).unwrap().clone();

        let new_offset = match op_code {
            opcode::RETURN => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_Return")
            }
            opcode::CONSTANT => {
                self.constant_instruction(&mut opcode_metadata, offset, "OP_Constant")
            }
            opcode::ADD => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_Add")
            }
            opcode::SUBTRACT => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_Subtract")
            }
            opcode::MULTIPLY => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_Multiply")
            }
            opcode::TRUE => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_True")
            }
            opcode::FALSE => {
                self.disassemble_simple_instruction(&mut opcode_metadata, offset, "OP_False")
            }
            _ => {
                // this is a unknown opcode
                opcode_metadata
                    .push_str(format!("{:<20}", format!("{}({})", op_code, "unknown")).as_str());
                offset + 1
            }
        };

        result.push(opcode_metadata);

        new_offset
    }
    pub fn disassemble_simple_instruction(
        &self,
        opcode_metadata: &mut String,
        offset: usize,
        name: &str,
    ) -> usize {
        opcode_metadata.push_str(format!(" {:<20}", name).as_str());

        offset + 1
    }

    pub fn constant_instruction(&self, line: &mut String, offset: usize, name: &str) -> usize {
        line.push_str(format!(" {:<20}", name).as_str());
        let constant = self.chunk.get_u32(offset + 1);
        let value = self.chunk.constants().get(constant as usize).unwrap();
        line.push_str(format!(" {:08} {:?}", constant, value).as_str());

        offset + 5
    }
}
