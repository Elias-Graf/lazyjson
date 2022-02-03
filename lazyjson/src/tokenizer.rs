use std::iter::Enumerate;
use std::slice::Iter;

mod error;

mod delimiter_consumer;
pub use delimiter_consumer::delimiter_consumer;

mod keyword_literal_consumer;
pub use keyword_literal_consumer::keyword_literal_consumer;

mod line_comment_consumer;
pub use line_comment_consumer::line_comment_consumer;

mod number_literal_consumer;
pub use number_literal_consumer::number_literal_consumer;

mod operator_consumer;
pub use operator_consumer::operator_consumer;

mod separator_consumer;
pub use separator_consumer::separator_consumer;

mod string_literal_consumer;
pub use string_literal_consumer::string_literal_consumer;

mod whitespace_consumer;
pub use whitespace_consumer::whitespace_consumer;

mod token;

use error::TokenizationErr;
pub use token::{Token, TokenType};

use crate::{char_queue::CharQueue, treebuilder::Config};

pub type TokenIndices<'a> = Enumerate<Iter<'a, Token>>;

type Consumer = dyn Fn(&mut CharQueue) -> Result<Option<Token>, TokenizationErr>;

/// TODO: The whole system does not (really) work with CRLF (\r\n).
///       An easy and (probably) silver bullet solution would just be to either strip
///       all \r before any processing happens, or modify the `whitespace_consumer`
///       to not include it in the value.
pub fn tokenize(inp: &str, config: &Config) -> Result<Vec<Token>, TokenizationErr> {
    if inp.is_empty() {
        return Err(TokenizationErr::new_no_inp());
    }

    let consumers: &[&Consumer] = &[
        &line_comment_consumer,
        &whitespace_consumer,
        &delimiter_consumer,
        &keyword_literal_consumer,
        &number_literal_consumer,
        &operator_consumer,
        &separator_consumer,
        &string_literal_consumer,
    ];

    let mut queue = CharQueue::new(inp);
    let mut toks = Vec::new();

    'o: while queue.has_remaining() {
        for consumer in consumers {
            let tok = consumer(&mut queue)?;

            if let Some(tok) = tok {
                // Omit unnecessary whitespace tokens
                if tok.typ == TokenType::WhitespaceLiteral {
                    continue 'o;
                }

                // Line comments are currently not supported by the treebuilder.
                // So if they are allowed, we omitted them, and otherwise throw an
                // error.
                if tok.typ == TokenType::LineComment {
                    if config.allow_line_comments {
                        continue 'o;
                    }

                    return Err(TokenizationErr::new_line_comments_not_allowed(
                        tok.from, tok.to,
                    ));
                }

                toks.push(tok);
                continue 'o;
            }
        }

        panic!("{:?} was not consumed", queue.peek());
    }

    Ok(toks)
}

#[cfg(test)]
mod tests {
    use crate::treebuilder::Config;

    use super::*;

    #[test]
    fn empty() {
        let r = tokenize("", &Config::DEFAULT).unwrap_err();
        let e = TokenizationErr::new_no_inp();

        assert_eq!(r, e);
    }

    #[test]
    fn keywords() {
        let r = tokenize("false null true", &Config::DEFAULT).unwrap();
        let e = [
            Token::new_kwd("false", 0, 5),
            Token::new_kwd("null", 6, 10),
            Token::new_kwd("true", 11, 15),
        ];

        assert_eq!(r, e);
    }

    #[test]
    fn numbers() {
        let r = tokenize("123 123.456", &Config::DEFAULT).unwrap();
        let e = [
            Token::new_num("123", 0, 3),
            Token::new_num("123.456", 4, 11),
        ];

        assert_eq!(r, e);
    }

    #[test]
    fn operators() {
        let r = tokenize(":", &Config::DEFAULT).unwrap();
        let e = [Token::new_json_assignment_op(0)];

        assert_eq!(r, e);
    }

    #[test]
    fn separators() {
        let r = tokenize(",", &Config::DEFAULT).unwrap();
        let e = [Token::new_sep(",", 0, 1)];

        assert_eq!(r, e);
    }

    #[test]
    fn delimiters() {
        let r = tokenize("[ ] { }", &Config::DEFAULT).unwrap();
        let e = [
            Token::new_delimiter("[", 0, 1),
            Token::new_delimiter("]", 2, 3),
            Token::new_delimiter("{", 4, 5),
            Token::new_delimiter("}", 6, 7),
        ];

        assert_eq!(r, e);
    }

    #[test]
    fn strings() {
        let r = tokenize("\"\" \"hello, world\" \"\\\"cool\\\"\"", &Config::DEFAULT).unwrap();
        let e = [
            Token::new_str("", 0, 2),
            Token::new_str("hello, world", 3, 17),
            Token::new_str("\"cool\"", 18, 28),
        ];

        assert_eq!(r, e);
    }

    #[test]
    fn trailing_whitespace() {
        let r = tokenize("1   ", &Config::DEFAULT).unwrap();
        let e = [Token::new_num("1", 0, 1)];

        assert_eq!(r, e);
    }

    #[test]
    fn comments_not_allowed() {
        assert_eq!(
            tokenize("// TO-DO: IMPLEMENT: good code", &Config::DEFAULT),
            Err(TokenizationErr::new_line_comments_not_allowed(0, 30)),
        );
    }

    #[test]
    fn variable() {
        assert_eq!(
            tokenize("{let test = 10}", &Config::DEFAULT).unwrap(),
            [
                Token::new_delimiter("{", 0, 1),
                Token::new_kwd("let", 1, 4),
                Token::new_kwd("test", 5, 9),
                Token::new_equal_assignment_op(10),
                Token::new_num("10", 12, 14),
                Token::new_delimiter("}", 14, 15),
            ],
        );
    }

    #[test]
    fn comments_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_line_comments = true;

        assert_eq!(
            tokenize("// TO-DO: IMPLEMENT: good code", &config),
            Ok(Vec::new())
        );
    }
}
