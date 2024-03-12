use crate::chunk::Chunk;

pub struct VM<'a> {
    chunk: &'a Chunk,
}

impl<'a> VM<'a> {
    fn init_vm(chunk: &'a Chunk) -> Self {
        Self { chunk }
    }
}
