use std::{env, io::Error};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    match args.len() {
        2 => {
            println!("run File");
            Ok(())
        }
        1 => {
            println!("REPL");
            Ok(())
        }
        _ => {
            println!("Usage roxty [script]");
            Ok(())
        }
    }
}
