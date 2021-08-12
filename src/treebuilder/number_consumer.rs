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
            let n = Node::new_num(tok.val.as_str(), tok.clone());
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
        let tok = Token::num("99.123", 0, 0);
        let r = number_consumer(&[tok.clone()], 0).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_num("99.123", tok)));

        assert_eq!(r, e);
    }
    #[test]
    pub fn at_offset() {
        let num_tok = Token::num("42", 0, 0);
        let inp = &[Token::kwd("null", 0, 0), num_tok.clone()];
        let r = number_consumer(inp, 1).unwrap();
        let e = ConsumerResponse::new(1, Some(Node::new_num("42", num_tok)));

        assert_eq!(r, e);
    }
}
