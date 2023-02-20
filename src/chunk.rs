use crate::{value::Value};

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<(usize, usize)>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }
    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }
    pub fn constants(&self) -> &Vec<Value> {
        &self.constants
    }
    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.save_line(line);
    }
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
    pub fn add_u8(&mut self, value: u8, line: usize) {
        self.write(value, line);
    }
    pub fn add_u32(&mut self, value: u32, line: usize) {
        let bytes = value.to_le_bytes();
        for i in 0..4 {
            self.write(bytes[i], line)
        }
    }

    pub fn get_u8(&self, idx: usize) -> u8 {
        unsafe { *self.code.get_unchecked(idx) }
    }

    pub fn get_u32(&self, idx: usize) -> u32 {
        let bytes = unsafe { self.code.get_unchecked(idx..idx + 4) };
        u32::from_le_bytes(bytes.try_into().unwrap())
    }

    pub fn get_line(&self, index: usize) -> usize {
        let mut total = 0;
        for (line, count) in &self.lines {
            total += count;
            if total > index {
                return *line;
            }
        }

        0
    }

    pub fn save_line(&mut self, line: usize) {
        match self.lines.last_mut() {
            Some((current_line_num, count)) if *current_line_num == line => *count += 1,
            _ => {
                self.lines.push((line, 1));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_yaml_snapshot;

    use crate::{debug::Debugger, opcode, value::Value};

    use super::Chunk;

    #[test]
    fn disassemble_simple_ins() {
        let mut chunk = Chunk::new();
        chunk.write(opcode::RETURN, 0);
        let debugger = Debugger::new(&chunk);
        let list = debugger.disassemble("test");

        assert_yaml_snapshot!(list, @r###"
        ---
        - ""
        "###);
    }

    #[test]
    fn disassemble_unknown_ins() {
        let mut chunk = Chunk::new();
        chunk.write(255, 0);
        chunk.write(opcode::RETURN, 1);
        let debugger = Debugger::new(&chunk);
        let list = debugger.disassemble("test");
        assert_yaml_snapshot!(list, @r###"
        ---
        - ""
        - ""
        "###)
    }

    #[test]
    fn disassemble_constant() {
        let mut chunk = Chunk::new();
        chunk.write(opcode::RETURN, 0);
        chunk.write(opcode::CONSTANT, 0);
        let idx = chunk.add_constant(Value::Number(1.2)) as u32;
        chunk.add_u32(idx, 0);
        chunk.write(opcode::RETURN, 1);
        let debugger = Debugger::new(&chunk);
        let list = debugger.disassemble("test");
        println!("{:?}", list);

        assert_yaml_snapshot!(list, @r###"
        ---
        - ""
        - ""
        - ""
        "###);
    }
}
