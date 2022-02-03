use std::iter::Peekable;

use crate::tokenizer::{Token, TokenIndices};

pub fn new_delimiter(val: &str) -> Token {
    Token::new_delimiter(val, 0, 0)
}

pub fn new_kwd(val: &str) -> Token {
    Token::new_kwd(val, 0, 0)
}

pub fn new_equal_assignment_op() -> Token {
    Token::new_equal_assignment_op(0)
}

pub fn new_sep(val: &str) -> Token {
    Token::new_sep(val, 0, 0)
}

pub fn new_str(val: &str) -> Token {
    Token::new_str(val, 0, 0)
}

pub fn inp_from(toks: &[Token]) -> Peekable<TokenIndices> {
    toks.iter().enumerate().peekable()
}
