mod TokenType;
mod Token;

use std::{env, io};
use std::cell::RefCell;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind, Read, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if let Some(filename) = args.get(1) {
        run_file(filename).expect("File should exist");
        if has_global_error() { exit(65) }
    } else {
        run_prompt()
    }
}

fn run_file(filename: &String) -> Result<(), Error> {
    let file = read_to_string(filename.clone());

    if file.is_err() {
        return Err(Error::new(ErrorKind::NotFound, format!("file {} does not exist!", filename)));
    } else {
        let file_content = file.unwrap();

        // FIXME: remove
        println!("{}", file_content);

        run(file_content.as_str());
        Ok(())
    }
}

fn run_prompt() {
    println!("Starting prompt");
    let mut input = String::new();
    loop {
        print!("λ ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // CTRL-D
                println!("Stopping prompt...");
                break;
            }
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => println!("error: {error}"),
        }
        run(input.as_str());
        set_global_error(false);
        input.clear();
    }
}

fn run(source: &str) {
    let scanner = LoxScanner { source };
    let tokens = scanner.scan_tokens();
    tokens.iter().for_each(|t| println!("Token:{:?}", t));
}


struct LoxScanner<'a> {
    source: &'a str,
}

trait Scanner {
    fn scan_tokens(self) -> Vec<Token::Token>;
}

impl Scanner for LoxScanner<'_> {
    fn scan_tokens(self) -> Vec<Token::Token> {
        // todo!()
        // vec![Token{}]
        report(81, "scan_tokens", "error");
        vec![
            Token::Token::new(
                TokenType::TokenType::EOF,
                "lexeme".to_string(),
                "literal".to_string(),
                0
            )
        ]
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    println!("[line {line}] Error {location}: {message}");
    set_global_error(true);
}

// Global error flag
// set to true when an error is encountered
thread_local!(static HAS_ERROR: RefCell<bool> = RefCell::new(false));

fn has_global_error() -> bool {
    HAS_ERROR.with(|has_error| {
        return *has_error.borrow();
    })
}

fn set_global_error(new_value: bool) {
    HAS_ERROR.with(|has_error| {
        let mut has_error = has_error.borrow_mut();
        *has_error = new_value;
    })
}