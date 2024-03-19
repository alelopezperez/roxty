// use std::{
//     collections::{HashMap, VecDeque},
//     env, fmt,
//     io::Write,
// };
// mod ast;
// mod interpreter;
// mod parser;
// mod scanner;
// mod token;
// use ast::{LoxVal, Stmt};
// use parser::parse;
// use scanner::Scanner;

// #[derive(Debug, Clone)]
// struct ArgsQuantityError;

// #[derive(Clone, Debug)]
// struct Enviroments {
//     enclosing: Option<Box<Enviroments>>,
//     map: HashMap<String, LoxVal>,
// }
// impl Enviroments {
//     fn get(&self, id: &str) -> &LoxVal {
//         if self.map.contains_key(id) {
//             return self.map.get(id).unwrap();
//         }

//         match &self.enclosing {
//             Some(enc) => enc.get(id),
//             None => {
//                 panic!("HEY NO VARI IN ENVIROMENT")
//             }
//         }
//     }
//     fn define(&mut self, key: String, value: LoxVal) {
//         self.map.insert(key, value);
//     }

//     fn assign(&mut self, key: String, value: LoxVal) {
//         if self.map.contains_key(&key) {
//             self.map.insert(key, value);
//             return;
//         }

//         match &mut self.enclosing {
//             Some(enc) => enc.assign(key, value),
//             None => {
//                 panic!("HEY NO VARI IN ENVIROMENT")
//             }
//         }
//     }
// }

// impl fmt::Display for ArgsQuantityError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "invalid first item to double")
//     }
// }

// fn main() -> Result<(), ArgsQuantityError> {
//     let args: Vec<String> = env::args().collect();

//     match args.len() {
//         2 => {
//             println!("run File {:?}", args);
//             let file_path = &args[1];
//             run_file(file_path).unwrap();
//             Ok(())
//         }
//         1 => {
//             run_prompt();
//             Ok(())
//         }
//         _ => {
//             eprintln!("Usage roxty [script]");
//             Err(ArgsQuantityError)
//         }
//     }
// }

// fn run_file(path: &str) -> Result<(), (usize, String)> {
//     let mut enviroments: HashMap<String, LoxVal> = HashMap::new();
//     let mut env = Enviroments {
//         enclosing: None,
//         map: enviroments.clone(),
//     };

//     match std::fs::read_to_string(path) {
//         Ok(file) => match run(&file, &mut env) {
//             Ok(_) => {
//                 println!("dentro de ok");
//                 Ok(())
//             }
//             Err(err) => {
//                 println!("dentro de err");

//                 error(err.0, err.1.clone());
//                 Err(err)
//             }
//         },
//         Err(_error) => panic!("File Does not exists"),
//     }
// }
// fn run_prompt() {
//     let mut enviroments: HashMap<String, LoxVal> = HashMap::new();

//     let mut env = Enviroments {
//         enclosing: None,
//         map: enviroments.clone(),
//     };

//     loop {
//         let mut line = String::new();
//         print!("> ");
//         std::io::stdout().flush().unwrap();

//         let check = std::io::stdin()
//             .read_line(&mut line)
//             .expect("Failed to read line");
//         if check == 0 {
//             println!();
//             break;
//         }

//         match run(&line, &mut env) {
//             Ok(_) => {}
//             Err(err) => error(err.0, err.1),
//         }
//     }
// }

// fn run(source: &str, enviroments: &mut Enviroments) -> Result<(), (usize, String)> {
//     let mut scanner = Scanner::new(source);

//     let tokens = scanner.scan_tokens()?;

//     let all_ast = parse(tokens, 0);

//     // let val = ast.interpret();
//     // println!("{:?}", val);
//     for stmt in all_ast.iter() {
//         stmt.eval(enviroments);
//     }
//     Ok(())
// }

// fn error(line: usize, message: String) {
//     report(line, "".to_string(), message);
// }
// fn report(line: usize, donde: String, message: String) {
//     eprintln!("[line {}] Error {}: {}", line, donde, message);
// }
mod chunk;
mod common;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use std::env;
use std::io::Write;
use std::process::ExitCode;

use chunk::Chunk;
use vm::InterpretResultError;
use vm::VM;

fn main() -> ExitCode {
    let mut vm = VM::new();
    vm.init_vm();
    let mut chunk = Chunk::init_chunk();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl(&mut vm, &mut chunk);
    } else if args.len() == 2 {
        return run_file(&args[1], &mut vm, &mut chunk);
    } else {
        panic!("Usage: roxty [path]\n");
    }

    ExitCode::SUCCESS
}

fn repl<'a>(vm: &mut VM<'a>, chunk: &'a mut Chunk) {
    loop {
        let mut line = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        let check = std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if check == 0 {
            break;
        }

        // vm.interpret(line, chunk);
    }
}

fn run_file<'a>(path: &str, vm: &mut VM<'a>, chunk: &'a mut Chunk) -> ExitCode {
    // let mut chunk = Chunk::init_chunk();

    match std::fs::read_to_string(path) {
        Ok(source) => {
            let result = vm.interpret(source, chunk);

            match result {
                Ok(_) => ExitCode::SUCCESS,
                Err(error) => match error {
                    InterpretResultError::INTERPRET_COMPILE_ERROR => ExitCode::from(65),
                    InterpretResultError::INTERPRET_RUNTIME_ERROR => ExitCode::from(70),
                    InterpretResultError::INTERPRET_OK => ExitCode::SUCCESS,
                },
            }
        }
        Err(_) => ExitCode::from(74),
    }
}
