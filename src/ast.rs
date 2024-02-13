use crate::token::{Token, TokenType};

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
            Expr::Unary(pro, b_expr) => {
                let b_exp = b_expr.interpret();

                match pro.token_type {
                    TokenType::MINUS => match b_exp {
                        LoxVal::Number(num) => LoxVal::Number(-num),
                        LoxVal::String(to_num) => {
                            let to_num = to_num.parse::<f64>();
                            match to_num {
                                Ok(val) => LoxVal::Number(-val),
                                Err(_error) => panic!("ParseErrorFloat"),
                            }
                        }
                        _ => {
                            panic!("error");
                        }
                    },
                    TokenType::BANG => LoxVal::Boolean(!(is_truthy(b_exp))),
                    _ => {
                        panic!("MORE")
                    }
                }
            }

            Expr::Grouping(group) => group.interpret(),

            Expr::Binary(exp_left, tok, exp_right) => {
                let left = exp_left.interpret();
                let right = exp_right.interpret();

                match tok.token_type {
                    TokenType::EQUAL_EQUAL => LoxVal::Boolean(is_equally(left, right)),
                    TokenType::BANG_EQUAL => LoxVal::Boolean(!(is_equally(left, right))),
                    TokenType::LESS => match (left, right) {
                        (LoxVal::Number(left_num), LoxVal::Number(right_num)) => {
                            LoxVal::Boolean(left_num < right_num)
                        }
                        _ => panic!("LESS ERROR"),
                    },
                    TokenType::LESS_EQUAL => match (left, right) {
                        (LoxVal::Number(left_num), LoxVal::Number(right_num)) => {
                            LoxVal::Boolean(left_num <= right_num)
                        }
                        _ => panic!("LESS EQUAL ERROR"),
                    },
                    TokenType::GREATER => match (left, right) {
                        (LoxVal::Number(left_num), LoxVal::Number(right_num)) => {
                            LoxVal::Boolean(left_num > right_num)
                        }
                        _ => panic!("Greater ERROR"),
                    },
                    TokenType::GREATER_EQUAL => match (left, right) {
                        (LoxVal::Number(left_num), LoxVal::Number(right_num)) => {
                            LoxVal::Boolean(left_num >= right_num)
                        }
                        _ => panic!("Greater EQUAL ERROR"),
                    },
                    TokenType::PLUS => {
                        if let LoxVal::Number(left_f) = left {
                            if let LoxVal::Number(right_f) = right {
                                return LoxVal::Number(left_f + right_f);
                            }
                        }

                        if let LoxVal::String(left_s) = left {
                            if let LoxVal::String(right_s) = right {
                                return LoxVal::String(left_s + right_s.as_str());
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
        }
    }
}

fn is_truthy(val: LoxVal) -> bool {
    match val {
        LoxVal::Nil => true,
        LoxVal::Boolean(booly) => booly,
        _ => true,
    }
}

fn is_equally(left: LoxVal, right: LoxVal) -> bool {
    match (left, right) {
        (LoxVal::Nil, LoxVal::Nil) => true,
        (LoxVal::Nil, _) => false,
        (LoxVal::Number(left_n), LoxVal::Number(right_n)) => left_n == right_n,
        (LoxVal::String(left_n), LoxVal::String(right_n)) => left_n == right_n,
        (_, _) => false,
    }
}
