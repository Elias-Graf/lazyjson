use std::iter::Peekable;

use crate::tokenizer::{Token, TokenIndices, TokenType};

use super::{
    consumer_response::ConsumerResponse,
    error::{OldTreebuilderError, TreebuilderErr},
    node::Node,
    old_node::OldNode,
};

pub fn string_consumer(toks: &mut Peekable<TokenIndices>) -> Result<Option<Node>, TreebuilderErr> {
    let (i, t) = match toks.peek() {
        None => return Err(TreebuilderErr::new_out_of_bounds()),
        Some((_, t)) => match t.typ {
            TokenType::StringLiteral => toks.next().unwrap(),
            _ => return Ok(None),
        },
    };

    Ok(Some(Node::new_str(&t.val, i, i + 1)))
}

#[cfg(test)]
mod tests {
    use crate::{
        tokenizer::Token,
        treebuilder::{error::TreebuilderErr, node::Node},
    };

    use super::string_consumer;

    #[test]
    fn empty_input() {
        let toks = [];
        let r = string_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_string() {
        let toks = [Token::kwd("false", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = string_consumer(toks_iter).unwrap();
        let e = None;

        assert_eq!(r, e);
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::kwd("false", 0, 0)));
    }

    #[test]
    fn string() {
        let toks = [Token::str("hello world", 0, 0)];
        let r = string_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_str("hello world", 0, 1));

        assert_eq!(r, e);
    }
}

/// Consumes a string literal token. Other tokens will be ignored.
pub fn old_string_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, OldTreebuilderError> {
    let tok = toks.get(offset).unwrap();

    Ok(match tok.typ {
        TokenType::StringLiteral => {
            let n = OldNode::new_str(tok.val.as_str(), tok.clone());
            ConsumerResponse::new(1, Some(n))
        }
        _ => ConsumerResponse::new(0, None),
    })
}

#[cfg(test)]
mod old_tests {
    use super::*;

    #[test]
    pub fn non_string() {
        let r = old_string_consumer(&[Token::kwd("null", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(0, None);

        assert_eq!(r, e);
    }
    #[test]
    pub fn string() {
        let tok = Token::str("test", 0, 0);
        let r = old_string_consumer(&[tok.clone()], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_str("test", tok)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn at_offset() {
        let tok = Token::str("test", 0, 0);
        let inp = &[Token::kwd("null", 0, 0), tok.clone()];
        let r = old_string_consumer(inp, 1).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_str("test", tok)));

        assert_eq!(r, e);
    }
}
