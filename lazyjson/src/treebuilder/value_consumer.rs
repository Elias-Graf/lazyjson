use std::iter::Peekable;

use crate::tokenizer::TokenIndices;

use super::{
    array_consumer, error::TreebuilderErr, keyword_consumer, node::Node, number_consumer,
    object_consumer, string_consumer, Config,
};

type Consumer =
    dyn Fn(&mut Peekable<TokenIndices>, &Config) -> Result<Option<Node>, TreebuilderErr>;

/// Consumes all possible forms of "value constellations". For example simple
/// numbers (`1`), or arrays (`[1, 2]`), and so on. This consumer combines other
/// "sub-consumers" to achieve this behavior.
pub fn value_consumer(
    toks: &mut Peekable<TokenIndices>,
    config: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let consumers: &[&Consumer] = &[
        &array_consumer,
        &keyword_consumer,
        &number_consumer,
        &object_consumer,
        &string_consumer,
    ];

    for consumer in consumers {
        let res = consumer(toks, config)?;

        if res.is_some() {
            return Ok(res);
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{tokenizer::Token, treebuilder::value_consumer};

    use super::*;

    #[test]
    fn array() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];

        let r = value_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
        let e = Some(Node::new_arr(Vec::new(), 0, 2));

        assert_eq!(r, e);
    }

    #[test]
    fn keyword() {
        let toks = [Token::new_kwd("false", 0, 0)];

        let r = value_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
        let e = Some(Node::new_bool(false, 0, 1));

        assert_eq!(r, e);
    }

    #[test]
    fn number() {
        let toks = [Token::new_num("123.456", 0, 0)];

        let r = value_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
        let e = Some(Node::new_num("123.456", 0, 1));

        assert_eq!(r, e);
    }

    #[test]
    fn object() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];

        let r = value_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
        let e = Some(Node::new_obj(HashMap::new(), 0, 2));

        assert_eq!(r, e);
    }

    #[test]
    fn string() {
        let toks = [Token::new_str("hello world", 0, 0)];

        let r = value_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
        let e = Some(Node::new_str("hello world", 0, 1));

        assert_eq!(r, e);
    }
}
