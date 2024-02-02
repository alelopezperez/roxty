use crate::token::{Object, TokenType};

use super::token::Token;

static KEYWORDS: [(&str, TokenType); 16] = [
    ("and", TokenType::AND),
    ("class", TokenType::CLASS),
    ("else", TokenType::ELSE),
    ("false", TokenType::FALSE),
    ("for", TokenType::FOR),
    ("fun", TokenType::FUN),
    ("if", TokenType::IF),
    ("nil", TokenType::NIL),
    ("or", TokenType::OR),
    ("print", TokenType::PRINT),
    ("return", TokenType::RETURN),
    ("super", TokenType::SUPER),
    ("this", TokenType::THIS),
    ("true", TokenType::TRUE),
    ("var", TokenType::VAR),
    ("while", TokenType::WHILE),
];
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}
impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, (usize, String)> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), Object::Null, 0));

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), (usize, String)> {
        let c = self.advance();
        println!(" Hello {c}");

        println!("{:?}", self.tokens);

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.matchi('=') {
                    self.add_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.matchi('=') {
                    self.add_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.matchi('=') {
                    self.add_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.matchi('=') {
                    self.add_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }

            '/' => {
                // We have to take all the letter if we see a second slash
                if self.matchi('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    self.add_token(TokenType::SLASH)
                }
            }

            ' ' | '\r' | '\t' => Ok(()),

            '\n' => {
                self.line += 1;
                Ok(())
            }
            '"' => self.string(),

            _ => {
                if c.is_ascii_digit() {
                    self.number()
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier()
                } else {
                    Err((self.line, "Unexpected character.".to_string()))
                }
            }
        }
    }

    fn identifier(&mut self) -> Result<(), (usize, String)> {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token = KEYWORDS.iter().find(|&&x| x.0 == text);
        match token {
            Some((_, token)) => self.add_token(*token),
            None => self.add_token(TokenType::IDENTIFIER),
        }
    }

    fn string(&mut self) -> Result<(), (usize, String)> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.advance();
            }
        }

        if self.is_at_end() {
            return Err((self.line, "Unterminated String".to_string()));
        }

        self.advance();

        let string_literal = (self.source[self.start + 1..self.current - 1]).to_string();
        self.add_token_list(TokenType::STRING, Object::String(string_literal));

        Ok(())
    }

    fn number(&mut self) -> Result<(), (usize, String)> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number_literal = self.source[self.start + 1..self.current - 1]
            .parse::<f64>()
            .unwrap();
        self.add_token_list(TokenType::NUMBER, Object::Number(number_literal));

        Ok(())
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        ch
    }

    fn add_token(&mut self, tipo: TokenType) -> Result<(), (usize, String)> {
        self.add_token_list(tipo, Object::String("null".to_string()));
        Ok(())
    }

    fn add_token_list(&mut self, tipo: TokenType, literal: Object) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(tipo, text.to_string(), literal, self.line))
    }

    fn matchi(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }
}
