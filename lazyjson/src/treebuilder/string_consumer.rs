use std::iter::Peekable;

use crate::tokenizer::{TokenIndices, TokenType};

use super::{config::Config, error::TreebuilderErr, node::Node};

pub fn string_consumer(
    toks: &mut Peekable<TokenIndices>,
    _: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
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
        treebuilder::{error::TreebuilderErr, node::Node, DEFAULT_CONFIG},
    };

    use super::string_consumer;

    #[test]
    fn empty_input() {
        let toks = [];
        let r =
            string_consumer(&mut toks.iter().enumerate().peekable(), &DEFAULT_CONFIG).unwrap_err();
        let e = TreebuilderErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_string() {
        let toks = [Token::kwd("false", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = string_consumer(toks_iter, &DEFAULT_CONFIG).unwrap();
        let e = None;

        assert_eq!(r, e);
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::kwd("false", 0, 0)));
    }

    #[test]
    fn string() {
        let toks = [Token::str("hello world", 0, 0)];
        let r = string_consumer(&mut toks.iter().enumerate().peekable(), &DEFAULT_CONFIG).unwrap();
        let e = Some(Node::new_str("hello world", 0, 1));

        assert_eq!(r, e);
    }
}
