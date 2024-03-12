use crate::value::{Value, ValueArray};

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OP_CONSTANT,
    OP_RETURN,
}

impl TryFrom<u8> for OpCode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const OP_RETURN: u8 = OpCode::OP_RETURN as u8;
        const OP_CONSTANT: u8 = OpCode::OP_CONSTANT as u8;
        match value {
            OP_RETURN => Ok(OpCode::OP_RETURN),
            OP_CONSTANT => Ok(OpCode::OP_CONSTANT),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn init_chunk() -> Self {
        Self {
            code: Vec::new(),
            constants: ValueArray::init_value_array(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write_value_array(value);
        (self.constants.values.len() - 1) as u8
    }
}
