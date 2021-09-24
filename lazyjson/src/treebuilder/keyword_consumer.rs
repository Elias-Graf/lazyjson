use std::iter::Peekable;

use crate::{
    tokenizer::{Token, TokenIndices, TokenType},
    treebuilder::error::TreebuilderErr,
};

use super::{
    consumer_response::ConsumerResponse, error::OldTreebuilderError, node::Node, old_node::OldNode,
};

pub fn keyword_consumer(toks: &mut Peekable<TokenIndices>) -> Result<Option<Node>, TreebuilderErr> {
    let (i, t) = match toks.peek() {
        None => return Err(TreebuilderErr::new_out_of_bounds()),
        Some(r) => *r,
    };

    if t.typ != TokenType::KeywordLiteral {
        return Ok(None);
    }

    let n = match t.val.as_str() {
        "false" => Node::new_bool(false, i, i + 1),
        "null" => Node::new_null(i, i + 1),
        "true" => Node::new_bool(true, i, i + 1),
        _ => return Err(TreebuilderErr::new_unknown_kwd(i)),
    };

    toks.next();

    Ok(Some(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let toks = [];
        let r = keyword_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    pub fn non_keyword() {
        let toks = [Token::num("123", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = keyword_consumer(toks_iter).unwrap();

        assert_eq!(r, None);
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::num("123", 0, 0)));
    }

    #[test]
    pub fn consume_false() {
        assert_correct_consume(Token::kwd("false", 0, 0), Node::new_bool(false, 0, 1));
    }

    #[test]
    pub fn consume_null() {
        assert_correct_consume(Token::kwd("null", 0, 0), Node::new_null(0, 1));
    }

    #[test]
    pub fn consume_true() {
        assert_correct_consume(Token::kwd("true", 0, 0), Node::new_bool(true, 0, 1));
    }

    fn assert_correct_consume(tok: Token, exp: Node) {
        let toks = [tok];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = keyword_consumer(toks_iter).unwrap();
        let e = Some(exp);

        assert_eq!(r, e);
        assert_eq!(toks_iter.next(), None);
    }
}

/// Consumes known keyword tokens. Unknown tokens result in an error. Non
/// keyword tokens are ignored.
pub fn old_keyword_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, OldTreebuilderError> {
    let tok = toks.get(offset).unwrap();

    if tok.typ != TokenType::KeywordLiteral {
        return Ok(ConsumerResponse::new(0, None));
    }

    match tok.val.as_str() {
        "false" => Ok(ConsumerResponse::new(
            1,
            Some(OldNode::new_bool(false, tok.clone())),
        )),
        "null" => Ok(ConsumerResponse::new(
            1,
            Some(OldNode::new_null(tok.clone())),
        )),
        "true" => Ok(ConsumerResponse::new(
            1,
            Some(OldNode::new_bool(true, tok.clone())),
        )),
        _ => Err(OldTreebuilderError::new_unknown_kwd(tok.clone())),
    }
}

#[cfg(test)]
mod old_tests {
    use super::*;

    #[test]
    pub fn non_keyword() {
        let r = old_keyword_consumer(&[Token::str("non token", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(0, None);

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_false() {
        let tok = Token::kwd("false", 0, 0);
        let r = old_keyword_consumer(&[tok.clone()], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_bool(false, tok)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_null() {
        let tok = Token::kwd("null", 0, 0);
        let r = old_keyword_consumer(&[tok.clone()], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_null(tok)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn consume_true() {
        let tok = Token::kwd("true", 0, 0);
        let r = old_keyword_consumer(&[tok.clone()], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_bool(true, tok)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn at_offset() {
        let null_tok = Token::kwd("null", 0, 0);
        let inp = &[Token::str("placeholder", 0, 0), null_tok.clone()];
        let r = old_keyword_consumer(inp, 1).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_null(null_tok)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn unknown_keyword() {
        let misspelled_true = Token::kwd("tru", 0, 0);
        let r = old_keyword_consumer(&[misspelled_true.clone()], 0).unwrap_err();
        let e = OldTreebuilderError::new_unknown_kwd(misspelled_true);

        assert_eq!(r, e);
    }
}
