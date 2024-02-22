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
        if let Some(stm) = declaration(&tokens, &mut pos) {
            stmt.push(stm);
        }
    }

    stmt
}

fn declaration(tokens: &Vec<Token>, pos: &mut usize) -> Option<Stmt> {
    if let TokenType::VAR = tokens[*pos].token_type {
        *pos += 1;
        return var_declaraton(tokens, pos);
    }

    return statements(tokens, pos);
}

fn var_declaraton(tokens: &Vec<Token>, pos: &mut usize) -> Option<Stmt> {
    let mut name = tokens[*pos].clone();
    match consume(
        TokenType::IDENTIFIER,
        "Expect variable name.".to_string(),
        tokens,
        pos,
    ) {
        Ok(n) => name = n,
        Err(_) => synchronize(tokens, pos),
    };

    let mut initializer = None;

    if TokenType::EQUAL == tokens[*pos].token_type {
        *pos += 1;
        initializer = Some(expression(tokens, pos).unwrap());
    }
    consume(
        TokenType::SEMICOLON,
        "Expect ';' after variable declaration.".to_string(),
        tokens,
        pos,
    );

    Some(Stmt::VarDecl(name, initializer))
}

fn statements(tokens: &Vec<Token>, pos: &mut usize) -> Option<Stmt> {
    if let TokenType::PRINT = tokens[*pos].token_type {
        *pos += 1;
        return Some(print_stmt(tokens, pos));
    }

    if let TokenType::LEFT_BRACE = tokens[*pos].token_type {
        *pos += 1;
        return Some(block_stmt(tokens, pos));
    }

    if let TokenType::IF = tokens[*pos].token_type {
        *pos += 1;
        return Some(if_stmt(tokens, pos));
    }

    if let TokenType::WHILE = tokens[*pos].token_type {
        *pos += 1;
        return Some(while_stmt(tokens, pos));
    }

    expr_stmt(tokens, pos)
}

fn while_stmt(tokens: &Vec<Token>, pos: &mut usize) -> Stmt {
    consume(
        TokenType::LEFT_PAREN,
        "Expect '(' after 'while'.".to_string(),
        tokens,
        pos,
    );

    let condition = expression(tokens, pos).unwrap();

    consume(
        TokenType::RIGHT_PAREN,
        "Expect '(' after 'if'.".to_string(),
        tokens,
        pos,
    );

    let body = match statements(tokens, pos) {
        Some(b) => Some(Box::new(b)),
        None => None,
    };

    Stmt::WhileStmt(condition, body)
}

fn if_stmt(tokens: &Vec<Token>, pos: &mut usize) -> Stmt {
    consume(
        TokenType::LEFT_PAREN,
        "Expect '(' after 'if'.".to_string(),
        tokens,
        pos,
    );
    let condition = expression(tokens, pos).unwrap();
    consume(
        TokenType::RIGHT_PAREN,
        "Expect '(' after 'if'.".to_string(),
        tokens,
        pos,
    );

    let then_branch = statements(tokens, pos).unwrap();

    let mut else_branch: Option<Box<Stmt>> = None;

    if let TokenType::ELSE = tokens[*pos].token_type {
        *pos += 1;
        else_branch = Some(Box::new(statements(tokens, pos).unwrap()));
    }

    return Stmt::IfStmt(condition, Box::new(then_branch), else_branch);
}

fn block_stmt(tokens: &Vec<Token>, pos: &mut usize) -> Stmt {
    let mut block = Vec::new();
    while *pos < tokens.len()
        && tokens[*pos].token_type != TokenType::EOF
        && tokens[*pos].token_type != TokenType::RIGHT_BRACE
    {
        match declaration(tokens, pos) {
            Some(decl) => block.push(decl),
            None => {}
        }
    }
    consume(
        TokenType::RIGHT_BRACE,
        "Expect '}' after block.".to_string(),
        tokens,
        pos,
    );
    Stmt::Block(block)
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

    if val.is_none() {
        panic!("ERROR");
    }
    Stmt::PrintStmt(val.unwrap())
}

fn expr_stmt(tokens: &Vec<Token>, pos: &mut usize) -> Option<Stmt> {
    let expr = parse_expr(tokens, pos);
    if expr.is_none() {
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
    assignment(tokens, pos)
}

fn assignment(tokens: &Vec<Token>, pos: &mut usize) -> Option<Expr> {
    let expr = or(tokens, pos);

    if let TokenType::EQUAL = tokens[*pos].token_type {
        *pos += 1;
        let equals = tokens[*pos - 1].clone();
        let value = assignment(tokens, pos);

        if let Expr::Variable(name) = expr.clone().unwrap() {
            return Some(Expr::Assign(name, Box::new(value.unwrap())));
        }
        error(tokens, pos, "Invalid assignment target.".to_string());
    }

    return expr;
}

fn or(tokens: &Vec<Token>, pos: &mut usize) -> Option<Expr> {
    let mut expr: Option<Expr> = and(tokens, pos);

    while tokens[*pos].token_type == TokenType::OR {
        *pos += 1;
        let operator = tokens[*pos - 1].clone();
        let right = and(tokens, pos);
        expr = Some(Expr::Logical(
            Box::new(expr.unwrap()),
            operator,
            Box::new(right.unwrap()),
        ));
    }

    expr
}

fn and(tokens: &Vec<Token>, pos: &mut usize) -> Option<Expr> {
    let mut expr = equality(tokens, pos);

    while tokens[*pos].token_type == TokenType::AND {
        *pos += 1;

        let operator = tokens[*pos - 1].clone();
        let right = equality(tokens, pos);

        expr = Some(Expr::Logical(
            Box::new(expr.unwrap()),
            operator,
            Box::new(right.unwrap()),
        ));
    }

    expr
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

        TokenType::IDENTIFIER => {
            *pos += 1;
            ast::Expr::Variable(tokens[*pos - 1].clone())
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
) -> Result<Token, String> {
    if tipo == tokens[*pos].token_type {
        *pos += 1;
        Ok(tokens[*pos - 1].clone())
    } else {
        println!("NOES SEMICOLON {:?}", tokens[*pos]);
        error(tokens, pos, message)
    }
}

fn error(tokens: &Vec<Token>, pos: &mut usize, message: String) -> Result<Token, String> {
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
