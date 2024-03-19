use crate::{
    chunk::{self, Chunk, OpCode},
    compiler::compile,
    debug::{disassemble_chunk, disassemble_instruction},
    value::{print_value, Value},
};
const STACK_MAX: usize = 256;

pub struct VM<'a> {
    pub chunk: Option<&'a Chunk>,
    ip: usize,
    stack: Vec<Value>,
    stack_top: usize,
}

#[derive(Debug)]
pub enum InterpretResultError {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

impl<'a> VM<'a> {
    pub fn init_vm(&mut self) {
        self.reset_stack();
    }
    fn reset_stack(&mut self) {
        self.stack_top = 0;
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack.pop().unwrap()
    }
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
            stack_top: 0,
        }
    }

    pub fn ip(&self) -> u8 {
        self.chunk.unwrap().code[self.ip]
    }

    pub fn interpret(
        &mut self,
        source: String,
        chunk: &'a mut Chunk,
    ) -> Result<InterpretResultError, InterpretResultError> {
        // let mut chunk = Chunk::init_chunk();

        if compile(source, chunk) {
            return Err(InterpretResultError::INTERPRET_COMPILE_ERROR);
        }

        self.chunk = Some(chunk);
        self.ip = self.chunk.unwrap().code[0] as usize;

        let result = self.run();

        return result;
    }

    pub fn run(&mut self) -> Result<InterpretResultError, InterpretResultError> {
        macro_rules! binary_op {
            ($op:tt) => {
                let b:f64 = self.pop();
                let a:f64 = self.pop();
                self.push(a $op b);
            };
        }
        #[allow(clippy::never_loop)]
        loop {
            #[cfg(feature = "debug")]
            {
                for slot in &self.stack {
                    print!("[ ");
                    print_value(slot);
                    print!(" ]")
                }
                disassemble_instruction(self.chunk.unwrap(), &(self.ip as usize));
            }

            let instruction = self.chunk.unwrap().code[self.ip];
            self.ip += 1;
            if let Ok(instruction) = instruction.try_into() {
                match instruction {
                    OpCode::OP_RETURN => {
                        print_value(&self.pop());
                        println!();
                        return Ok(InterpretResultError::INTERPRET_OK);
                    }
                    OpCode::OP_NEGATE => {
                        let val = self.pop();
                        self.push(-val);
                    }
                    OpCode::OP_ADD => {
                        binary_op! {+};
                    }
                    OpCode::OP_SUBTRACT => {
                        binary_op! {-};
                    }
                    OpCode::OP_DIVIDE => {
                        binary_op! {/};
                    }
                    OpCode::OP_MULTIPLY => {
                        binary_op! {*};
                    }
                    OpCode::OP_CONSTANT => {
                        let constant = self.chunk.unwrap().constants.values[self.ip() as usize];
                        self.push(constant);
                        self.ip += 1;
                    }
                }
            } else {
                return Err(InterpretResultError::INTERPRET_COMPILE_ERROR);
            };
        }
    }
}
