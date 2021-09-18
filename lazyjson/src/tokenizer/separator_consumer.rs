use std::{iter::Peekable, str::CharIndices};

use super::{error::TokenizationError, ConsumerResponse, Token};

// I would like to use a `HashSet`, but static `HashSet`s are currently not
// supported.
static SEPARATORS: &'static [char] = &[',', '[', ']', '{', '}'];

#[deprecated(note = "use `separator_consumer`")]
pub fn old_separator_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    let c = &inp.chars().nth(offset).unwrap();

    if SEPARATORS.contains(c) {
        let tok = Some(Token::sep(&c.to_string(), offset, offset + 1));
        return Ok(ConsumerResponse { cons: 1, tok });
    }
    Ok(ConsumerResponse { cons: 0, tok: None })
}

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

#[cfg(test)]
mod old_tests {
    use super::*;

    #[test]
    pub fn consume_comma() {
        consume_and_assert_separator(",");
    }
    #[test]
    pub fn consume_open_bracket() {
        consume_and_assert_separator("[");
    }
    #[test]
    pub fn consume_close_bracket() {
        consume_and_assert_separator("]");
    }
    #[test]
    pub fn consume_open_brace() {
        consume_and_assert_separator("{");
    }
    #[test]
    pub fn consume_close_brace() {
        consume_and_assert_separator("}");
    }
    #[test]
    pub fn consume_at_offset() {
        let r = old_separator_consumer(&String::from("    ,"), 4).unwrap();
        let e = ConsumerResponse {
            cons: 1,
            tok: Some(Token::sep(",", 4, 5)),
        };

        assert_eq!(r, e);
    }

    fn consume_and_assert_separator(val: &str) {
        let val = &String::from(val);
        let r = old_separator_consumer(val, 0).unwrap();
        let len = val.chars().count();
        let e = ConsumerResponse {
            cons: len,
            tok: Some(Token::sep(val, 0, len)),
        };

        assert_eq!(r, e);
    }
}
