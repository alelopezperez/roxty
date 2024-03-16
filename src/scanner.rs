pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    TOKEN_LEFT_PAREN,
    TOKEN_RIGHT_PAREN,
    TOKEN_LEFT_BRACE,
    TOKEN_RIGHT_BRACE,
    TOKEN_COMMA,
    TOKEN_DOT,
    TOKEN_MINUS,
    TOKEN_PLUS,
    TOKEN_SEMICOLON,
    TOKEN_SLASH,
    TOKEN_STAR,
    // One or two character tokens.
    TOKEN_BANG,
    TOKEN_BANG_EQUAL,
    TOKEN_EQUAL,
    TOKEN_EQUAL_EQUAL,
    TOKEN_GREATER,
    TOKEN_GREATER_EQUAL,
    TOKEN_LESS,
    TOKEN_LESS_EQUAL,
    // Literals.
    TOKEN_IDENTIFIER,
    TOKEN_STRING,
    TOKEN_NUMBER,
    // Keywords.
    TOKEN_AND,
    TOKEN_CLASS,
    TOKEN_ELSE,
    TOKEN_FALSE,
    TOKEN_FOR,
    TOKEN_FUN,
    TOKEN_IF,
    TOKEN_NIL,
    TOKEN_OR,
    TOKEN_PRINT,
    TOKEN_RETURN,
    TOKEN_SUPER,
    TOKEN_THIS,
    TOKEN_TRUE,
    TOKEN_VAR,
    TOKEN_WHILE,

    TOKEN_ERROR,
    TOKEN_EOF,
}
pub struct Token {
    pub typo: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
    pub error: Option<&'static str>,
}
impl Scanner {
    pub fn init_scanner(source: String) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::TOKEN_EOF);
        }

        let c = self.advance();
        match c {
            '(' => self.make_token(TokenType::TOKEN_LEFT_PAREN),
            ')' => self.make_token(TokenType::TOKEN_RIGHT_PAREN),
            '{' => self.make_token(TokenType::TOKEN_LEFT_BRACE),
            '}' => self.make_token(TokenType::TOKEN_RIGHT_BRACE),
            ';' => self.make_token(TokenType::TOKEN_SEMICOLON),
            ',' => self.make_token(TokenType::TOKEN_COMMA),
            '.' => self.make_token(TokenType::TOKEN_DOT),
            '-' => self.make_token(TokenType::TOKEN_MINUS),
            '+' => self.make_token(TokenType::TOKEN_PLUS),
            '/' => self.make_token(TokenType::TOKEN_SLASH),
            '*' => self.make_token(TokenType::TOKEN_STAR),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn make_token(&self, typo: TokenType) -> Token {
        Token {
            typo,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            error: None,
        }
    }

    fn error_token(&self, message: &'static str) -> Token {
        Token {
            typo: TokenType::TOKEN_ERROR,
            start: 0,
            length: message.len(),
            line: self.line,
            error: Some(message),
        }
    }
}
