use std::{collections::HashMap, env, fmt, io::Write};
mod ast;
mod interpreter;
mod parser;
mod scanner;
mod token;
use ast::LoxVal;
use parser::parse;
use scanner::Scanner;

#[derive(Debug, Clone)]
struct ArgsQuantityError;

impl fmt::Display for ArgsQuantityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn main() -> Result<(), ArgsQuantityError> {
    let args: Vec<String> = env::args().collect();

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
    let mut enviroments: HashMap<String, LoxVal> = HashMap::new();

    match std::fs::read_to_string(path) {
        Ok(file) => match run(&file, &mut enviroments) {
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
    let mut enviroments: HashMap<String, LoxVal> = HashMap::new();

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

        match run(&line, &mut enviroments) {
            Ok(_) => {}
            Err(err) => error(err.0, err.1),
        }
    }
}

fn run(source: &str, enviroments: &mut HashMap<String, LoxVal>) -> Result<(), (usize, String)> {
    let mut scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens()?;

    let all_ast = parse(tokens, 0);

    // let val = ast.interpret();
    // println!("{:?}", val);
    for stmt in all_ast.iter() {
        stmt.eval(enviroments);
    }
    Ok(())
}

fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}
fn report(line: usize, donde: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, donde, message);
}
