use super::{error::TokenizationErr, token::*};

use crate::char_queue::CharQueue;

// TODO: split up into "keyword literals" (hardcoded and predefined), and name literals
// (dynamic and user supplied).
pub fn keyword_literal_consumer(inp: &mut CharQueue) -> Result<Option<Token>, TokenizationErr> {
    let kwd = match read_until_non_alphabetical(inp) {
        Some(kwd) => kwd,
        None => return Ok(None),
    };

    let from = inp.idx();
    let to = inp.idx() + kwd.len();

    inp.advance_by(kwd.len());

    Ok(Some(Token::new_kwd(&kwd, from, to)))
}

fn read_until_non_alphabetical(inp: &mut CharQueue) -> Option<String> {
    let to = inp.find_next(|c| !c.is_alphabetic()).unwrap_or(inp.len());
    let val = inp.get(inp.idx()..to)?.iter().collect::<String>();

    if val.is_empty() {
        return None;
    }
    return Some(val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_keyword() {
        let inp = &mut CharQueue::new("1");
        let t = keyword_literal_consumer(inp).unwrap();

        assert_eq!(t, None);
    }

    #[test]
    fn checking_does_not_consume() {
        let inp = &mut CharQueue::new("1");

        keyword_literal_consumer(inp).unwrap();

        assert_eq!(inp.next(), Some(&'1'));
    }

    #[test]
    fn valid_at_start() {
        consume_valid_at_start("false");
        consume_valid_at_start("true");
    }

    #[test]
    fn valid_at_offset() {
        let inp = &mut CharQueue::new("   false");
        inp.advance_by(3);

        let t = keyword_literal_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_kwd("false", 3, 8)));
    }

    #[test]
    fn is_consumed() {
        let inp = &mut CharQueue::new("false ");
        keyword_literal_consumer(inp).unwrap();

        assert_eq!(inp.next(), Some(&' '));
    }

    fn consume_valid_at_start(inp: &str) {
        let inp_iter = &mut CharQueue::new(inp);
        let r = keyword_literal_consumer(inp_iter).unwrap();
        let e = Some(Token::new_kwd(inp, 0, inp.chars().count()));
        assert_eq!(r, e);
    }
}
