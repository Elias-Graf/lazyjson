use super::{consumer_response::ConsumerResponse, error::TokenizationError, token::*};

static KEYWORDS: &'static [&'static str] = &["false", "null", "true"];

pub fn keyword_literal_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    for &keyword in KEYWORDS {
        let k_len = keyword.chars().count();
        let end = offset + k_len;

        if is_inp_long_enough(inp, end) {
            let slice = &inp[offset..end];

            if slice == keyword {
                let tok = Some(Token {
                    typ: TokenType::KeywordLiteral,
                    val: String::from(slice),
                });

                return Ok(ConsumerResponse { cons: k_len, tok });
            }
        }
    }

    Ok(ConsumerResponse { cons: 0, tok: None })
}

fn is_inp_long_enough(inp: &String, offset: usize) -> bool {
    inp.chars().count() >= offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn consume_false() {
        consume_and_assert_keyword("false", 0);
    }
    #[test]
    pub fn consume_null() {
        consume_and_assert_keyword("null", 0);
    }
    #[test]
    pub fn consume_true() {
        consume_and_assert_keyword("true", 0);
    }
    #[test]
    pub fn consume_at_offset() {
        let val = &String::from("    false");
        let r = keyword_literal_consumer(val, 4).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            tok: Some(Token {
                typ: TokenType::KeywordLiteral,
                val: String::from("false"),
            }),
        };

        assert_eq!(r, e)
    }
    #[test]
    pub fn consume_non_keyword() {
        let val = &String::from("0");
        let r = keyword_literal_consumer(val, 0).unwrap();
        let e = ConsumerResponse { cons: 0, tok: None };

        assert_eq!(r, e);
    }

    fn consume_and_assert_keyword(val: &str, offset: usize) {
        let val = &String::from(val);
        let e = ConsumerResponse {
            cons: val.chars().count(),
            tok: Some(Token {
                typ: TokenType::KeywordLiteral,
                val: String::from(val),
            }),
        };
        let r = keyword_literal_consumer(val, offset).unwrap();

        assert_eq!(r, e);
    }
}
