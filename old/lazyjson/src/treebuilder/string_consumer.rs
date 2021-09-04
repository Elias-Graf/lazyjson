use crate::tokenizer::{Token, TokenType};

use super::{consumer_response::ConsumerResponse, error::TreebuilderError, node::Node};

/// Consumes a string literal token. Other tokens will be ignored.
pub fn string_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, TreebuilderError> {
    let tok = toks.get(offset).unwrap();

    Ok(match tok.typ {
        TokenType::StringLiteral => {
            let n = Node::new_str(tok.val.as_str(), tok.clone());
            ConsumerResponse::new(1, Some(n))
        }
        _ => ConsumerResponse::new(0, None),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn non_string() {
        let r = string_consumer(&[Token::kwd("null", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(0, None);

        assert_eq!(r, e);
    }
    #[test]
    pub fn string() {
        let tok = Token::str("test", 0, 0);
        let r = string_consumer(&[tok.clone()], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_str("test", tok)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn at_offset() {
        let tok = Token::str("test", 0, 0);
        let inp = &[Token::kwd("null", 0, 0), tok.clone()];
        let r = string_consumer(inp, 1).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_str("test", tok)));

        assert_eq!(r, e);
    }
}
