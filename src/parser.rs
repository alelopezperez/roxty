use std::fmt::Debug;

use crate::{
    ast::{self, Expr, Stmt},
    token::{Object, Token, TokenType},
};

trait Prod {
    type Input: Debug;
}

pub fn parse(tokens: Vec<Token>, mut pos: usize) -> Vec<Stmt> {
    let mut stmt = Vec::new();

    while pos < tokens.len() {
        if let Some(stm) = statements(&tokens, &mut pos) {
            stmt.push(stm);
        }
    }

    stmt
}

fn statements(tokens: &Vec<Token>, pos: &mut usize) -> Option<Stmt> {
    if let TokenType::PRINT = tokens[*pos].token_type {
        *pos += 1;
        return Some(print_stmt(tokens, pos));
    }

    expr_stmt(tokens, pos)
}
fn print_stmt(tokens: &Vec<Token>, pos: &mut usize) -> Stmt {
    let val = parse_expr(tokens, pos);

    match consume(
        TokenType::SEMICOLON,
        "Expected ';' after value".to_string(),
        tokens,
        pos,
    ) {
        Ok(_) => {}
        Err(_) => {
            panic!("print stmt hey");
        }
    }
    println!("salimo?");

    if val.is_none() {
        panic!("ERROR");
    }
    Stmt::PrintStmt(val.unwrap())
}

fn expr_stmt(tokens: &Vec<Token>, pos: &mut usize) -> Option<Stmt> {
    let expr = parse_expr(tokens, pos);
    println!("fue aqu?");
    if expr.is_none() {
        println!("oye");
        *pos += 1;
        return None;
    }
    match consume(
        TokenType::SEMICOLON,
        "Expected ';' after value".to_string(),
        tokens,
        pos,
    ) {
        Ok(_) => {}
        Err(_) => {
            panic!("exprr stmt hey");
        }
    }

    Some(Stmt::ExprStmt(expr.unwrap()))
}

fn parse_expr(tokens: &Vec<Token>, pos: &mut usize) -> Option<Expr> {
    expression(tokens, pos)
}

fn expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Expr> {
    equality(tokens, pos)
}
fn equality(tokens: &Vec<Token>, pos: &mut usize) -> Option<Expr> {
    if tokens[*pos].token_type == TokenType::EOF {
        return None;
    }
    let mut expr = comparison(tokens, pos);

    while *pos < tokens.len() && tokens[*pos].token_type != TokenType::EOF {
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
    Some(expr)
}

fn comparison(tokens: &Vec<Token>, pos: &mut usize) -> Expr {
    let mut expr = term(tokens, pos);

    while *pos < tokens.len() && tokens[*pos].token_type != TokenType::EOF {
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

    while *pos < tokens.len() && tokens[*pos].token_type != TokenType::EOF {
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

    while *pos < tokens.len() && tokens[*pos].token_type != TokenType::EOF {
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
            let expr = expression(tokens, pos).unwrap();
            match consume(
                TokenType::RIGHT_PAREN,
                "Expected ')' after expresion".to_string(),
                tokens,
                pos,
            ) {
                Ok(_) => {}
                Err(_) => panic!("print right param"),
            }
            ast::Expr::Grouping(Box::new(expr))
        }

        _ => {
            println!("{:?}", tokens[*pos]);
            panic!("why")
        }
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
        println!("NOES SEMICOLON {:?}", tokens[*pos]);
        error(tokens, pos, message)
    }
}

fn error(tokens: &Vec<Token>, pos: &mut usize, message: String) -> Result<TokenType, String> {
    if tokens[*pos].token_type == TokenType::EOF {
        Err(format!("{} at end {}", tokens[*pos].line, message).to_string())
    } else {
        Err(format!(
            "{} at  {} {}",
            tokens[*pos].line, tokens[*pos].lexeme, message
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
