use super::{error::TokenizationError, ConsumerResponse, Token};

// I would like to use a `HashSet`, but static `HashSet`s are currently not
// supported.
static SEPARATORS: &'static [char] = &[',', '[', ']', '{', '}'];

pub fn separator_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    let c = &inp.chars().nth(offset).unwrap();

    if SEPARATORS.contains(c) {
        let tok = Some(Token::sep(&c.to_string(), offset, offset + 1));
        return Ok(ConsumerResponse { cons: 1, tok });
    }
    Ok(ConsumerResponse { cons: 0, tok: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn consume_comma() {
        consume_and_assert_separator(",");
    }
    #[test]
    pub fn consume_open_bracket() {
        consume_and_assert_separator("[");
    }
    #[test]
    pub fn consume_close_bracket() {
        consume_and_assert_separator("]");
    }
    #[test]
    pub fn consume_open_brace() {
        consume_and_assert_separator("{");
    }
    #[test]
    pub fn consume_close_brace() {
        consume_and_assert_separator("}");
    }
    #[test]
    pub fn consume_at_offset() {
        let r = separator_consumer(&String::from("    ,"), 4).unwrap();
        let e = ConsumerResponse {
            cons: 1,
            tok: Some(Token::sep(",", 4, 5)),
        };

        assert_eq!(r, e);
    }

    fn consume_and_assert_separator(val: &str) {
        let val = &String::from(val);
        let r = separator_consumer(val, 0).unwrap();
        let len = val.chars().count();
        let e = ConsumerResponse {
            cons: len,
            tok: Some(Token::sep(val, 0, len)),
        };

        assert_eq!(r, e);
    }
}
