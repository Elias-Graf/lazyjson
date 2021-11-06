use crate::char_queue::CharQueue;

use super::{error::TokenizationErr, Token};

pub fn number_literal_consumer(inp: &mut CharQueue) -> Result<Option<Token>, TokenizationErr> {
    let num = match read_until_non_numeric(inp) {
        Some(num) => num,
        None => return Ok(None),
    };

    let from = inp.idx();
    let to = inp.idx() + num.len();

    inp.advance_by(num.len());

    Ok(Some(Token::new_num(&num, from, to)))
}

fn read_until_non_numeric(inp: &mut CharQueue) -> Option<String> {
    let to = inp
        .find_next_by_closure(|c| !c.is_numeric() && c != '.')
        .unwrap_or(inp.len());
    let val = inp.get(inp.idx()..to)?;

    if val.is_empty() {
        return None;
    }
    Some(val.to_string())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn non_number() {
        let inp = &mut CharQueue::new("a");
        let t = number_literal_consumer(inp).unwrap();

        assert_eq!(t, None);
    }

    #[test]
    fn checking_does_not_consume() {
        let inp = &mut CharQueue::new("a");

        number_literal_consumer(inp).unwrap();

        assert_eq!(inp.next(), Some('a'));
    }

    #[test]
    fn at_start() {
        let inp = &mut CharQueue::new("123456789");
        let t = number_literal_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_num("123456789", 0, 9)));
    }

    #[test]
    fn at_start_with_decimal() {
        let inp = &mut CharQueue::new("123.456");
        let t = number_literal_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_num("123.456", 0, 7)));
    }

    #[test]
    fn is_consumed() {
        let inp = &mut CharQueue::new("123 ");

        number_literal_consumer(inp).unwrap();

        assert_eq!(inp.next(), Some(' '));
    }
}
