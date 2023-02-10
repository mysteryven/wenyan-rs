use crate::{opcode, value::Value};

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
        }
    }
    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }
    pub fn constants(&self) -> &Vec<Value> {
        &self.constants
    }
    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
    pub fn add_u8(&mut self, value: u8) {
        self.write(value)
    }
    pub fn add_u32(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        for i in 0..4 {
            self.code.push(bytes[i]);
        }
    }

    pub fn get_u8(&self, idx: usize) -> u8 {
        unsafe { *self.code.get_unchecked(idx) }
    }

    pub fn get_u32(&self, idx: usize) -> u32 {
        let bytes = unsafe { self.code.get_unchecked(idx..idx + 4) };
        u32::from_le_bytes(bytes.try_into().unwrap())
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
        chunk.write(opcode::RETURN);
        let debugger = Debugger::new(&chunk);
        let list = debugger.disassemble("test");

        assert_yaml_snapshot!(list, @r###"
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
        let debugger = Debugger::new(&chunk);
        let list = debugger.disassemble("test");
        assert_yaml_snapshot!(list, @r###"
        ---
        - "== test =="
        - 00000000    255(unknown)
        - 00000001       OP_Return
        "###)
    }

    fn create_opcodes(list: Vec<u8>, name: &str) -> Vec<String> {
        let mut chunk = Chunk::new();
        for op_code in list {
            chunk.write(op_code);
        }
        let debugger = Debugger::new(&chunk);
        debugger.disassemble(name)
    }

    #[test]
    fn disassemble_constant() {
        let mut chunk = Chunk::new();
        chunk.write(opcode::CONSTANT);
        let idx = chunk.add_constant(Value::Number(1.2)) as u32;
        chunk.add_u32(idx);
        chunk.write(opcode::RETURN);
        let debugger = Debugger::new(&chunk);
        let list = debugger.disassemble("test");

        assert_yaml_snapshot!(list, @r###"
        ---
        - "== test =="
        - 00000000     OP_Constant 00000000 Number(1.2)
        - 00000005       OP_Return
        "###);
    }
}
