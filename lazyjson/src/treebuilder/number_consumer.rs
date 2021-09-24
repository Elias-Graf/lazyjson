use std::iter::Peekable;

use crate::tokenizer::{Token, TokenIndices, TokenType};

use super::{
    consumer_response::ConsumerResponse,
    error::{OldTreebuilderError, TreebuilderErr},
    node::Node,
    old_node::OldNode,
};

pub fn number_consumer(toks: &mut Peekable<TokenIndices>) -> Result<Option<Node>, TreebuilderErr> {
    let (i, t) = match toks.peek() {
        None => return Err(TreebuilderErr::new_out_of_bounds()),
        Some((_, t)) => match t.typ {
            TokenType::NumberLiteral => toks.next().unwrap(),
            _ => return Ok(None),
        },
    };

    Ok(Some(Node::new_num(&t.val, i, i + 1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let toks = [];
        let r = number_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_number() {
        let toks = [Token::kwd("false", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = number_consumer(toks_iter).unwrap();

        assert_eq!(r, None);
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::kwd("false", 0, 0)));
    }

    #[test]
    fn number() {
        let toks = [Token::num("123.456", 0, 0)];
        let r = number_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_num("123.456", 0, 1));

        assert_eq!(r, e);
    }
}

// Consumes a number literal. Non number literals are ignored.
pub fn old_number_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, OldTreebuilderError> {
    let tok = toks.get(offset).unwrap();

    Ok(match tok.typ {
        TokenType::NumberLiteral => {
            let n = OldNode::new_num(tok.val.as_str(), tok.clone());
            ConsumerResponse::new(1, Some(n))
        }
        _ => ConsumerResponse::new(0, None),
    })
}

#[cfg(test)]
mod old_tests {
    use super::*;

    #[test]
    fn non_number() {
        let r = old_number_consumer(&[Token::kwd("null", 0, 0)], 0).unwrap();
        let e = ConsumerResponse::new(0, None);

        assert_eq!(r, e);
    }

    #[test]
    fn number() {
        let tok = Token::num("99.123", 0, 0);
        let r = old_number_consumer(&[tok.clone()], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_num("99.123", tok)));

        assert_eq!(r, e);
    }
    #[test]
    fn at_offset() {
        let num_tok = Token::num("42", 0, 0);
        let inp = &[Token::kwd("null", 0, 0), num_tok.clone()];
        let r = old_number_consumer(inp, 1).unwrap();
        let e = ConsumerResponse::new(1, Some(OldNode::new_num("42", num_tok)));

        assert_eq!(r, e);
    }
}
