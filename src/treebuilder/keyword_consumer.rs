use crate::tokenizer::{Token, TokenType};

use super::{consumer_response::ConsumerResponse, error::TreebuilderError, node::Node};

/// Consumes known keyword tokens. Unknown tokens result in an error. Non
/// keyword tokens are ignored.
pub fn keyword_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, TreebuilderError> {
    let tok = toks.get(offset).unwrap();

    if tok.typ != TokenType::KeywordLiteral {
        return Ok(ConsumerResponse::new(0, None));
    }

    match tok.val.as_str() {
        "false" => Ok(ConsumerResponse::new(1, Some(Node::new_bool(false)))),
        "null" => Ok(ConsumerResponse::new(1, Some(Node::new_null()))),
        "true" => Ok(ConsumerResponse::new(1, Some(Node::new_bool(true)))),
        _ => Err(TreebuilderError::new_unknown_kwd(tok.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn non_keyword() {
        let r = keyword_consumer(&[Token::str("non token", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(0, None);

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_false() {
        let r = keyword_consumer(&[Token::kwd("false", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_bool(false)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_null() {
        let r = keyword_consumer(&[Token::kwd("null", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_null()));

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_true() {
        let r = keyword_consumer(&[Token::kwd("true", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_bool(true)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn at_offset() {
        let inp = &[Token::str("placeholder", 0, 0), Token::kwd("null", 0, 0)];
        let r = keyword_consumer(inp, 1).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_null()));

        assert_eq!(r, e);
    }
    #[test]
    pub fn unknown_keyword() {
        let misspelled_true = Token::kwd("tru", 0, 0);
        let r = keyword_consumer(&[misspelled_true.clone()], 0).unwrap_err();
        let e = TreebuilderError::new_unknown_kwd(misspelled_true);

        assert_eq!(r, e);
    }
}
