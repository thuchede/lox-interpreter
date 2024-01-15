use crate::{token};

pub trait Scanner {
    fn scan_tokens(self) -> Vec<token::Token>;
}
