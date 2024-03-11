use std::u8;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Change,
    OP_RETURN,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        const OP_RETURN: u8 = OpCode::OP_RETURN as u8;
        match value {
            OP_RETURN => OpCode::OP_RETURN,
            _ => {
                panic!("NOT ")
            }
        }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
}

impl Chunk {
    pub fn init_chunk() -> Self {
        Self { code: Vec::new() }
    }

    pub fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
    }
}
