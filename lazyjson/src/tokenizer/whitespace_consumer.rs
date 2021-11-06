use crate::char_queue::CharQueue;

use super::{error::TokenizationErr, Token};

pub fn whitespace_consumer(inp: &mut CharQueue) -> Result<Option<Token>, TokenizationErr> {
    let from = inp.idx();
    let val = read_until_non_whitespace(inp);

    Ok(match val.len() {
        0 => None,
        _ => Some(Token::new_whitespace(&val, from, from + val.len())),
    })
}

fn read_until_non_whitespace(inp: &mut CharQueue) -> String {
    let mut val = String::new();

    while let Some(c) = inp.peek() {
        if !c.is_whitespace() {
            break;
        }

        val.push(c);
        inp.advance_by(1);
    }

    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_whitespace() {
        let inp = &mut CharQueue::new("1");
        let r = whitespace_consumer(inp).unwrap();
        let e = None;

        dbg!(&inp);

        assert_eq!(r, e);
        assert_eq!(inp.next().unwrap(), '1');
    }

    #[test]
    fn newlines() {
        let inp = &mut CharQueue::new("\n\n");

        let r = whitespace_consumer(inp).unwrap();
        let e = Some(Token::new_whitespace("\n\n", 0, 2));

        assert_eq!(r, e);
        assert_eq!(inp.next(), None);
    }

    #[test]
    fn spaces() {
        let inp = &mut CharQueue::new("   ");

        let r = whitespace_consumer(inp).unwrap();
        let e = Some(Token::new_whitespace("   ", 0, 3));

        assert_eq!(r, e);
        assert_eq!(inp.next(), None);
    }

    #[test]
    fn tabs() {
        let inp = &mut CharQueue::new("\t\t\t\t");

        let r = whitespace_consumer(inp).unwrap();
        let e = Some(Token::new_whitespace("\t\t\t\t", 0, 4));

        assert_eq!(r, e);
        assert_eq!(inp.next(), None);
    }
}
