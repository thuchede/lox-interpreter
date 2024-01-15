use crate::{token, token_type};
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token_type::TokenType::EOF;

pub struct LoxScanner<'a> {
    pub(crate) source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl Default for LoxScanner<'_> {
    fn default() -> Self {
        LoxScanner {
            source: "",
            start: 0,
            current: 0,
            line: 1
        }
    }
}

impl <'a> LoxScanner<'a> {
    pub fn new(source: &'a str) -> Self {
        LoxScanner {
            source,
            ..Default::default()
        }
    }
}


impl Scanner for LoxScanner<'_> {
    fn scan_tokens(self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        // while !is_at_end() {
        //     start = current;
        //     scan_token()
        // }

        tokens.push(Token::new(EOF, "".to_string(), "".to_string(), self.line));

        tokens
    }
}