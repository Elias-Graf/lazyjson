use std::iter::Peekable;

use crate::tokenizer::{Token, TokenIndices};

use super::{
    array_consumer::{array_consumer, old_array_consumer},
    consumer_response::ConsumerResponse,
    error::{OldTreebuilderError, TreebuilderErr},
    keyword_consumer::{keyword_consumer, old_keyword_consumer},
    node::Node,
    number_consumer::{number_consumer, old_number_consumer},
    object_consumer::{object_consumer, old_object_consumer},
    string_consumer::{old_string_consumer, string_consumer},
    Consumer,
};

pub fn value_consumer(toks: &mut Peekable<TokenIndices>) -> Result<Option<Node>, TreebuilderErr> {
    let consumers: &[&Consumer] = &[
        &array_consumer,
        &keyword_consumer,
        &number_consumer,
        &object_consumer,
        &string_consumer,
    ];

    for consumer in consumers {
        let res = consumer(toks)?;

        if res.is_some() {
            return Ok(res);
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn array() {
        let toks = [Token::sep("[", 0, 0), Token::sep("]", 0, 0)];
        let r = value_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_arr(Vec::new(), 0, 2));

        assert_eq!(r, e);
    }

    #[test]
    fn keyword() {
        let toks = [Token::kwd("false", 0, 0)];
        let r = value_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_bool(false, 0, 1));

        assert_eq!(r, e);
    }

    #[test]
    fn number() {
        let toks = [Token::num("123.456", 0, 0)];
        let r = value_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_num("123.456", 0, 1));

        assert_eq!(r, e);
    }

    #[test]
    fn object() {
        let toks = [Token::sep("{", 0, 0), Token::sep("}", 0, 0)];
        let r = value_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_obj(HashMap::new(), 0, 2));

        assert_eq!(r, e);
    }

    #[test]
    fn string() {
        let toks = [Token::str("hello world", 0, 0)];
        let r = value_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_str("hello world", 0, 1));

        assert_eq!(r, e);
    }
}

/// Consumes tokens that are considered values. For example an array contains
/// values and separators (","). An object has a list of keys, assignment tokens
/// (":") and values. Non value tokens will be ignored.
pub fn old_value_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, OldTreebuilderError> {
    let consumers = &[
        old_keyword_consumer,
        old_number_consumer,
        old_string_consumer,
        old_array_consumer,
        old_object_consumer,
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
mod old_tests {
    use std::collections::HashMap;

    use crate::treebuilder::old_node::OldNode;

    use super::*;

    #[test]
    pub fn keywords() {
        let tok = Token::kwd("false", 0, 0);
        let r = old_value_consumer(&[tok.clone()], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_bool(false, tok)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn number() {
        let tok = Token::num("99.22", 0, 0);
        let r = old_value_consumer(&[tok.clone()], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_num("99.22", tok)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn string() {
        let tok = Token::str("test", 0, 0);
        let r = old_value_consumer(&[tok.clone()], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_str("test", tok)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn array() {
        let inp = vec![Token::sep("[", 0, 0), Token::sep("]", 0, 0)];
        let r = old_value_consumer(&inp.clone(), 0).unwrap();
        let e = ConsumerResponse::new(2, Some(OldNode::new_arr(Vec::new(), inp)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn object() {
        let inp = vec![Token::sep("{", 0, 0), Token::sep("}", 0, 0)];
        let r = old_value_consumer(&inp.clone(), 0).unwrap();
        let e = ConsumerResponse::new(2, Some(OldNode::new_obj(HashMap::new(), inp)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn at_offset() {
        let str_tok = Token::str("desired value", 0, 0);
        let inp = &[
            Token::kwd("null", 0, 0),
            Token::kwd("null", 0, 0),
            Token::kwd("null", 0, 0),
            str_tok.clone(),
        ];

        let r = old_value_consumer(inp, 3).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_str("desired value", str_tok)));

        assert_eq!(r, e);
    }
}
