use crate::token_type::TokenType;
use std::fmt;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}


impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: String,
        line: usize,) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}

impl fmt::Display for Token {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        todo!()
        // write!(f, "{}", self.0);
    }
}