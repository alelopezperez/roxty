use std::borrow::Borrow;

use crate::{
    chunk::{self, Chunk, OpCode},
    debug::disassemble_chunk,
    scanner::{self, Scanner, Token, TokenType},
    value::Value,
};

#[derive(Debug, Clone)]
enum Precedence {
    PREC_NONE,
    PREC_ASSIGNMENT, // =
    PREC_OR,         // or
    PREC_AND,        // and
    PREC_EQUALITY,   // == !=
    PREC_COMPARISON, // < > <= >=
    PREC_TERM,       // + -
    PREC_FACTOR,     // * /
    PREC_UNARY,      // ! -
    PREC_CALL,       // . ()
    PREC_PRIMARY,
}

impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        const PREC_NONE: u8 = Precedence::PREC_NONE as u8;
        const PREC_ASSIGNMENT: u8 = Precedence::PREC_ASSIGNMENT as u8;
        const PREC_OR: u8 = Precedence::PREC_OR as u8;
        const PREC_AND: u8 = Precedence::PREC_AND as u8;
        const PREC_EQUALITY: u8 = Precedence::PREC_EQUALITY as u8;
        const PREC_COMPARISON: u8 = Precedence::PREC_COMPARISON as u8;
        const PREC_TERM: u8 = Precedence::PREC_TERM as u8;
        const PREC_FACTOR: u8 = Precedence::PREC_FACTOR as u8;
        const PREC_UNARY: u8 = Precedence::PREC_UNARY as u8;
        const PREC_CALL: u8 = Precedence::PREC_CALL as u8;
        const PREC_PRIMARY: u8 = Precedence::PREC_PRIMARY as u8;

        match value {
            PREC_NONE => Precedence::PREC_NONE,
            PREC_ASSIGNMENT => Precedence::PREC_ASSIGNMENT,
            PREC_OR => Precedence::PREC_OR,
            PREC_AND => Precedence::PREC_AND,
            PREC_EQUALITY => Precedence::PREC_EQUALITY,
            PREC_COMPARISON => Precedence::PREC_COMPARISON,
            PREC_TERM => Precedence::PREC_TERM,
            PREC_FACTOR => Precedence::PREC_FACTOR,
            PREC_UNARY => Precedence::PREC_UNARY,
            PREC_CALL => Precedence::PREC_CALL,
            PREC_PRIMARY => Precedence::PREC_PRIMARY,
            _ => Precedence::PREC_NONE,
        }
    }
}

struct ParseRule {
    prefix: Option<fn(parser: &mut Parser, scanner: &mut Scanner, compiling_chunk: &mut Chunk)>,
    infix: Option<fn(parser: &mut Parser, scanner: &mut Scanner, compiling_chunk: &mut Chunk)>,
    precedence: Precedence,
}

