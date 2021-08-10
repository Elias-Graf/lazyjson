use super::{
    error::{ErrorKind, TokenizationError},
    ConsumerResponse, Token,
};

pub fn number_literal_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    let mut cons: usize = 0;
    let mut has_decimal = false;
    let mut val = String::new();

    for c in inp[offset..].chars() {
        if !c.is_digit(10) {
            if c != '.' {
                break;
            }

            if has_decimal {
                return Err(TokenizationError::new(ErrorKind::MultipleDecimalPoints));
            }

            has_decimal = true;
        }

        cons += 1;
        val.push(c);
    }

    let mut tok = None;

    if cons != 0 {
        tok = Some(Token::num(&val, offset, offset + cons));
    }

    return Ok(ConsumerResponse { cons, tok });
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn consume_non_number() {
        let r = number_literal_consumer(&"false".to_string(), 0).unwrap();
        let e = ConsumerResponse { cons: 0, tok: None };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_number_without_decimals() {
        let r = number_literal_consumer(&"1234".to_string(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 4,
            tok: Some(Token::num("1234", 0, 4)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_number_with_decimals() {
        let r = number_literal_consumer(&"1234.5678".to_string(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 9,
            tok: Some(Token::num("1234.5678", 0, 9)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_number_at_offset() {
        let r = number_literal_consumer(&"    1".to_string(), 4).unwrap();
        let e = ConsumerResponse {
            cons: 1,
            tok: Some(Token::num("1", 4, 5)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn consumer_number_with_two_decimal_points() {
        let r = number_literal_consumer(&"1.2.3".to_string(), 0)
            .err()
            .unwrap();

        assert_eq!(r.kind, ErrorKind::MultipleDecimalPoints);
    }
}
