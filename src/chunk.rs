use crate::opcode;

struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: vec![] }
    }
    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }
    pub fn disassemble(&self, name: &str) -> Vec<String> {
        let mut result = vec![];
        result.push(format!("== {} ==", name));

        let mut offset = 0;
        loop {
            if offset >= self.code.len() {
                break;
            }

            offset = self.disassemble_instruction(&mut result, offset);
        }

        result
    }
    pub fn disassemble_instruction(&self, result: &mut Vec<String>, offset: usize) -> usize {
        let mut line = String::new();
        line.push_str(format!("{:08}", offset).as_str());

        let op_code = self.code.get(offset).unwrap().clone();

        let new_offset = match op_code {
            opcode::RETURN => self.disassemble_simple_instruction(&mut line, offset, "OP_Return"),
            _ => {
                // this is a unknown opcode
                line.push_str(format!("{:>16}", format!("{}({})", op_code, "unknown")).as_str());
                offset + 1
            }
        };

        result.push(line);

        new_offset
    }
    pub fn disassemble_simple_instruction(
        &self,
        result: &mut String,
        offset: usize,
        name: &str,
    ) -> usize {
        result.push_str(format!("{:>16}", name).as_str());

        offset + 1
    }
}

#[cfg(test)]
mod test {
    use insta::{assert_snapshot, assert_yaml_snapshot};

    use crate::opcode;

    use super::Chunk;

    #[test]
    fn disassemble_simple_ins() {
        let mut chunk = Chunk::new();
        chunk.write(opcode::RETURN);
        let strs = chunk.disassemble("test");

        assert_yaml_snapshot!(strs, @r###"
        ---
        - "== test =="
        - 00000000       OP_Return
        "###);
    }

    #[test]
    fn disassemble_unknown_ins() {
        let mut chunk = Chunk::new();
        chunk.write(255);
        chunk.write(opcode::RETURN);
        let strs = chunk.disassemble("test");
        assert_yaml_snapshot!(strs, @r###"
        ---
        - "== test =="
        - 00000000    255(unknown)
        - 00000001       OP_Return
        "###)
    }
}
