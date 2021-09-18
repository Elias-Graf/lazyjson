use std::{iter::Peekable, str::CharIndices};

use super::{error::TokenizationError, Token};

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
