use super::consumer_response::ConsumerResponse;
use super::error::TokenizationError;
use super::token::*;

static KEYWORDS: &'static [&'static str] = &["false", "null", "true"];

pub fn keyword_literal_consumer(
    inp: &String,
    idx: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    for &keyword in KEYWORDS {
        let k_len = keyword.chars().count();
        let end = idx + k_len;

        if is_inp_long_enough(inp, end) {
            let slice = &inp[idx..end];

            if slice == keyword {
                let tok = Some(Token {
                    typ: TokenType::KeywordLit,
                    val: String::from(slice),
                });

                return Ok(ConsumerResponse { cons: k_len, tok });
            }
        }
    }

    Ok(ConsumerResponse { cons: 0, tok: None })
}

fn is_inp_long_enough(inp: &String, idx: usize) -> bool {
    inp.chars().count() >= idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn consume_false() {
        consume_and_assert_keyword("false");
    }
    #[test]
    pub fn consume_null() {
        consume_and_assert_keyword("null");
    }
    #[test]
    pub fn consume_true() {
        consume_and_assert_keyword("true");
    }
    #[test]
    pub fn consume_non_keyword() {
        let val = &String::from("0");
        let r = keyword_literal_consumer(val, 0).unwrap();
        let e = ConsumerResponse { cons: 0, tok: None };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_at_index() {
        let val = &String::from("    false");
        let r = keyword_literal_consumer(val, 4).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            tok: Some(Token {
                typ: TokenType::KeywordLit,
                val: String::from("false"),
            }),
        };

        assert_eq!(r, e)
    }

    fn consume_and_assert_keyword(val: &str) {
        let val = &String::from(val);
        let e = ConsumerResponse {
            cons: val.chars().count(),
            tok: Some(Token {
                typ: TokenType::KeywordLit,
                val: String::from(val),
            }),
        };
        let r = keyword_literal_consumer(val, 0).unwrap();

        assert_eq!(r, e);
    }
}
