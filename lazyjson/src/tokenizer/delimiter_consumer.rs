use crate::char_queue::CharQueue;

use super::{error::TokenizationErr, Token};

const DELIMITERS: [char; 4] = ['[', ']', '{', '}'];

pub fn delimiter_consumer(inp: &mut CharQueue) -> Result<Option<Token>, TokenizationErr> {
    let c = inp.peek().ok_or(TokenizationErr::new_out_of_bounds())?;

    if !DELIMITERS.contains(&c) {
        return Ok(None);
    }

    let from = inp.idx();
    let to = from + 1;
    let tok = Token::new_delimiter(&c.to_string(), from, to);

    inp.advance_by(1);

    Ok(Some(tok))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_delimiter_is_not_consumed() {
        let inp = &mut CharQueue::new("1");

        assert_eq!(delimiter_consumer(inp).unwrap(), None);
        assert_eq!(inp.next().unwrap(), &'1');
    }

    #[test]
    fn valid_at_start() {
        for delimiter in DELIMITERS {
            let delimiter = delimiter.to_string();

            let inp = &mut CharQueue::new(&delimiter);

            let r = delimiter_consumer(inp).unwrap();
            let e = Some(Token::new_delimiter(&delimiter, 0, 1));

            assert_eq!(r, e);
        }
    }

    #[test]
    fn valid_at_offset() {
        let inp = &mut CharQueue::new("   [");

        inp.advance_by(3);

        let r = delimiter_consumer(inp).unwrap();
        let e = Some(Token::new_delimiter("[", 3, 4));

        assert_eq!(r, e);
    }

    #[test]
    fn is_consumed() {
        let inp = &mut CharQueue::new("[1");
        let t = delimiter_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_delimiter("[", 0, 1)));
        assert_eq!(inp.next(), Some(&'1'));
    }
}
