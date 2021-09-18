use std::{iter::Peekable, str::CharIndices};

use crate::peak_while::PeekWhileExt;

use super::{error::TokenizationError, Token};

pub fn number_literal_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationError> {
    if inp.peek().is_none() {
        return Err(TokenizationError::new_out_of_bounds());
    }

    let num = read_until_non_numeric(inp);

    if is_not_number(&num) {
        return Ok(None);
    }

    let val = convert_to_string(&num);
    let from = num.first().unwrap().0;
    let to = num.last().unwrap().0 + 1;

    Ok(Some(Token::num(&val, from, to)))
}

fn read_until_non_numeric(inp: &mut Peekable<CharIndices>) -> Vec<(usize, char)> {
    inp.peek_while(|(_, c)| c.is_digit(10) || c == &'.')
        .collect()
}

fn is_not_number(num: &Vec<(usize, char)>) -> bool {
    num.is_empty()
}

fn convert_to_string(num: &Vec<(usize, char)>) -> String {
    num.iter()
        .fold(String::new(), |s, (_, c)| s + &c.to_string())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn empty() {
        let inp = &mut "".char_indices().peekable();
        let r = number_literal_consumer(inp).unwrap_err();
        let e = TokenizationError::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_number() {
        let inp = &mut "a".char_indices().peekable();
        let r = number_literal_consumer(inp).unwrap();
        let e = None;

        assert_eq!(r, e);
    }

    #[test]
    fn checking_does_not_consume() {
        let inp = &mut "a".char_indices().peekable();

        number_literal_consumer(inp).unwrap();

        assert_eq!(inp.next().unwrap(), (0, 'a'));
    }

    #[test]
    fn valid_at_start() {
        let val = "123456789";
        let inp = &mut val.char_indices().peekable();
        let r = number_literal_consumer(inp).unwrap();
        let e = Some(Token::num(val, 0, val.chars().count()));

        assert_eq!(r, e);
    }

    #[test]
    fn valid_at_start_with_decimal() {
        let val = "123.456";
        let inp = &mut val.char_indices().peekable();
        let r = number_literal_consumer(inp).unwrap();
        let e = Some(Token::num(val, 0, val.len()));

        assert_eq!(r, e);
    }
}
