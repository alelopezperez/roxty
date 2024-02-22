use core::num;
use std::collections::HashMap;

use crate::{
    token::{self, Token, TokenType},
    Enviroments,
};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(LoxVal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
}

pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    VarDecl(Token, Option<Expr>),
    Block(Vec<Stmt>),
    IfStmt(Expr, Box<Stmt>, Option<Box<Stmt>>),
}

#[derive(Debug, Clone)]
pub enum LoxVal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
impl Stmt {
    pub fn eval(&self, enviroments: &mut Enviroments) -> LoxVal {
        match self {
            Stmt::IfStmt(cond, then, else_stmt) => {
                if is_truthy(cond.interpret(enviroments)) {
                    then.eval(enviroments);
                }
                match else_stmt {
                    Some(else_b) => {
                        else_b.eval(enviroments);
                    }
                    None => {}
                }

                LoxVal::Nil
            }
            Stmt::PrintStmt(expr) => {
                let val = expr.interpret(enviroments);
                match val {
                    LoxVal::Boolean(bol) => println!("{bol}"),
                    LoxVal::Number(num) => println!("{num}"),
                    LoxVal::String(word) => println!("{word}"),
                    LoxVal::Nil => println!("Nil"),
                }
                LoxVal::Nil
            }
            Stmt::ExprStmt(expr) => {
                expr.interpret(enviroments);
                LoxVal::Nil
            }
            Stmt::VarDecl(name, init) => {
                let val = match init {
                    Some(exp) => exp.interpret(enviroments),
                    None => LoxVal::Nil,
                };

                println!("{:?} {:?}", name.lexeme, val);

                enviroments.define(name.lexeme.clone(), val);

                LoxVal::Nil
            }
            Stmt::Block(block) => {
                let mut new_env = Enviroments {
                    enclosing: Some(Box::new(enviroments.clone())),
                    map: HashMap::new(),
                };

                for blk in block {
                    blk.eval(&mut new_env);
                }

                LoxVal::Nil
            }
        }
    }
}

impl Expr {
    pub fn interpret(&self, enviroments: &mut Enviroments) -> LoxVal {
        match self {
            Expr::Literal(val) => val.clone(),
            Expr::Unary(pro, b_expr) => {
                let b_exp = b_expr.interpret(enviroments);

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
            Expr::Logical(left, opr, right) => {
                let left = left.interpret(enviroments);

                if let TokenType::OR = opr.token_type {
                    if is_truthy(left.clone()) {
                        return left.clone();
                    }
                } else if !is_truthy(left.clone()) {
                    return left;
                }

                right.interpret(enviroments)
            }

            Expr::Assign(name, expr) => {
                let value = expr.interpret(enviroments);

                enviroments.assign(name.lexeme.clone(), value.clone());
                value
            }

            Expr::Grouping(group) => group.interpret(enviroments),

            Expr::Binary(exp_left, tok, exp_right) => {
                let left = exp_left.interpret(enviroments);
                let right = exp_right.interpret(enviroments);

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
            Expr::Variable(var) => enviroments.get(&var.lexeme).clone(),
        }
    }
}

fn is_truthy(val: LoxVal) -> bool {
    match val {
        LoxVal::Nil => false,
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
