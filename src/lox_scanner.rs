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

pub struct LoxScanner<'a> {
    pub(crate) source: &'a[u8],
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Default for LoxScanner<'_> {
    fn default() -> Self {
        LoxScanner {
            source: &[],
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1
        }
    }
}

impl <'a> LoxScanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
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
                let res = if is_digit(c) {
                    self.number()
                } else if is_alpha(c) {
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
        let lexeme_slice: &[u8] = self.source[self.start..self.current].as_ref();
        let lexeme: String = std::str::from_utf8(lexeme_slice).expect("Invalid utf8 sequence").to_string();
        self.tokens.push(Token::new(token_type, lexeme, literal.unwrap_or("".to_string()), self.line))
    }

    fn advance(&mut self) -> char {
        let res = self.source[self.current];
        self.current += 1;
        res as char
    }

    fn match_next(&mut self, next_expected_char: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source[self.current] as char != next_expected_char {
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
            self.source[self.current] as char
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current+1 >= self.source.len() - 1 {
            return '\0';
        }
        self.source[self.current+1] as char
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
        let value: String = std::str::from_utf8(&self.source[self.start+1..self.current-1]).expect("Invalid utf8 sequence").to_string();
        Some(LoxString(value))
    }

    fn number(&mut self) -> Option<TokenType> {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
        }
        while is_digit(self.peek()) {
            self.advance();
        }
        let value: f32 = std::str::from_utf8(&self.source[self.start..self.current]).expect("Invalid utf8 sequence").to_string().parse::<f32>().unwrap();
        Some(Number(value))
    }

    fn identifier(&mut self) -> Option<TokenType> {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let value: &str = std::str::from_utf8(&self.source[self.start+1..self.current-1]).expect("Invalid utf8 sequence");
        let token_type: TokenType = KEYWORDS.with(|k| match k.get(value) {
            Some(c) => c.clone(),
            None => Identifier
        });
        Some(token_type)
    }
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||  c == '_'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}


impl Scanner for LoxScanner<'_> {
    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {

            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(EOF, "".to_string(), "".to_string(), self.line));

        self.tokens.clone()
    }
}
