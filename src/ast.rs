use core::num;
use std::{
    collections::HashMap,
    env::args,
    time::{SystemTime, UNIX_EPOCH},
};

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
    Call(Box<Expr>, Token, Vec<Option<Expr>>),
}
#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    VarDecl(Token, Option<Expr>),
    Block(Vec<Stmt>),
    IfStmt(Expr, Box<Stmt>, Option<Box<Stmt>>),
    WhileStmt(Expr, Option<Box<Stmt>>),
    Functions(Token, Vec<Token>, Box<Stmt>),
    Return(Option<Expr>),
}

#[derive(Debug, Clone)]
pub enum LoxVal {
    String(String),
    Number(f64),
    Boolean(bool),
    Functions(Box<Stmt>, Option<Enviroments>),
    Nil,
}

impl LoxVal {
    fn call(&mut self, enviroments: &mut Enviroments, arguments: Vec<LoxVal>) -> LoxVal {
        if let LoxVal::Functions(fun, fun_env) = self {
            if let Stmt::Functions(name, params, body) = fun.as_ref() {
                let mut new_env = Enviroments {
                    enclosing: Some(Box::new(fun_env.as_ref().unwrap().clone())),
                    map: HashMap::new(),
                };

                for (i, param) in params.iter().enumerate() {
                    new_env.define(param.lexeme.clone(), arguments[i].clone());
                }

                let val = body.eval(&mut new_env);
                if let LoxVal::Nil = val {
                } else {
                    *fun_env = Some(new_env.clone());

                    return val;
                }

                *fun_env = Some(new_env.clone());
            }
        }
        LoxVal::Nil
    }
}
/*
let n_env = Enviroments {
                  enclosing: Some(Box::new(enviroments.clone())),
                  map: HashMap::new(),
              };

              let param_values = params.

              for arg in params {
                  n_env.define(arg.lexeme.clone(), value)
              }

              *enviroments = n_env.enclosing.unwrap().as_ref().clone();

              LoxVal::Nil */
impl Stmt {
    // pub fn call(&self, callee: &Expr, args: &Vec<Expr>, enviroments: &mut Enviroments) -> LoxVal {
    //     todo!()
    // }
    pub fn eval(&self, enviroments: &mut Enviroments) -> LoxVal {
        match self {
            Stmt::Return(exp) => match exp {
                Some(expr) => expr.interpret(enviroments),
                None => LoxVal::Nil,
            },
            Stmt::Functions(name, _params, _body) => {
                let fun = LoxVal::Functions(Box::new(self.clone()), Some(enviroments.clone()));
                enviroments.define(name.lexeme.clone(), fun);
                LoxVal::Nil
            }
            Stmt::WhileStmt(condition, body) => {
                while is_truthy(condition.interpret(enviroments)) {
                    match body {
                        Some(body) => {
                            body.eval(enviroments);
                        }
                        None => {}
                    }
                }

                LoxVal::Nil
            }
            Stmt::IfStmt(cond, then, else_stmt) => {
                if is_truthy(cond.interpret(enviroments)) {
                    let val = then.eval(enviroments);
                    if let LoxVal::Nil = val {
                    } else {
                        return val;
                    }
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
                    LoxVal::Functions(_, _) => println!("fun"),
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
                    let val = blk.eval(&mut new_env);

                    if let LoxVal::Nil = val {
                    } else {
                        *enviroments = new_env.enclosing.unwrap().as_ref().clone();

                        return val;
                    }
                }
                *enviroments = new_env.enclosing.unwrap().as_ref().clone();

                LoxVal::Nil
            }
        }
    }
}

impl Expr {
    pub fn interpret(&self, enviroments: &mut Enviroments) -> LoxVal {
        match self {
            Expr::Call(callee, _, arguments) => {
                if let Expr::Variable(s) = callee.as_ref() {
                    if s.lexeme == "clock" {
                        return LoxVal::Number(
                            SystemTime::elapsed(&UNIX_EPOCH).unwrap().as_millis() as f64 / 1000.0,
                        );
                    }
                }
                let mut callee = callee.interpret(enviroments);

                let args = arguments
                    .iter()
                    .filter(|x| x.is_some())
                    .map(|x| x.clone().unwrap().interpret(enviroments))
                    .collect::<Vec<_>>();

                let val = callee.call(enviroments, args);

                if let LoxVal::Nil = val {
                } else {
                    return val;
                }

                LoxVal::Nil
            }
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
