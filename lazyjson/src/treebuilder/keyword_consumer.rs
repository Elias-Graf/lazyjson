use std::iter::Peekable;

use crate::{
    tokenizer::{TokenIndices, TokenType},
    treebuilder::error::TreebuilderErr,
};

use super::node::Node;

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
    use crate::tokenizer::Token;

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
