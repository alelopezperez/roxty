pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(non_camel_case_types)]
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

    pub fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();

            match c {
                ' ' => {}
                '\r' => {}
                '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    pub fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    pub fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::TOKEN_EOF);
        }

        let c = self.advance();

        if c.is_ascii_alphabetic() {
            return self.identifier();
        }
        if c.is_ascii_digit() {
            return self.number();
        }
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
            '!' => {
                if self.matchi('=') {
                    return self.make_token(TokenType::TOKEN_BANG_EQUAL);
                }
                self.make_token(TokenType::TOKEN_BANG)
            }
            '=' => {
                if self.matchi('=') {
                    return self.make_token(TokenType::TOKEN_EQUAL_EQUAL);
                }
                self.make_token(TokenType::TOKEN_EQUAL)
            }
            '<' => {
                if self.matchi('=') {
                    return self.make_token(TokenType::TOKEN_LESS_EQUAL);
                }
                self.make_token(TokenType::TOKEN_LESS)
            }
            '>' => {
                if self.matchi('>') {
                    return self.make_token(TokenType::TOKEN_GREATER_EQUAL);
                }
                self.make_token(TokenType::TOKEN_GREATER)
            }
            '"' => self.string(),

            _ => self.error_token("Unexpected character."),
        }
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.source.chars().nth(self.start).unwrap() {
            'a' => self.check_keyword(1, 2, "nd", TokenType::TOKEN_AND),
            'c' => self.check_keyword(1, 4, "lass", TokenType::TOKEN_CLASS),
            'e' => self.check_keyword(1, 3, "lse", TokenType::TOKEN_ELSE),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).unwrap() {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::TOKEN_FALSE),
                        'o' => self.check_keyword(2, 1, "r", TokenType::TOKEN_FOR),
                        'u' => self.check_keyword(2, 1, "n", TokenType::TOKEN_FUN),
                        _ => TokenType::TOKEN_IDENTIFIER,
                    }
                } else {
                    TokenType::TOKEN_IDENTIFIER
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::TOKEN_IF),
            'n' => self.check_keyword(1, 2, "il", TokenType::TOKEN_NIL),
            'o' => self.check_keyword(1, 1, "r", TokenType::TOKEN_OR),
            'p' => self.check_keyword(1, 4, "rint", TokenType::TOKEN_PRINT),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::TOKEN_RETURN),
            's' => self.check_keyword(1, 4, "uper", TokenType::TOKEN_SUPER),
            't' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).unwrap() {
                        'h' => self.check_keyword(2, 2, "is", TokenType::TOKEN_THIS),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::TOKEN_TRUE),
                        _ => TokenType::TOKEN_IDENTIFIER,
                    }
                } else {
                    TokenType::TOKEN_IDENTIFIER
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::TOKEN_VAR),
            'w' => self.check_keyword(1, 4, "hile", TokenType::TOKEN_WHILE),
            _ => TokenType::TOKEN_IDENTIFIER,
        }
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, typo: TokenType) -> TokenType {
        if self.current - self.start == start + length
            && rest == &self.source[self.start..self.current + 1]
        {
            return typo;
        }
        TokenType::TOKEN_IDENTIFIER
    }
    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        self.make_token(TokenType::TOKEN_NUMBER)
    }
    fn string(&mut self) -> Token {
        while self.peek() != '"' && self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.advance();
            }
        }
        if self.is_at_end() {
            self.error_token("Unterminated String")
        } else {
            self.advance();
            self.make_token(TokenType::TOKEN_STRING)
        }
    }

    fn matchi(&mut self, letter: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source.chars().nth(self.current).unwrap() != letter {
            return false;
        } else {
            self.current += 1;
            true
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
