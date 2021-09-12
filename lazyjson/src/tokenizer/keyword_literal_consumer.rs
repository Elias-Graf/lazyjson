use std::{iter::Peekable, str::CharIndices};

use super::{consumer_response::ConsumerResponse, error::TokenizationError, token::*};

static KEYWORDS: &'static [&'static str] = &["false", "null", "true"];

#[deprecated(note = "use `keyword_literal_consumer`")]
pub fn old_keyword_literal_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    for &keyword in KEYWORDS {
        let k_len = keyword.chars().count();
        let end = offset + k_len;

        if is_inp_long_enough(inp, end) {
            let slice = &inp[offset..end];

            if slice == keyword {
                let tok = Some(Token::kwd(&String::from(slice), offset, offset + k_len));

                return Ok(ConsumerResponse { cons: k_len, tok });
            }
        }
    }

    Ok(ConsumerResponse { cons: 0, tok: None })
}

fn is_inp_long_enough(inp: &String, offset: usize) -> bool {
    inp.chars().count() >= offset
}

pub fn keyword_literal_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationError> {
    if inp.peek().is_none() {
        return Err(TokenizationError::new_out_of_bounds());
    }

    let kwd = read_until_non_alphabetical(inp);

    if is_not_keyword(&kwd) {
        return Ok(None);
    }

    let val = convert_to_string(&kwd);
    let from = kwd.first().unwrap().0;
    let to = kwd.last().unwrap().0 + 1;

    Ok(Some(Token::kwd(&val, from, to)))
}

fn read_until_non_alphabetical(inp: &mut Peekable<CharIndices>) -> Vec<(usize, char)> {
    inp.take_while(|(_, c)| c.is_alphabetic()).collect()
}

fn is_not_keyword(kwd: &Vec<(usize, char)>) -> bool {
    kwd.is_empty()
}

fn convert_to_string(kwd: &Vec<(usize, char)>) -> String {
    kwd.iter()
        .fold(String::new(), |s, (_, c)| s + &c.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let inp = &mut "".char_indices().peekable();
        let r = keyword_literal_consumer(inp).unwrap_err();
        let e = TokenizationError::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_keyword() {
        let inp = &mut "1".char_indices().peekable();
        let r = keyword_literal_consumer(inp).unwrap();
        let e = None;

        assert_eq!(r, e);
    }

    #[test]
    fn valid_at_start() {
        consume_valid_at_start("false");
        consume_valid_at_start("true");
    }

    fn consume_valid_at_start(inp: &str) {
        let inp_iter = &mut inp.char_indices().peekable();
        let r = keyword_literal_consumer(inp_iter).unwrap();
        let e = Some(Token::kwd(inp, 0, inp.chars().count()));
        assert_eq!(r, e);
    }
}

#[cfg(test)]
mod old_tests {
    use super::*;

    #[test]
    pub fn consume_false() {
        consume_and_assert_keyword("false");
    }
    #[test]
    pub fn consume_null() {
        consume_and_assert_keyword("null");
    }
    #[test]
    pub fn consume_true() {
        consume_and_assert_keyword("true");
    }
    #[test]
    pub fn consume_at_offset() {
        let val = &String::from("    false");
        let r = old_keyword_literal_consumer(val, 4).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            tok: Some(Token::kwd("false", 4, 9)),
        };

        assert_eq!(r, e)
    }
    #[test]
    pub fn consume_non_keyword() {
        let val = &String::from("0");
        let r = old_keyword_literal_consumer(val, 0).unwrap();
        let e = ConsumerResponse { cons: 0, tok: None };

        assert_eq!(r, e);
    }

    fn consume_and_assert_keyword(val: &str) {
        let val = &String::from(val);
        let r = old_keyword_literal_consumer(val, 0).unwrap();
        let len = val.chars().count();
        let e = ConsumerResponse {
            cons: len,
            tok: Some(Token::kwd(val, 0, len)),
        };

        assert_eq!(r, e);
    }
}
