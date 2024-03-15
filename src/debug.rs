use crate::{
    chunk::{Chunk, OpCode},
    value::print_value,
};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, &offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: &usize) -> usize {
    print!("{:#04} ", offset);

    if *offset > 0 && chunk.lines[*offset] == chunk.lines[*offset - 1] {
        print!("   | ");
    } else {
        print!("{:>4} ", chunk.lines[*offset]);
    }

    if let Ok(chunk_instruction) = chunk.code[*offset].try_into() {
        match chunk_instruction {
            OpCode::OP_CONSTANT => constant_instruction("OP_CONSTANT", chunk, offset),
            OpCode::OP_NEGATE => simple_instruction("OP_NEGATE", offset),
            OpCode::OP_RETURN => simple_instruction("OP_RETURN", offset),
            OpCode::OP_ADD => simple_instruction("OP_ADD", offset),
            OpCode::OP_DIVIDE => simple_instruction("OP_DIVIDE", offset),
            OpCode::OP_MULTIPLY => simple_instruction("OP_MULTIPLY", offset),
            OpCode::OP_SUBTRACT => simple_instruction("OP_SUBTRACT", offset),
        }
    } else {
        println!("Unknown opcode {}", chunk.code[*offset]);
        offset + 1
    }
}

fn simple_instruction(name: &str, offset: &usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: &usize) -> usize {
    let constant = chunk.code[*offset + 1];
    print!("{:<16} {:>4} '", name, offset);
    print_value(&chunk.constants.values[constant as usize]);
    println!("'");
    *offset + 2
}
