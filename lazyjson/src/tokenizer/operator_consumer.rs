use std::{iter::Peekable, str::CharIndices};

use super::{error::TokenizationError, ConsumerResponse, Token};

#[deprecated(note = "use `operator_consumer` instead")]
pub fn old_operator_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    if inp.chars().nth(offset).unwrap() == ':' {
        let tok = Some(Token::op(":", offset, offset + 1));
        return Ok(ConsumerResponse { cons: 1, tok });
    }
    Ok(ConsumerResponse { cons: 0, tok: None })
}

pub fn operator_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationError> {
    match inp.peek() {
        None => Err(TokenizationError::new_out_of_bounds()),
        Some((_, c)) => match c {
            ':' => {
                let (i, _) = inp.next().unwrap();

                Ok(Some(Token::op(":", i, i + 1)))
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
        let r = operator_consumer(inp).unwrap_err();
        let e = TokenizationError::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_operator() {
        let inp = &mut "1".char_indices().peekable();
        let r = operator_consumer(inp).unwrap();
        let e = None;

        assert_eq!(r, e);
    }

    #[test]
    fn checking_does_not_consume() {
        let inp = &mut "1".char_indices().peekable();

        operator_consumer(inp).unwrap();

        assert_eq!(inp.next().unwrap(), (0, '1'));
    }

    #[test]
    fn colon() {
        let inp = &mut ":".char_indices().peekable();
        let r = operator_consumer(inp).unwrap();
        let e = Some(Token::op(":", 0, 1));

        assert_eq!(r, e);
    }
}

mod old_tests {
    use super::*;

    #[test]
    pub fn consume_colon() {
        let r = old_operator_consumer(&String::from(":"), 0).unwrap();
        let e = ConsumerResponse {
            cons: 1,
            tok: Some(Token::op(":", 0, 1)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_non_operator() {
        let r = old_operator_consumer(&String::from("0"), 0).unwrap();
        let e = ConsumerResponse { cons: 0, tok: None };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_at_offset() {
        let r = old_operator_consumer(&String::from("    :"), 4).unwrap();
        let e = ConsumerResponse {
            cons: 1,
            tok: Some(Token::op(":", 4, 5)),
        };

        assert_eq!(r, e)
    }
}
