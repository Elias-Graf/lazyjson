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

pub fn tokenize(inp: &str) -> Vec<Token> {
    let consumers: &[&dyn Fn(&String, usize) -> Result<ConsumerResponse, TokenizationError>] = &[
        &keyword_literal_consumer,
        &number_literal_consumer,
        &operator_consumer,
        &separator_consumer,
        &string_literal_consumer,
        &whitespace_consumer,
    ];

    let mut toks = Vec::new();
    let mut cons: usize = 0;

    loop {
        for consumer in consumers {
            if cons >= inp.chars().count() {
                return toks;
            }

            let res = consumer(&inp.into(), cons).unwrap();

            if res.cons > 0 {
                cons += res.cons;

                if res.tok != None {
                    toks.push(res.tok.unwrap());
                }

                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn primitive() {
        let r = tokenize("false");
        let e: Vec<Token> = [Token {
            typ: TokenType::KeywordLiteral,
            val: "false".into(),
        }]
        .into();

        assert_eq!(r, e);
    }
    #[test]
    pub fn array_of_primitives() {
        tokenize_and_assert(
            "[false, true, null]",
            &[
                Token {
                    typ: TokenType::Separator,
                    val: "[".into(),
                },
                Token {
                    typ: TokenType::KeywordLiteral,
                    val: "false".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: ",".into(),
                },
                Token {
                    typ: TokenType::KeywordLiteral,
                    val: "true".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: ",".into(),
                },
                Token {
                    typ: TokenType::KeywordLiteral,
                    val: "null".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "]".into(),
                },
            ],
        );
    }
    #[test]
    pub fn array_of_strings() {
        tokenize_and_assert(
            "[\"hello\", \"world\"]",
            &[
                Token {
                    typ: TokenType::Separator,
                    val: "[".into(),
                },
                Token {
                    typ: TokenType::StringLiteral,
                    val: "hello".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: ",".into(),
                },
                Token {
                    typ: TokenType::StringLiteral,
                    val: "world".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "]".into(),
                },
            ],
        );
    }
    #[test]
    pub fn maps() {
        tokenize_and_assert(
            "{\"firstName\": \"Bob\", \"lastName\": \"Miller\"}",
            &[
                Token {
                    typ: TokenType::Separator,
                    val: "{".into(),
                },
                Token {
                    typ: TokenType::StringLiteral,
                    val: "firstName".into(),
                },
                Token {
                    typ: TokenType::Operator,
                    val: ":".into(),
                },
                Token {
                    typ: TokenType::StringLiteral,
                    val: "Bob".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: ",".into(),
                },
                Token {
                    typ: TokenType::StringLiteral,
                    val: "lastName".into(),
                },
                Token {
                    typ: TokenType::Operator,
                    val: ":".into(),
                },
                Token {
                    typ: TokenType::StringLiteral,
                    val: "Miller".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "}".into(),
                },
            ],
        );
    }
    #[test]
    pub fn array_of_arrays() {
        tokenize_and_assert(
            "[[false],[false],[false]]",
            &[
                Token {
                    typ: TokenType::Separator,
                    val: "[".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "[".into(),
                },
                Token {
                    typ: TokenType::KeywordLiteral,
                    val: "false".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "]".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: ",".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "[".into(),
                },
                Token {
                    typ: TokenType::KeywordLiteral,
                    val: "false".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "]".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: ",".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "[".into(),
                },
                Token {
                    typ: TokenType::KeywordLiteral,
                    val: "false".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "]".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "]".into(),
                },
            ],
        );
    }
    #[test]
    pub fn array_of_objects() {
        tokenize_and_assert(
            "[{\"city\": \"New York\"}, {\"city\": \"London\"}]",
            &[
                Token {
                    typ: TokenType::Separator,
                    val: "[".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "{".into(),
                },
                Token {
                    typ: TokenType::StringLiteral,
                    val: "city".into(),
                },
                Token {
                    typ: TokenType::Operator,
                    val: ":".into(),
                },
                Token {
                    typ: TokenType::StringLiteral,
                    val: "New York".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "}".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: ",".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "{".into(),
                },
                Token {
                    typ: TokenType::StringLiteral,
                    val: "city".into(),
                },
                Token {
                    typ: TokenType::Operator,
                    val: ":".into(),
                },
                Token {
                    typ: TokenType::StringLiteral,
                    val: "London".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "}".into(),
                },
                Token {
                    typ: TokenType::Separator,
                    val: "]".into(),
                },
            ],
        );
    }

    fn tokenize_and_assert(inp: &str, exp: &[Token]) {
        let ret = tokenize(inp);
        let exp_v: Vec<Token> = exp.into();

        assert_eq!(ret, exp_v);
    }
}
