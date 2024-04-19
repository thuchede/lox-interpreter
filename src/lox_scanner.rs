use std::collections::HashMap;
use std::string::ToString;
use crate::error;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::token_type::TokenType::{And, Bang, BangEqual, Class, Comma, Dot, Else, EOF, Equal, EqualEqual, False, For, Fun, Greater, GreaterEqual, Identifier, If, LeftBrace, LeftParen, Less, LessEqual, LoxString, Minus, Nil, Number, Or, Plus, Print, Return, RightBrace, RightParen, SemiColon, Slash, Star, Super, This, True, Var, While};

thread_local!(static KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
    ("and", And),
    ("class", Class),
    ("else", Else),
    ("false", False),
    ("for", For),
    ("fun", Fun),
    ("if", If),
    ("nil", Nil),
    ("or", Or),
    ("print", Print),
    ("return", Return),
    ("super", Super),
    ("this", This),
    ("true", True),
    ("var", Var),
    ("while", While),
]));

pub struct LoxScanner {
    pub(crate) source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Default for LoxScanner {
    fn default() -> Self {
        LoxScanner {
            source: "".to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1
        }
    }
}

impl LoxScanner {
    pub fn new(source: String) -> Self {
        LoxScanner {
            source,
            ..Default::default()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() - 1
    }

    fn scan_token(&mut self) {
        let char: char = self.advance();
        let token = match char {
            '(' => Some(LeftParen),
            ')'=> Some(RightParen),
            '{' => Some(LeftBrace),
            '}' => Some(RightBrace),
            ',' => Some(Comma),
            '.' => Some(Dot),
            '-' => Some(Minus),
            '+' => Some(Plus),
            ';' => Some(SemiColon),
            '*' => Some(Star
            ),
            // operators
            '!' => if self.match_next('=') { Some(BangEqual) } else { Some(Bang) },
            '=' => if self.match_next('=') { Some(EqualEqual) } else { Some(Equal) },
            '<' => if self.match_next('=') { Some(LessEqual) } else { Some(Less) },
            '>' => if self.match_next('=') { Some(GreaterEqual) } else { Some(Greater) },
            '/' => if self.match_next('/') {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                None
            } else { Some(Slash) },
            ' ' => None,
            '\r' => None,
            '\t' => None,
            '\n' => {
                self.line+=1;
                None
            },
            '"' => self.string(),
            c => {
                let res = if isDigit(c) {
                    self.number()
                } else if isAlpha(c) {
                    self.identifier()
                } else {
                    error(self.line, format!("Unexpected character {}", c).as_str());
                    None
                };
                res
            }
        };

        match token {
            Some(t) => self.add_token(t),
            None => {}
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_full_token(token_type, None)
    }

    fn add_full_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme: String = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, lexeme, literal.unwrap_or("".to_string()), self.line))
    }

    fn advance(&mut self) -> char {
        let res = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        res
    }

    fn match_next(&mut self, next_expected_char: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source.chars().nth(self.current).unwrap() != next_expected_char {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current+1 >= self.source.len() - 1 {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) -> Option<TokenType> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string");
            return None
        }

        self.advance();
        let value: String = self.source.clone()[self.start+1..self.current-1].to_string();
        Some(LoxString(value))
    }

    fn number(&mut self) -> Option<TokenType> {
        while isDigit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && isDigit(self.peek_next()) {
            self.advance();
        }
        while isDigit(self.peek()) {
            self.advance();
        }
        let value: f32 = self.source.clone()[self.start..self.current].parse::<f32>().unwrap();
        Some(Number(value))
    }

    fn identifier(&mut self) -> Option<TokenType> {
        while isAlphaNumeric(self.peek()) {
            self.advance();
        }
        let value = &self.source.clone()[self.start..self.current];
        let token_type: TokenType = KEYWORDS.with(|k| match k.get(&value) {
            Some(c) => c.clone(),
            None => Identifier
        });
        Some(token_type)
    }
}

fn isAlpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||  c == '_'
}

fn isDigit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn isAlphaNumeric(c: char) -> bool {
    isAlpha(c) || isDigit(c)
}


impl Scanner for LoxScanner {
    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {

            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(EOF, "".to_string(), "".to_string(), self.line));

        self.tokens.clone()
    }
}
