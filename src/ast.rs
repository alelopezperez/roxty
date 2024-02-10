use std::ops::Neg;

use crate::token::{self, Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(LoxVal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum LoxVal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Expr {
    pub fn interpret(&self) -> LoxVal {
        match self {
            Expr::Literal(val) => {
                return val.clone();
            }
            Expr::Unary(pro, b_expr) => match pro.token_type {
                TokenType::MINUS => match b_expr.as_ref() {
                    Expr::Literal(val) => {
                        if let LoxVal::Boolean(bol) = val {
                            return LoxVal::Boolean(!bol);
                        }
                        panic!("a")
                    }
                    _ => {
                        panic!("as")
                    }
                },
                TokenType::BANG => match b_expr.as_ref() {
                    Expr::Literal(val) => {
                        if let LoxVal::Number(num) = val {
                            return LoxVal::Number(num.neg());
                        }
                        panic!("a")
                    }
                    _ => {
                        panic!("as")
                    }
                },
                _ => {
                    panic!("NOt")
                }
            },

            _ => {
                panic!("NOt")
            }
        }
    }
}
pub enum Operator {}
