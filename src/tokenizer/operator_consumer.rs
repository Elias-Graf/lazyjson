use super::{ConsumerResponse, Token, TokenType};

pub fn operator_consumer(inp: &String, offset: usize) -> Result<ConsumerResponse, ()> {
    if inp.chars().nth(offset).unwrap() == ':' {
        let tok = Some(Token {
            typ: TokenType::Operator,
            val: String::from(":"),
        });
        return Ok(ConsumerResponse { cons: 1, tok });
    }
    Ok(ConsumerResponse { cons: 0, tok: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn consume_colon() {
        let r = operator_consumer(&String::from(":"), 0).unwrap();
        let e = ConsumerResponse {
            cons: 1,
            tok: Some(Token {
                typ: TokenType::Operator,
                val: String::from(":"),
            }),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_non_operator() {
        let r = operator_consumer(&String::from("0"), 0).unwrap();
        let e = ConsumerResponse { cons: 0, tok: None };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_at_offset() {
        let r = operator_consumer(&String::from("    :"), 4).unwrap();
        let e = ConsumerResponse {
            cons: 1,
            tok: Some(Token {
                typ: TokenType::Operator,
                val: String::from(":"),
            }),
        };

        assert_eq!(r, e)
    }
}
