use crate::value::{Value, ValueArray};

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OP_CONSTANT,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_NEGATE,
    OP_RETURN,
}

impl TryFrom<u8> for OpCode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const OP_RETURN: u8 = OpCode::OP_RETURN as u8;
        const OP_CONSTANT: u8 = OpCode::OP_CONSTANT as u8;
        const OP_NEGATE: u8 = OpCode::OP_NEGATE as u8;
        const OP_ADD: u8 = OpCode::OP_ADD as u8;
        const OP_SUBTRACT: u8 = OpCode::OP_SUBTRACT as u8;
        const OP_MULTIPLY: u8 = OpCode::OP_MULTIPLY as u8;
        const OP_DIVIDE: u8 = OpCode::OP_DIVIDE as u8;

        match value {
            OP_RETURN => Ok(OpCode::OP_RETURN),
            OP_CONSTANT => Ok(OpCode::OP_CONSTANT),
            OP_NEGATE => Ok(OpCode::OP_NEGATE),
            OP_ADD => Ok(OpCode::OP_ADD),
            OP_SUBTRACT => Ok(OpCode::OP_SUBTRACT),
            OP_MULTIPLY => Ok(OpCode::OP_MULTIPLY),
            OP_DIVIDE => Ok(OpCode::OP_DIVIDE),

            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone)]
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
