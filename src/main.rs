use std::{env, io::Error};
use thiserror::Error;


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {println!("run File")Ok(())},
        0 => {println!("REPL");Ok(())},
        _=>Err(anyhow!("asd"))
    }

}
