#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OP_RETURN,
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
