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
            Expr::Literal(val) => val.clone(),
            Expr::Unary(pro, b_expr) => match pro.token_type {
                TokenType::BANG => match b_expr.as_ref() {
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
                TokenType::MINUS => match b_expr.as_ref() {
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

            Expr::Binary(exp_left, tok, exp_right) => {
                let left = exp_left.interpret();
                let right = exp_right.interpret();

                match tok.token_type {
                    TokenType::EQUAL_EQUAL => match (left, right) {
                        (LoxVal::Boolean(left_bool), LoxVal::Boolean(right_bool)) => {
                            LoxVal::Boolean(left_bool == right_bool)
                        }
                        (LoxVal::Number(left_f), LoxVal::Number(right_f)) => {
                            LoxVal::Boolean(left_f == right_f)
                        }
                        (LoxVal::String(left_s), LoxVal::String(right_s)) => {
                            LoxVal::Boolean(left_s == right_s)
                        }

                        _ => {
                            panic!("TYPE ERROR");
                        }
                    },
                    TokenType::BANG_EQUAL => match (left, right) {
                        (LoxVal::Boolean(left_bool), LoxVal::Boolean(right_bool)) => {
                            LoxVal::Boolean(left_bool != right_bool)
                        }
                        (LoxVal::Number(left_f), LoxVal::Number(right_f)) => {
                            LoxVal::Boolean(left_f != right_f)
                        }
                        (LoxVal::String(left_s), LoxVal::String(right_s)) => {
                            LoxVal::Boolean(left_s != right_s)
                        }

                        _ => {
                            panic!("TYPE ERROR");
                        }
                    },
                    // TokenType::LESS => {}
                    // TokenType::LESS_EQUAL => {}
                    // TokenType::GREATER => {}
                    // TokenType::GREATER_EQUAL => {}
                    TokenType::PLUS => {
                        if let LoxVal::Number(left_f) = left {
                            if let LoxVal::Number(right_f) = right {
                                return LoxVal::Number(left_f + right_f);
                            }
                        }
                        panic!("ANOTHEr type");
                    }
                    TokenType::MINUS => {
                        if let LoxVal::Number(left_f) = left {
                            if let LoxVal::Number(right_f) = right {
                                return LoxVal::Number(left_f - right_f);
                            }
                        }
                        panic!("ANOTHEr type");
                    }
                    TokenType::STAR => {
                        if let LoxVal::Number(left_f) = left {
                            if let LoxVal::Number(right_f) = right {
                                return LoxVal::Number(left_f * right_f);
                            }
                        }
                        panic!("ANOTHEr type");
                    }
                    TokenType::SLASH => {
                        if let LoxVal::Number(left_f) = left {
                            if let LoxVal::Number(right_f) = right {
                                return LoxVal::Number(left_f / right_f);
                            }
                        }
                        panic!("ANOTHEr type");
                    }
                    _ => {
                        panic!("NO OPERATOR")
                    }
                }
            }

            _ => {
                panic!("NOt")
            }
        }
    }
}
pub enum Operator {}
