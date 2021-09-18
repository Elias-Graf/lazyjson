use std::{iter::Peekable, num::NonZeroI32, str::CharIndices};

use crate::peak_while::PeekWhileExt;

use super::{error::TokenizationError, ConsumerResponse, Token};

#[deprecated(note = "use `whitespace_consumer` instead")]
pub fn old_whitespace_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    let mut cons = 0;

    for c in inp[offset..].chars() {
        match c {
            c if c.is_whitespace() => cons += 1,
            _ => break,
        }
    }

    Ok(ConsumerResponse { cons, tok: None })
}

pub fn whitespace_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationError> {
    if inp.peek().is_none() {
        return Err(TokenizationError::new_out_of_bounds());
    }

    while let Some((_, c)) = inp.peek() {
        if c.is_whitespace() {
            inp.next();
        } else {
            break;
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let inp = &mut "".char_indices().peekable();
        let r = whitespace_consumer(inp).unwrap_err();
        let e = TokenizationError::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_whitespace() {
        let inp = &mut "1".char_indices().peekable();
        let r = whitespace_consumer(inp).unwrap();
        let e = None;

        assert_eq!(r, e);
    }

    #[test]
    fn checking_does_not_consume() {
        let inp = &mut "1".char_indices().peekable();

        whitespace_consumer(inp).unwrap();

        assert_eq!(inp.next().unwrap(), (0, '1'));
    }

    #[test]
    fn newlines() {
        consume_three_and_validate_next(&'\n');
    }

    #[test]
    fn spaces() {
        consume_three_and_validate_next(&' ');
    }

    #[test]
    fn tabs() {
        consume_three_and_validate_next(&'\t');
    }

    fn consume_three_and_validate_next(c: &char) {
        let mut val = c.to_string().repeat(3);

        val += "1";

        let inp = &mut val.char_indices().peekable();

        whitespace_consumer(inp).unwrap();

        assert_eq!(inp.next().unwrap(), (3, '1'));
    }
}

#[cfg(test)]
mod old_tests {
    use super::*;

    #[test]
    pub fn non_whitespace() {
        consume_and_expect_length("false", 0);
    }
    #[test]
    pub fn single_space() {
        consume_and_expect_length(" ", 1);
    }
    #[test]
    pub fn multiple_spaces() {
        consume_and_expect_length("   ", 3);
    }
    #[test]
    pub fn tabs() {
        consume_and_expect_length("\t", 1);
    }
    #[test]
    pub fn newline() {
        consume_and_expect_length("\n", 1);
    }
    #[test]
    pub fn at_offset() {
        let r = old_whitespace_consumer(&String::from("false "), 5).unwrap();
        let e = ConsumerResponse { cons: 1, tok: None };

        assert_eq!(r, e);
    }

    fn consume_and_expect_length(inp: &str, cons: usize) {
        let r = old_whitespace_consumer(&String::from(inp), 0).unwrap();
        let e = ConsumerResponse { cons, tok: None };

        assert_eq!(r, e);
    }
}
