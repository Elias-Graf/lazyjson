use std::{iter::Peekable, str::CharIndices};

use crate::peak_while::PeekWhileExt;

use super::{error::TokenizationError, ConsumerResponse, Token};

#[deprecated(note = "use `number_literal_consumer`")]
pub fn old_number_literal_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    let mut cons: usize = 0;
    let mut has_decimal = false;
    let mut val = String::new();

    for c in inp[offset..].chars() {
        if !c.is_digit(10) {
            if c != '.' {
                break;
            }

            if has_decimal {
                return Err(TokenizationError::new_multiple_decimal_points(
                    inp,
                    offset + cons,
                ));
            }

            has_decimal = true;
        }

        cons += 1;
        val.push(c);
    }

    let mut tok = None;

    if cons != 0 {
        tok = Some(Token::num(&val, offset, offset + cons));
    }

    return Ok(ConsumerResponse { cons, tok });
}

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

#[cfg(test)]
pub mod old_tests {
    use super::*;

    #[test]
    pub fn consume_non_number() {
        let r = old_number_literal_consumer(&"false".to_string(), 0).unwrap();
        let e = ConsumerResponse { cons: 0, tok: None };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_number_without_decimals() {
        let r = old_number_literal_consumer(&"1234".to_string(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 4,
            tok: Some(Token::num("1234", 0, 4)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_number_with_decimals() {
        let r = old_number_literal_consumer(&"1234.5678".to_string(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 9,
            tok: Some(Token::num("1234.5678", 0, 9)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_number_at_offset() {
        let r = old_number_literal_consumer(&"    1".to_string(), 4).unwrap();
        let e = ConsumerResponse {
            cons: 1,
            tok: Some(Token::num("1", 4, 5)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consumer_number_with_two_decimal_points() {
        let inp = "1.2.3";
        let r = old_number_literal_consumer(&inp.to_string(), 0).unwrap_err();
        let e = TokenizationError::new_multiple_decimal_points(inp, 3);

        assert_eq!(r, e);
    }
}
