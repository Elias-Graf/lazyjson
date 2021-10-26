use std::{iter::Peekable, str::CharIndices};

use super::{error::TokenizationErr, token::*};

use crate::peak_while::PeekWhileExt;

pub fn keyword_literal_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationErr> {
    if inp.peek().is_none() {
        return Err(TokenizationErr::new_out_of_bounds());
    }

    let kwd = read_until_non_alphabetical(inp);

    if is_not_keyword(&kwd) {
        return Ok(None);
    }

    let val = convert_to_string(&kwd);
    let from = kwd.first().unwrap().0;
    let to = kwd.last().unwrap().0 + 1;

    Ok(Some(Token::new_kwd(&val, from, to)))
}

fn read_until_non_alphabetical(inp: &mut Peekable<CharIndices>) -> Vec<(usize, char)> {
    inp.peek_while(|(_, c)| c.is_alphabetic()).collect()
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
        let e = TokenizationErr::new_out_of_bounds();

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
    fn checking_does_not_consume() {
        let inp = &mut "1".char_indices().peekable();

        keyword_literal_consumer(inp).unwrap();

        assert_eq!(inp.next().unwrap(), (0, '1'));
    }

    #[test]
    fn valid_at_start() {
        consume_valid_at_start("false");
        consume_valid_at_start("true");
    }

    fn consume_valid_at_start(inp: &str) {
        let inp_iter = &mut inp.char_indices().peekable();
        let r = keyword_literal_consumer(inp_iter).unwrap();
        let e = Some(Token::new_kwd(inp, 0, inp.chars().count()));
        assert_eq!(r, e);
    }
}
