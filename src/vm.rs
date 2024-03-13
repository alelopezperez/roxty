use crate::{
    chunk::{Chunk, OpCode},
    value::print_value,
};

pub struct VM<'a> {
    pub chunk: Option<&'a Chunk>,
    ip: usize,
}

#[derive(Debug)]
pub enum InterpretResultError {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

impl<'a> VM<'a> {
    pub fn init_vm() -> Self {
        Self { chunk: None, ip: 0 }
    }

    pub fn interpret(
        &mut self,
        chunk: &'a mut Chunk,
    ) -> Result<InterpretResultError, InterpretResultError> {
        self.chunk = Some(chunk);
        // ip to curent code in chunk
        self.run()
    }

    pub fn run(&mut self) -> Result<InterpretResultError, InterpretResultError> {
        #[allow(clippy::never_loop)]
        loop {
            let instruction = self.chunk.unwrap().code[self.ip];
            self.ip += 1;
            if let Ok(instruction) = instruction.try_into() {
                match instruction {
                    OpCode::OP_RETURN => {
                        println!("HERE");
                        return Ok(InterpretResultError::INTERPRET_OK);
                    }
                    OpCode::OP_CONSTANT => {
                        let value: f64 = self.chunk.unwrap().constants.values
                            [self.chunk.unwrap().code[self.ip] as usize];
                        print_value(&value);
                        println!();
                        self.ip += 1;
                    }
                }
            } else {
                return Err(InterpretResultError::INTERPRET_COMPILE_ERROR);
            };
        }
    }
}
