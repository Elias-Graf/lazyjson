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

use self::error::TokenizationError;
use self::keyword_literal_consumer::keyword_literal_consumer;
use self::number_literal_consumer::number_literal_consumer;
use self::operator_consumer::operator_consumer;
use self::separator_consumer::separator_consumer;
use self::string_literal_consumer::string_literal_consumer;
use self::whitespace_consumer::whitespace_consumer;

pub type TokenIndices<'a> = Enumerate<Iter<'a, Token>>;

type Consumer = dyn Fn(&mut Peekable<CharIndices>) -> Result<Option<Token>, TokenizationError>;

pub fn tokenize(inp: &str) -> Result<Vec<Token>, TokenizationError> {
    if inp.is_empty() {
        return Err(TokenizationError::new_no_input());
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

            if tok.is_some() {
                toks.push(tok.unwrap());

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
        let e = TokenizationError::new_no_input();

        assert_eq!(r, e);
    }

    #[test]
    fn keywords() {
        let r = tokenize("false null true").unwrap();
        let e = [
            Token::kwd("false", 0, 5),
            Token::kwd("null", 6, 10),
            Token::kwd("true", 11, 15),
        ];

        assert_eq!(r, e);
    }

    #[test]
    fn numbers() {
        let r = tokenize("123 123.456").unwrap();
        let e = [Token::num("123", 0, 3), Token::num("123.456", 4, 11)];

        assert_eq!(r, e);
    }

    #[test]
    fn operators() {
        let r = tokenize(":").unwrap();
        let e = [Token::op(":", 0, 1)];

        assert_eq!(r, e);
    }

    #[test]
    fn separators() {
        let r = tokenize(", [ ] { }").unwrap();
        let e = [
            Token::sep(",", 0, 1),
            Token::sep("[", 2, 3),
            Token::sep("]", 4, 5),
            Token::sep("{", 6, 7),
            Token::sep("}", 8, 9),
        ];

        assert_eq!(r, e);
    }

    #[test]
    fn strings() {
        let r = tokenize("\"\" \"hello, world\" \"\\\"cool\\\"\"").unwrap();
        let e = [
            Token::str("", 0, 2),
            Token::str("hello, world", 3, 17),
            Token::str("\"cool\"", 18, 28),
        ];

        assert_eq!(r, e);
    }
}
