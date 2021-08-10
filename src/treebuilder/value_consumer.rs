use crate::tokenizer::Token;

use super::{
    array_consumer::array_consumer, consumer_response::ConsumerResponse, error::TreebuilderError,
    keyword_consumer::keyword_consumer, number_consumer::number_consumer,
    object_consumer::object_consumer, string_consumer::string_consumer,
};

/// Consumes tokens that are considered values. For example an array contains
/// values and separators (","). An object has a list of keys, assignment tokens
/// (":") and values. Non value tokens will be ignored.
pub fn value_consumer(toks: &[Token], offset: usize) -> Result<ConsumerResponse, TreebuilderError> {
    let consumers = &[
        keyword_consumer,
        number_consumer,
        string_consumer,
        array_consumer,
        object_consumer,
    ];

    for consumer in consumers {
        let resp = consumer(toks, offset)?;

        if resp.cons > 0 {
            return Ok(resp);
        }
    }

    Ok(ConsumerResponse::new(0, None))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::treebuilder::node::Node;

    use super::*;

    #[test]
    pub fn keywords() {
        let r = value_consumer(&[Token::kwd("false", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_bool(false)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn number() {
        let r = value_consumer(&[Token::num("99.22", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_num("99.22")));

        assert_eq!(r, e);
    }
    #[test]
    pub fn string() {
        let r = value_consumer(&[Token::str("test", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_str("test")));

        assert_eq!(r, e);
    }
    #[test]
    pub fn array() {
        let r = value_consumer(&[Token::sep("[", 0, 0), Token::sep("]", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(2, Some(Node::new_arr(Vec::new())));

        assert_eq!(r, e);
    }
    #[test]
    pub fn object() {
        let r = value_consumer(&[Token::sep("{", 0, 0), Token::sep("}", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(2, Some(Node::new_obj(HashMap::new())));

        assert_eq!(r, e);
    }
    #[test]
    pub fn at_offset() {
        let inp = &[
            Token::kwd("null", 0, 0),
            Token::kwd("null", 0, 0),
            Token::kwd("null", 0, 0),
            Token::str("desired value", 0, 0),
        ];

        let r = value_consumer(inp, 3).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_str("desired value")));

        assert_eq!(r, e);
    }
}
