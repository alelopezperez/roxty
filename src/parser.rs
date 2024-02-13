use std::fmt::Debug;

use crate::{
    ast::{self, Expr},
    token::{Object, Token, TokenType},
};

trait Prod {
    type Input: Debug;
}

pub fn parse_expr(tokens: &Vec<Token>, mut pos: usize) -> Expr {
    expression(tokens, &mut pos)
}

fn expression(tokens: &Vec<Token>, pos: &mut usize) -> Expr {
    equality(tokens, pos)
}
fn equality(tokens: &Vec<Token>, pos: &mut usize) -> Expr {
    let mut expr = comparison(tokens, pos);

    while *pos < tokens.len() {
        match tokens[*pos].token_type {
            TokenType::BANG_EQUAL | TokenType::EQUAL_EQUAL => {
                *pos += 1;
                let operator = tokens[*pos - 1].clone();
                let right = comparison(tokens, pos);
                expr = ast::Expr::Binary(Box::new(expr.clone()), operator, Box::new(right));
            }
            _ => {
                break;
            }
        }
    }
    expr
}

fn comparison(tokens: &Vec<Token>, pos: &mut usize) -> Expr {
    let mut expr = term(tokens, pos);

    while *pos < tokens.len() {
        match tokens[*pos].token_type {
            TokenType::GREATER
            | TokenType::GREATER_EQUAL
            | TokenType::LESS
            | TokenType::LESS_EQUAL => {
                *pos += 1;
                let operator = tokens[*pos - 1].clone();
                let right = term(tokens, pos);
                expr = ast::Expr::Binary(Box::new(expr.clone()), operator, Box::new(right));
            }
            _ => {
                break;
            }
        }
    }
    expr
}

fn term(tokens: &Vec<Token>, pos: &mut usize) -> Expr {
    let mut expr = factor(tokens, pos);

    while *pos < tokens.len() {
        match tokens[*pos].token_type {
            TokenType::MINUS | TokenType::PLUS => {
                *pos += 1;
                let operator = tokens[*pos - 1].clone();
                let right = factor(tokens, pos);
                expr = ast::Expr::Binary(Box::new(expr.clone()), operator, Box::new(right));
            }
            _ => {
                break;
            }
        }
    }
    expr
}
fn factor(tokens: &Vec<Token>, pos: &mut usize) -> Expr {
    let mut expr = unary(tokens, pos);

    while *pos < tokens.len() {
        match tokens[*pos].token_type {
            TokenType::SLASH | TokenType::STAR => {
                *pos += 1;
                let operator = tokens[*pos - 1].clone();
                let right = unary(tokens, pos);
                expr = ast::Expr::Binary(Box::new(expr.clone()), operator, Box::new(right));
            }
            _ => {
                break;
            }
        }
    }
    expr
}

fn unary(tokens: &Vec<Token>, pos: &mut usize) -> Expr {
    match tokens[*pos].token_type {
        TokenType::BANG | TokenType::MINUS => {
            *pos += 1;
            let operator = tokens[*pos - 1].clone();
            let right = unary(tokens, pos);
            ast::Expr::Unary(operator, Box::new(right))
        }
        _ => primary(tokens, pos),
    }
}

fn primary(tokens: &Vec<Token>, pos: &mut usize) -> Expr {
    match tokens[*pos].token_type {
        TokenType::FALSE => {
            *pos += 1;
            ast::Expr::Literal(ast::LoxVal::Boolean(false))
        }
        TokenType::TRUE => {
            *pos += 1;
            ast::Expr::Literal(ast::LoxVal::Boolean(true))
        }
        TokenType::NIL => {
            *pos += 1;
            ast::Expr::Literal(ast::LoxVal::Nil)
        }

        TokenType::NUMBER => {
            *pos += 1;
            if let Object::Number(num) = tokens[*pos - 1].literal {
                ast::Expr::Literal(ast::LoxVal::Number(num))
            } else {
                panic!("what");
            }
        }
        TokenType::STRING => {
            *pos += 1;
            if let Object::String(word) = tokens[*pos - 1].literal.clone() {
                ast::Expr::Literal(ast::LoxVal::String(word))
            } else {
                panic!("what");
            }
        }

        TokenType::LEFT_PAREN => {
            *pos += 1;
            let expr = expression(tokens, pos);
            consume(
                TokenType::RIGHT_PAREN,
                "Expected ')' after expresion".to_string(),
                tokens,
                pos,
            );
            ast::Expr::Grouping(Box::new(expr))
        }

        _ => panic!("why"),
    }
}

fn consume(
    tipo: TokenType,
    message: String,
    tokens: &Vec<Token>,
    pos: &mut usize,
) -> Result<TokenType, String> {
    if tipo == tokens[*pos].token_type {
        *pos += 1;
        Ok(tipo)
    } else {
        error(tokens, pos, message)
    }
}

fn error(tokens: &Vec<Token>, pos: &mut usize, message: String) -> Result<TokenType, String> {
    if tokens[*pos + 1].token_type == TokenType::EOF {
        Err(format!("{} at end {}", tokens[*pos + 1].line, message).to_string())
    } else {
        Err(format!(
            "{} at  {} {}",
            tokens[*pos + 1].line,
            tokens[*pos + 1].lexeme,
            message
        )
        .to_string())
    }
}

fn synchronize(tokens: &Vec<Token>, pos: &mut usize) {
    *pos += 1;

    while *pos != tokens.len() - 1 {
        if tokens[*pos - 1].token_type == TokenType::SEMICOLON {
            return;
        }

        match tokens[*pos].token_type {
            TokenType::CLASS => {}
            TokenType::FUN => {}
            TokenType::VAR => {}
            TokenType::FOR => {}
            TokenType::IF => {}
            TokenType::WHILE => {}
            TokenType::PRINT => {}
            TokenType::RETURN => {
                return;
            }
            _ => {}
        }
        *pos += 1;
    }
}
