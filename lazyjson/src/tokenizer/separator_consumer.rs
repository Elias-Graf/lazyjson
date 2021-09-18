use std::{iter::Peekable, str::CharIndices};

use super::{error::TokenizationError, Token};

pub fn separator_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationError> {
    match inp.peek() {
        None => Err(TokenizationError::new_out_of_bounds()),
        Some((_, c)) => match c {
            ',' | '[' | ']' | '{' | '}' => {
                let (i, c) = inp.next().unwrap();

                Ok(Some(Token::sep(c.to_string().as_str(), i, i + 1)))
            }
            _ => Ok(None),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let inp = &mut "".char_indices().peekable();
        let r = separator_consumer(inp).unwrap_err();
        let e = TokenizationError::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_separator() {
        let inp = &mut "1".char_indices().peekable();
        let r = separator_consumer(inp).unwrap();
        let e = None;

        assert_eq!(r, e);
    }

    #[test]
    fn checking_does_not_consume() {
        let inp = &mut "1".char_indices().peekable();

        separator_consumer(inp).unwrap();

        assert_eq!(inp.next().unwrap(), (0, '1'));
    }

    #[test]
    fn valid_at_start() {
        consume_valid_at_start(",");
        consume_valid_at_start("[");
        consume_valid_at_start("]");
        consume_valid_at_start("{");
        consume_valid_at_start("}");
    }

    fn consume_valid_at_start(val: &str) {
        let inp = &mut val.char_indices().peekable();
        let r = separator_consumer(inp).unwrap();
        let e = Some(Token::sep(val, 0, 1));

        assert_eq!(r, e);
    }
}
