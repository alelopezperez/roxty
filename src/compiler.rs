use crate::scanner::{Scanner, TokenType};

pub fn compile(source: String) {
    let mut scanner = Scanner::init_scanner(source);
    let mut line: isize = -1;
    loop {
        let token = scanner.scan_token();

        if line < 0 && token.line != line as usize {
            print!("{:<4}", token.line);
            line = token.line as isize;
        } else {
            print!("   | ");
        }
        println!("{:<2} {}", token.typo as u8, {
            match token.error {
                Some(msg) => msg,
                None => &scanner.source[token.start..token.start + token.length],
            }
        });

        if token.typo == TokenType::TOKEN_EOF {
            break;
        }
    }
}
