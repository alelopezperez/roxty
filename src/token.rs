use enum_stringify::EnumStringify;

#[derive(Debug, EnumStringify, Clone, Copy)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f64),
    Null,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Object,
    line: usize,
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    // pub fn to_string(&self) -> String {
    //     format!("{} {}  ", self.token_type, self.lexeme,)
    // }
}
