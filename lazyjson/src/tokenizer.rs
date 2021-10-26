mod consumer_response;
mod error;
mod keyword_literal_consumer;
mod number_literal_consumer;
mod operator_consumer;
mod separator_consumer;
mod string_literal_consumer;
mod token;
mod whitespace_consumer;

use std::iter::{Enumerate, Peekable};
use std::slice::Iter;
use std::str::CharIndices;

pub use token::{Token, TokenType};

use self::error::TokenizationErr;
use self::keyword_literal_consumer::keyword_literal_consumer;
use self::number_literal_consumer::number_literal_consumer;
use self::operator_consumer::operator_consumer;
use self::separator_consumer::separator_consumer;
use self::string_literal_consumer::string_literal_consumer;
use self::whitespace_consumer::whitespace_consumer;

pub type TokenIndices<'a> = Enumerate<Iter<'a, Token>>;

type Consumer = dyn Fn(&mut Peekable<CharIndices>) -> Result<Option<Token>, TokenizationErr>;

pub fn tokenize(inp: &str) -> Result<Vec<Token>, TokenizationErr> {
    if inp.is_empty() {
        return Err(TokenizationErr::new_no_inp());
    }

    let consumers: &[&Consumer] = &[
        &whitespace_consumer,
        &keyword_literal_consumer,
        &number_literal_consumer,
        &operator_consumer,
        &separator_consumer,
        &string_literal_consumer,
    ];

    let mut inp_char_indices = inp.char_indices().peekable();
    let mut toks = Vec::new();

    'o: while inp_char_indices.peek().is_some() {
        for consumer in consumers {
            let tok = consumer(&mut inp_char_indices)?;

            if let Some(tok) = tok {
                // Whitespace tokens are currently not used anywhere and would
                // require adjustments to the rest of the code.
                // Thus they are simply omitted.
                if tok.typ != TokenType::WhitespaceLiteral {
                    toks.push(tok);
                }

                continue 'o;
            }
        }

        panic!("{:?} was not consumed", inp_char_indices.next());
    }

    Ok(toks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let r = tokenize("").unwrap_err();
        let e = TokenizationErr::new_no_inp();

        assert_eq!(r, e);
    }

    #[test]
    fn keywords() {
        let r = tokenize("false null true").unwrap();
        let e = [
            Token::new_kwd("false", 0, 5),
            Token::new_kwd("null", 6, 10),
            Token::new_kwd("true", 11, 15),
        ];

        assert_eq!(r, e);
    }

    #[test]
    fn numbers() {
        let r = tokenize("123 123.456").unwrap();
        let e = [
            Token::new_num("123", 0, 3),
            Token::new_num("123.456", 4, 11),
        ];

        assert_eq!(r, e);
    }

    #[test]
    fn operators() {
        let r = tokenize(":").unwrap();
        let e = [Token::new_op(":", 0, 1)];

        assert_eq!(r, e);
    }

    #[test]
    fn separators() {
        let r = tokenize(", [ ] { }").unwrap();
        let e = [
            Token::new_sep(",", 0, 1),
            Token::new_sep("[", 2, 3),
            Token::new_sep("]", 4, 5),
            Token::new_sep("{", 6, 7),
            Token::new_sep("}", 8, 9),
        ];

        assert_eq!(r, e);
    }

    #[test]
    fn strings() {
        let r = tokenize("\"\" \"hello, world\" \"\\\"cool\\\"\"").unwrap();
        let e = [
            Token::new_str("", 0, 2),
            Token::new_str("hello, world", 3, 17),
            Token::new_str("\"cool\"", 18, 28),
        ];

        assert_eq!(r, e);
    }

    #[test]
    fn trailing_whitespace() {
        let r = tokenize("1   ").unwrap();
        let e = [Token::new_num("1", 0, 1)];

        assert_eq!(r, e);
    }
}
