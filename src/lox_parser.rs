use std::cmp::PartialEq;
use crate::expression::{Expr, Literal};
use crate::token::Token;
use crate::token_type::TokenType;
use crate::token_type::TokenType::EOF;

pub struct LoxParser {
    tokens: Vec<Token>,
    current: usize,
}

impl Default for LoxParser {
    fn default() -> Self {
        LoxParser {
            tokens: vec![],
            current: 0,
        }
    }
}

impl LoxParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        LoxParser {
            tokens,
            ..Default::default()
        }
    }

    fn expression() -> Expr {
        LoxParser::equality()
    }

    fn equality() -> Expr {
        let expr: Expr = LoxParser::comparison();
        //

        expr
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for token in types.iter() {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == EOF
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn comparison() -> Expr {
        Expr::Literal(Literal::new(Some("123".to_string()))) // FIXME:
    }
}