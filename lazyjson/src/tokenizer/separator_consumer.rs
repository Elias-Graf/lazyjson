use crate::char_queue::CharQueue;

use super::{error::TokenizationErr, Token};

pub fn separator_consumer(inp: &mut CharQueue) -> Result<Option<Token>, TokenizationErr> {
    let c = inp.peek().ok_or(TokenizationErr::new_out_of_bounds())?;

    if c != ',' {
        return Ok(None);
    }

    let from = inp.idx();
    let to = from + 1;

    inp.advance_by(1);

    Ok(Some(Token::new_sep(",", from, to)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_separator_is_not_consumed() {
        let inp = &mut CharQueue::new("1");

        separator_consumer(inp).unwrap();

        assert_eq!(inp.next(), Some('1'));
    }

    #[test]
    fn valid_at_start() {
        let inp = &mut CharQueue::new(",");
        let t = separator_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_sep(",", 0, 1)));
    }

    #[test]
    fn valid_at_offset() {
        let inp = &mut CharQueue::new("   ,");

        inp.advance_by(3);

        let t = separator_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_sep(",", 3, 4)));
    }

    #[test]
    fn is_consumed() {
        let inp = &mut CharQueue::new(", ");

        separator_consumer(inp).unwrap();

        assert_eq!(inp.next(), Some(' '));
    }
}
