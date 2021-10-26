use std::{iter::Peekable, str::CharIndices};

use super::{error::TokenizationErr, Token};

pub fn whitespace_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationErr> {
    let &(from, _) = inp.peek().ok_or(TokenizationErr::new_out_of_bounds())?;
    let val = read_until_non_whitespace(inp);

    Ok(match val.len() {
        0 => None,
        _ => Some(Token::new_whitespace(&val, from, from + val.len())),
    })
}

fn read_until_non_whitespace(inp: &mut Peekable<CharIndices>) -> String {
    let mut val = String::new();

    while let Some((_, c)) = inp.peek() {
        if !c.is_whitespace() {
            break;
        }

        val.push(*c);
        inp.next();
    }

    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let inp = &mut "".char_indices().peekable();
        let r = whitespace_consumer(inp).unwrap_err();
        let e = TokenizationErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_whitespace() {
        let inp = &mut "1".char_indices().peekable();
        let r = whitespace_consumer(inp).unwrap();
        let e = None;

        assert_eq!(r, e);
        assert_eq!(inp.next().unwrap(), (0, '1'));
    }

    #[test]
    fn newlines() {
        let mut inp = "\n\n".char_indices().peekable();

        let r = whitespace_consumer(&mut inp).unwrap();
        let e = Some(Token::new_whitespace("\n\n", 0, 2));

        assert_eq!(r, e);
        assert_eq!(inp.next(), None);
    }

    #[test]
    fn spaces() {
        let mut inp = "   ".char_indices().peekable();

        let r = whitespace_consumer(&mut inp).unwrap();
        let e = Some(Token::new_whitespace("   ", 0, 3));

        assert_eq!(r, e);
        assert_eq!(inp.next(), None);
    }

    #[test]
    fn tabs() {
        let mut inp = "\t\t\t\t".char_indices().peekable();

        let r = whitespace_consumer(&mut inp).unwrap();
        let e = Some(Token::new_whitespace("\t\t\t\t", 0, 4));

        assert_eq!(r, e);
        assert_eq!(inp.next(), None);
    }
}
