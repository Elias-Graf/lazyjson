use std::iter::Peekable;

use crate::tokenizer::{TokenIndices, TokenType};

use super::{config::Config, error::TreebuilderErr, node::Node};

pub fn number_consumer(
    toks: &mut Peekable<TokenIndices>,
    _: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let (i, t) = match toks.peek() {
        None => return Err(TreebuilderErr::new_out_of_bounds()),
        Some((_, t)) => match t.typ {
            TokenType::NumberLiteral => toks.next().unwrap(),
            _ => return Ok(None),
        },
    };

    Ok(Some(Node::new_num(&t.val, i, i + 1)))
}
