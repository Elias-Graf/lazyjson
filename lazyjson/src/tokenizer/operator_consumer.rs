use super::{error::TokenizationError, ConsumerResponse, Token};

pub fn operator_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    if inp.chars().nth(offset).unwrap() == ':' {
        let tok = Some(Token::op(":", offset, offset + 1));
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
            tok: Some(Token::op(":", 0, 1)),
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
            tok: Some(Token::op(":", 4, 5)),
        };

        assert_eq!(r, e)
    }
}
