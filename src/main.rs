mod token_type;
mod token;
mod scanner;
mod lox_scanner;
mod ast_printer;
mod expression;

use std::{env, io};
use std::cell::RefCell;
use std::fs::{File};
use std::io::{Error, ErrorKind, Read, Write};
use std::process::exit;
use crate::ast_printer::AstPrinter;
use crate::expression::{Binary, Expr, Grouping, Literal, Unary, VisitedElement};
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token_type::TokenType;


fn main() {
    let expr = Expr::Binary(Binary::new(
        Box::new(Expr::Unary(Unary::new(
            Token::new(TokenType::Minus, "-".to_string(), "".to_string(), 1),
            Box::new(Expr::Literal(Literal::new(Some("123".to_string())))),
        ))),
        Token::new(TokenType::Star, "*".to_string(), "".to_string(), 1),
        Box::new(Expr::Grouping(Grouping::new(
            Box::new(Expr::Literal(Literal::new(Some("45.67".to_string()))))
        )))
    ));
    let printer: AstPrinter = AstPrinter { };
    let pretty = expr.accept(printer);

    println!("{pretty}");
}

// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     if args.len() > 2 {
//         println!("Usage: rlox [script]");
//     } else if let Some(filename) = args.get(1) {
//         run_file(filename).expect("File should exist");
//         if has_global_error() { exit(65) }
//     } else {
//         run_prompt()
//     }
// }

fn run_file(filename: &String) -> Result<(), Error> {
    // let file = read_to_string(filename.clone());
    let mut file = File::open(filename.clone()).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let res = file.read_to_end(&mut buf);

    if res.is_err() {
        return Err(Error::new(ErrorKind::NotFound, format!("file {} does not exist!", filename)));
    } else {
        run(buf.as_slice());
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
        run(input.as_bytes());
        set_global_error(false);
        input.clear();
    }
}

fn run(source: &[u8]) {
    let mut scanner = lox_scanner::LoxScanner::new(source);
    let tokens = scanner.scan_tokens();
    tokens.iter().for_each(|t| println!("Token:{:?}", t));
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
