use std::{iter::Peekable, str::CharIndices};

use super::{error::TokenizationErr, Token};

pub fn delimiter_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationErr> {
    let &(i, c) = inp.peek().ok_or(TokenizationErr::new_out_of_bounds())?;

    Ok(match c {
        '[' | ']' | '{' | '}' => {
            inp.next();

            Some(Token::new_delimiter(&c.to_string(), i, i + 1))
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let inp = &mut "".char_indices().peekable();

        let r = delimiter_consumer(inp).unwrap_err();
        let e = TokenizationErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_delimiter_is_not_consumed() {
        let inp = &mut "1".char_indices().peekable();

        assert_eq!(delimiter_consumer(inp).unwrap(), None);
        assert_eq!(inp.next().unwrap(), (0, '1'));
    }

    #[test]
    fn valid_at_start() {
        let delimiters = ["[", "]", "{", "}"];

        for delimiter in delimiters {
            let inp = &mut delimiter.char_indices().peekable();

            let r = delimiter_consumer(inp).unwrap();
            let e = Some(Token::new_delimiter(delimiter, 0, 1));

            assert_eq!(r, e);
            assert_eq!(inp.next(), None);
        }
    }

    #[test]
    fn valid_at_offset() {
        let inp = &mut "   [".char_indices().peekable();

        inp.nth(2);

        let r = delimiter_consumer(inp).unwrap();
        let e = Some(Token::new_delimiter("[", 3, 4));

        assert_eq!(r, e);
    }
}
