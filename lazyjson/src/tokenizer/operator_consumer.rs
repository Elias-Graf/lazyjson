use crate::char_queue::CharQueue;

use super::{error::TokenizationErr, Token};

pub fn operator_consumer(inp: &mut CharQueue) -> Result<Option<Token>, TokenizationErr> {
    let c = inp.peek().ok_or(TokenizationErr::new_out_of_bounds())?;

    if c != ':' {
        return Ok(None);
    }

    let from = inp.idx();
    let to = from + 1;

    inp.advance_by(1);

    Ok(Some(Token::new_op(":", from, to)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_operator() {
        let inp = &mut CharQueue::new("1");
        let t = operator_consumer(inp).unwrap();

        assert_eq!(t, None);
    }

    #[test]
    fn checking_does_not_consume() {
        let inp = &mut CharQueue::new("1");

        operator_consumer(inp).unwrap();

        assert_eq!(inp.next(), Some('1'));
    }

    #[test]
    fn at_start() {
        let inp = &mut CharQueue::new(":");
        let t = operator_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_op(":", 0, 1)));
    }

    #[test]
    fn at_offset() {
        let inp = &mut CharQueue::new(" :");

        inp.advance_by(1);

        let t = operator_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_op(":", 1, 2)));
    }

    #[test]
    fn is_consumed() {
        let inp = &mut CharQueue::new(": ");

        operator_consumer(inp).unwrap();

        assert_eq!(inp.next(), Some(' '));
    }
}
