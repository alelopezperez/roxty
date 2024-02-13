use std::{env, fmt, io::Write};
mod ast;
mod interpreter;
mod parser;
mod scanner;
mod token;
use ast::{Expr, LoxVal};
use scanner::Scanner;
use token::Token;

#[derive(Debug, Clone)]
struct ArgsQuantityError;

impl fmt::Display for ArgsQuantityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn main() -> Result<(), ArgsQuantityError> {
    let args: Vec<String> = env::args().collect();

    let to_print = ast::Expr::Binary(
        Box::new(ast::Expr::Unary(
            Token::new(
                token::TokenType::MINUS,
                "-".to_string(),
                token::Object::Null,
                1,
            ),
            Box::new(Expr::Literal(LoxVal::Number(20.0))),
        )),
        Token::new(
            token::TokenType::STAR,
            "*".to_string(),
            token::Object::Null,
            1,
        ),
        Box::new(ast::Expr::Grouping(Box::new(Expr::Literal(
            ast::LoxVal::Number(123.2),
        )))),
    );

    println!("{:?}", to_print);

    match args.len() {
        2 => {
            println!("run File {:?}", args);
            let file_path = &args[1];
            run_file(file_path).unwrap();
            Ok(())
        }
        1 => {
            run_prompt();
            Ok(())
        }
        _ => {
            eprintln!("Usage roxty [script]");
            Err(ArgsQuantityError)
        }
    }
}

fn run_file(path: &str) -> Result<(), (usize, String)> {
    match std::fs::read_to_string(path) {
        Ok(file) => match run(&file) {
            Ok(_) => {
                println!("dentro de ok");
                Ok(())
            }
            Err(err) => {
                println!("dentro de err");

                error(err.0, err.1.clone());
                Err(err)
            }
        },
        Err(_error) => panic!("File Does not exists"),
    }
}
fn run_prompt() {
    loop {
        let mut line = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();

        let check = std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if check == 0 {
            println!();
            break;
        }

        match run(&line) {
            Ok(_) => {}
            Err(err) => error(err.0, err.1),
        }
    }
}

fn run(source: &str) -> Result<(), (usize, String)> {
    let mut scanner = Scanner::new(source);
    println!("PRINT SOURCE {}", source);

    println!("\n\n\n aqui empiez");

    let tokens = scanner.scan_tokens()?;

    println!("\n\n\n aqui termino de escanear");
    for token in tokens.iter() {
        println!("TOKTOK::{:?}", token);
    }
    let ast = parser::parse_expr(&tokens, 0);
    println!("{:#?}", ast);

    // Let's interpret

    let val = ast.interpret();
    println!("{:?}", val);
    Ok(())
}

fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}
fn report(line: usize, donde: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, donde, message);
}
