use std::{
    env, fmt,
    io::Write,
    path::{Path, PathBuf},
};

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
            run_file(file_path);
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

fn run_file(path: &str) {
    match std::fs::read_to_string(path) {
        Ok(file) => {
            run(&file);
        }
        Err(_error) => panic!("File Does not exists"),
    };
}
fn run_prompt() {
    let mut line = String::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let check = std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if check == 0 {
            println!();
            break;
        }

        run(&line);
    }
}

struct Scanner {}
impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {}
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        Vec::new()
    }
}

#[derive(Debug)]
struct Token {}
impl Token {}

fn run(source: &str) {
    let scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}
fn report(line: usize, donde: String, message: String) {
    eprintln!("line {} Error {}: {}", line, donde, message);
}
