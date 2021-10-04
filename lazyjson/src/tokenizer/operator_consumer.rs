use std::{iter::Peekable, str::CharIndices};

use super::{error::TokenizationErr, Token};

pub fn operator_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationErr> {
    match inp.peek() {
        None => Err(TokenizationErr::new_out_of_bounds()),
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
        let e = TokenizationErr::new_out_of_bounds();

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
