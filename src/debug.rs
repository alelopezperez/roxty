use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, &offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: &usize) -> usize {
    print!(" {:#04}", offset);

    match chunk.code[*offset] as OpCode {
        OpCode::OP_RETURN => {}
    }
    todo!()
}
