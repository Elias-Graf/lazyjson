use std::iter::Peekable;

use crate::tokenizer::{Token, TokenIndices};

pub fn inp_from(toks: &[Token]) -> Peekable<TokenIndices> {
    toks.iter().enumerate().peekable()
}
