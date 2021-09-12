pub mod error;
use self::error::TokenizationError;

mod token;
pub use self::token::*;

mod consumer_response;
pub use self::consumer_response::*;

mod keyword_literal_consumer;
pub use self::keyword_literal_consumer::*;

mod number_literal_consumer;
pub use self::number_literal_consumer::*;

mod operator_consumer;
pub use self::operator_consumer::*;

mod separator_consumer;
pub use self::separator_consumer::*;

mod string_literal_consumer;
pub use self::string_literal_consumer::*;

mod whitespace_consumer;
pub use self::whitespace_consumer::*;

pub fn tokenize(inp: &str) -> Result<Vec<Token>, TokenizationError> {
    if inp.chars().count() == 0 {
        return Err(TokenizationError::new_no_input());
    }

    let consumers: &[&dyn Fn(&String, usize) -> Result<ConsumerResponse, TokenizationError>] = &[
        &old_keyword_literal_consumer,
        &old_number_literal_consumer,
        &old_operator_consumer,
        &old_separator_consumer,
        &old_string_literal_consumer,
        &whitespace_consumer,
    ];

    let mut toks = Vec::new();
    let mut cons: usize = 0;

    'outer: loop {
        for consumer in consumers {
            if cons >= inp.chars().count() {
                return Ok(toks);
            }

            let res = consumer(&inp.into(), cons)?;

            if res.cons > 0 {
                cons += res.cons;

                if res.tok != None {
                    toks.push(res.tok.unwrap());
                }

                continue 'outer;
            }
        }

        return Err(TokenizationError::new_unhandled_character(inp, cons));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn no_input() {
        let r = tokenize("").unwrap_err();
        let e = TokenizationError::new_no_input();

        assert_eq!(r, e);
    }
    #[test]
    pub fn primitive() {
        let r = tokenize("false").unwrap();
        let e = [Token::kwd("false", 0, 5)];

        assert_eq!(r, e);
    }
    #[test]
    pub fn array_of_primitives() {
        tokenize_and_assert(
            "[false, true, null]",
            &[
                Token::sep("[", 0, 1),
                Token::kwd("false", 1, 6),
                Token::sep(",", 6, 7),
                Token::kwd("true", 8, 12),
                Token::sep(",", 12, 13),
                Token::kwd("null", 14, 18),
                Token::sep("]", 18, 19),
            ],
        );
    }
    #[test]
    pub fn array_of_strings() {
        tokenize_and_assert(
            "[\"hello\", \"world\"]",
            &[
                Token::sep("[", 0, 1),
                Token::str("hello", 1, 8),
                Token::sep(",", 8, 9),
                Token::str("world", 10, 17),
                Token::sep("]", 17, 18),
            ],
        );
    }
    #[test]
    pub fn objects() {
        tokenize_and_assert(
            "{\"firstName\": \"Bob\", \"lastName\": \"Miller\"}",
            &[
                Token::sep("{", 0, 1),
                Token::str("firstName", 1, 12),
                Token::op(":", 12, 13),
                Token::str("Bob", 14, 19),
                Token::sep(",", 19, 20),
                Token::str("lastName", 21, 31),
                Token::op(":", 31, 32),
                Token::str("Miller", 33, 41),
                Token::sep("}", 41, 42),
            ],
        );
    }
    #[test]
    pub fn array_of_arrays() {
        tokenize_and_assert(
            "[[false],[false],[false]]",
            &[
                Token::sep("[", 0, 1),
                Token::sep("[", 1, 2),
                Token::kwd("false", 2, 7),
                Token::sep("]", 7, 8),
                Token::sep(",", 8, 9),
                Token::sep("[", 9, 10),
                Token::kwd("false", 10, 15),
                Token::sep("]", 15, 16),
                Token::sep(",", 16, 17),
                Token::sep("[", 17, 18),
                Token::kwd("false", 18, 23),
                Token::sep("]", 23, 24),
                Token::sep("]", 24, 25),
            ],
        );
    }
    #[test]
    pub fn array_of_objects() {
        tokenize_and_assert(
            "[{\"city\": \"New York\"}, {\"city\": \"London\"}]",
            &[
                Token::sep("[", 0, 1),
                Token::sep("{", 1, 2),
                Token::str("city", 2, 8),
                Token::op(":", 8, 9),
                Token::str("New York", 10, 20),
                Token::sep("}", 20, 21),
                Token::sep(",", 21, 22),
                Token::sep("{", 23, 24),
                Token::str("city", 24, 30),
                Token::op(":", 30, 31),
                Token::str("London", 32, 40),
                Token::sep("}", 40, 41),
                Token::sep("]", 41, 42),
            ],
        );
    }

    fn tokenize_and_assert(inp: &str, exp: &[Token]) {
        let r = tokenize(inp).unwrap();

        assert_eq!(r, exp);
    }
}
