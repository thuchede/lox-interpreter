use crate::{token};

pub trait Scanner {
    fn scan_tokens(&mut self) -> Vec<token::Token>;
}
