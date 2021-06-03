use super::{error::TokenizationError, ConsumerResponse, Token, TokenType};

pub fn string_literal_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, UnterminatedStringError> {
    // TODO: declare chars as variable
    if inp.chars().nth(offset).unwrap() != '"' {
        return Ok(ConsumerResponse { cons: 0, tok: None });
    }

    let mut cons: usize = 1;
    let mut val = String::new();

    loop {
        if offset + cons >= inp.chars().count() {
            return Err(UnterminatedStringError);
        }

        let c = inp.chars().nth(offset + cons).unwrap();

        if c == '\\' {
            let next = inp.chars().nth(offset + cons + 1).unwrap();

            if next == '"' {
                val.push('"');

                cons += 2;
            }
        } else if c == '"' {
            cons += 1;
            break;
        } else {
            val.push(c);
            cons += 1;
        }
    }

    Ok(ConsumerResponse {
        cons,
        tok: Some(Token {
            typ: TokenType::StringLiteral,
            val,
        }),
    })
}

#[derive(Clone, Debug)]
pub struct UnterminatedStringError;

impl TokenizationError for UnterminatedStringError {
    fn kind(&self) -> String {
        "string not terminated".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_non_string() {
        let r = string_literal_consumer(&"0".to_string(), 0).unwrap();
        let e = ConsumerResponse { cons: 0, tok: None };

        assert_eq!(r, e);
    }
    #[test]
    fn consume_empty_string() {
        consume_and_assert_string("\"\"");
    }
    #[test]
    fn consume_string() {
        consume_and_assert_string("\"hello world\"");
    }
    #[test]
    fn consume_string_with_escaped_quote() {
        let r = string_literal_consumer(&"\"hello \\\" world\"".to_string(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 16,
            tok: Some(Token {
                typ: TokenType::StringLiteral,
                val: "hello \" world".to_string(),
            }),
        };

        assert_eq!(r, e);
    }
    #[test]
    fn consume_unterminated_string() {
        let rec = string_literal_consumer(&"\"hello world".to_string(), 0).map_err(|e| e.kind());
        let exp = Err(UnterminatedStringError.kind());

        assert_eq!(rec, exp);
    }

    fn consume_and_assert_string(val: &str) {
        let val = &val.to_string();
        let r = string_literal_consumer(val, 0).unwrap();
        let e = ConsumerResponse {
            cons: val.chars().count(),
            tok: Some(Token {
                typ: TokenType::StringLiteral,
                val: val[1..val.chars().count() - 1].to_string(),
            }),
        };
        assert_eq!(r, e);
    }
}
