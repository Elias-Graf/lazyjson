use crate::tokenizer::{Token, TokenType};

use super::{consumer_response::ConsumerResponse, error::TreebuilderError, node::Node};

// Consumes a number literal. Non number literals are ignored.
pub fn number_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, TreebuilderError> {
    let tok = toks.get(offset).unwrap();

    Ok(match tok.typ {
        TokenType::NumberLiteral => {
            let n = Node::new_num(tok.val.as_str());
            ConsumerResponse::new(1, Some(n))
        }
        _ => ConsumerResponse::new(0, None),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn non_number() {
        let r = number_consumer(&[Token::kwd("null", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(0, None);

        assert_eq!(r, e);
    }
    #[test]
    pub fn number() {
        let r = number_consumer(&[Token::num("99.123", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_num("99.123")));

        assert_eq!(r, e);
    }
    #[test]
    pub fn at_offset() {
        let inp = &[Token::kwd("null", 0, 0), Token::num("42", 0, 0)];
        let r = number_consumer(inp, 1).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_num("42")));

        assert_eq!(r, e);
    }
}