const RULE: [(
    Option<fn(parser: &mut Parser, scanner: &mut Scanner, compiling_chunk: &mut Chunk)>,
    Option<fn(parser: &mut Parser, scanner: &mut Scanner, compiling_chunk: &mut Chunk)>,
    Precedence,
); 40] = [
    (Some(grouping), None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (Some(unary), Some(binary), Precedence::PREC_TERM),
    (None, Some(binary), Precedence::PREC_TERM),
    (None, None, Precedence::PREC_NONE),
    (None, Some(binary), Precedence::PREC_FACTOR),
    (None, Some(binary), Precedence::PREC_FACTOR),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (Some(number), None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
    (None, None, Precedence::PREC_NONE),
];

#[derive(Debug, Clone)]
pub struct Parser {
    previous: Option<Token>,
    current: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    fn advance(&mut self, scanner: &mut Scanner) {
        self.previous = self.current.clone();

        loop {
            self.current = Some(scanner.scan_token());
            if self.current.as_ref().unwrap().typo != TokenType::TOKEN_ERROR {
                break;
            }

            error_at_current(self.current.as_ref().unwrap().error.unwrap(), self, scanner);
        }
    }
}

pub fn compile(source: String, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::init_scanner(source);
    let mut compiling_chunk = chunk;

    let mut parser = Parser {
        previous: None,
        current: None,
        had_error: false,
        panic_mode: false,
    };
    parser.advance(&mut scanner);

    expression(&mut parser, &mut scanner, compiling_chunk);
    consume(
        TokenType::TOKEN_EOF,
        "Expected end of expression.",
        &mut parser,
        &mut scanner,
    );
    end_compiler(&mut parser, &mut compiling_chunk);
    parser.had_error
}

fn error_at_current(start: &str, parser: &mut Parser, scanner: &Scanner) {
    error_at(&parser.current.clone().unwrap(), start, scanner, parser);
}

fn error(current: &Token, message: &str, scanner: &Scanner, parser: &mut Parser) {
    error_at(current, message, scanner, parser);
}
fn error_at(token: &Token, message: &str, scanner: &Scanner, parser: &mut Parser) {
    if parser.panic_mode {
        return;
    }

    parser.panic_mode = true;

    eprint!("[line {}] Error", token.line);
    if token.typo == TokenType::TOKEN_EOF {
        eprint!(" at end");
    } else if token.typo == TokenType::TOKEN_ERROR {
    } else {
        eprint!(
            " at {}",
            &scanner.source[token.start..token.start + token.length]
        )
    }

    eprintln!("{}", message);
    parser.had_error = true;
    //parser.hadError = true;
}

fn consume(typo: TokenType, message: &str, parser: &mut Parser, scanner: &mut Scanner) {
    if parser.current.as_ref().unwrap().typo == typo {
        parser.advance(scanner);
        return;
    }
    error_at_current(message, parser, scanner);
}

fn emit_byte(byte: u8, parser: &Parser, compiling_chunk: &mut Chunk) {
    current_chunk(compiling_chunk).write_chunk(byte, parser.previous.as_ref().unwrap().line);
}

fn current_chunk(compiling_chunk: &mut Chunk) -> &mut Chunk {
    compiling_chunk
}

fn end_compiler(parser: &mut Parser, compiling_chunk: &mut Chunk) {
    emit_return(parser, compiling_chunk);

    #[cfg(feature = "debug")]
    {
        if !parser.had_error {
            disassemble_chunk(compiling_chunk, "code");
        }
    }
}

fn emit_return(parser: &mut Parser, compiling_chunk: &mut Chunk) {
    emit_byte(OpCode::OP_RETURN as u8, parser, compiling_chunk)
}

fn emit_bytes(byte_1: u8, byte_2: u8, parser: &Parser, compiling_chunk: &mut Chunk) {
    emit_byte(byte_1, parser, compiling_chunk);
    emit_byte(byte_2, parser, compiling_chunk);
}

fn expression(parser: &mut Parser, scanner: &mut Scanner, compiling_chunk: &mut Chunk) {
    parse_precedence(
        Precedence::PREC_ASSIGNMENT,
        parser,
        scanner,
        compiling_chunk,
    );
}

fn number(parser: &mut Parser, scanner: &mut Scanner, compiling_chunk: &mut Chunk) {
    let value: Value = (scanner.source[parser.previous.as_ref().unwrap().start
        ..parser.previous.as_ref().unwrap().start + parser.previous.as_ref().unwrap().length])
        .parse()
        .unwrap();
    emit_constant(value, parser, compiling_chunk, scanner);
}

fn grouping(parser: &mut Parser, scanner: &mut Scanner, compiling_chunk: &mut Chunk) {
    expression(parser, scanner, compiling_chunk);
    consume(
        TokenType::TOKEN_RIGHT_PAREN,
        "Expect ')' after expression.",
        parser,
        scanner,
    );
}

fn unary(parser: &mut Parser, scanner: &mut Scanner, compiling_chunk: &mut Chunk) {
    let operator_type = parser.previous.as_ref().unwrap().typo;

    parse_precedence(Precedence::PREC_UNARY, parser, scanner, compiling_chunk);

    match operator_type {
        TokenType::TOKEN_MINUS => emit_byte(OpCode::OP_NEGATE as u8, parser, compiling_chunk),
        _ => {}
    }
}

fn binary(parser: &mut Parser, scanner: &mut Scanner, compiling_chunk: &mut Chunk) {
    let operator_type = parser.previous.as_ref().unwrap().typo;
    let rule = get_rule(operator_type);
    parse_precedence(
        Precedence::from(rule.precedence as u8 + 1),
        parser,
        scanner,
        compiling_chunk,
    );

    match operator_type {
        TokenType::TOKEN_PLUS => emit_byte(OpCode::OP_ADD as u8, parser, compiling_chunk),
        TokenType::TOKEN_MINUS => emit_byte(OpCode::OP_SUBTRACT as u8, parser, compiling_chunk),
        TokenType::TOKEN_STAR => emit_byte(OpCode::OP_MULTIPLY as u8, parser, compiling_chunk),
        TokenType::TOKEN_SLASH => emit_byte(OpCode::OP_DIVIDE as u8, parser, compiling_chunk),
        _ => {}
    }
}

fn get_rule(typo: TokenType) -> ParseRule {
    let rule = RULE[typo as usize].clone();
    ParseRule {
        prefix: rule.0,
        infix: rule.1,
        precedence: rule.2,
    }
}

fn parse_precedence(
    precedence: Precedence,
    parser: &mut Parser,
    scanner: &mut Scanner,
    chunk: &mut Chunk,
) {
    let mut precedence = precedence;
    parser.advance(scanner);
    let prefix_rule = get_rule(parser.previous.as_ref().unwrap().typo).prefix;
    match prefix_rule {
        None => error(
            &parser.previous.as_ref().unwrap().clone(),
            "Expected Expression",
            scanner,
            parser,
        ),
        Some(prefix) => {
            prefix(parser, scanner, chunk);
            while (precedence.clone() as u8)
                < (get_rule(parser.current.as_ref().unwrap().typo).precedence as u8)
            {
                parser.advance(scanner);
                let infix = get_rule(parser.previous.as_ref().unwrap().typo)
                    .infix
                    .unwrap();

                infix(parser, scanner, chunk);
            }
        }
    }
}

fn emit_constant(
    value: Value,
    parser: &mut Parser,
    compiling_chunk: &mut Chunk,
    scanner: &Scanner,
) {
    emit_bytes(
        OpCode::OP_CONSTANT as u8,
        make_constant(value, compiling_chunk, parser, scanner),
        parser,
        compiling_chunk,
    )
}
fn make_constant(
    value: Value,
    current_chunk: &mut Chunk,
    parser: &mut Parser,
    scanner: &Scanner,
) -> u8 {
    let constant = current_chunk.add_constant(value);

    if constant == u8::MAX {
        error(
            &parser.previous.clone().unwrap(),
            "Too Many Constansts",
            scanner,
            parser,
        );
        return 0;
    }
    constant
}
