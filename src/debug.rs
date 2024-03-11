use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, &offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: &usize) -> usize {
    print!("{:#04} ", offset);

    match chunk.code[*offset].into() {
        OpCode::OP_RETURN => simple_instruction("OP_RETURN", offset),
        _ => {
            println!("Unknown opcode");
            offset + 1
        }
    }
}

fn simple_instruction(name: &str, offset: &usize) -> usize {
    println!("{}", name);
    offset + 1
}
