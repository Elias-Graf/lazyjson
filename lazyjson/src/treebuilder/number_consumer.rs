use std::{iter::Peekable, rc::Rc};

use crate::tokenizer::{TokenIndices, TokenType};

use super::{
    config::Config,
    error::TreebuilderErr,
    node::{Node, NumberNode},
    var_dict::VarDict,
};

pub fn number_consumer(
    toks: &mut Peekable<TokenIndices>,
    _: &Rc<VarDict>,
    _: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let (i, t) = match toks.peek() {
        None => return Err(TreebuilderErr::new_out_of_bounds()),
        Some((_, t)) => match t.typ {
            TokenType::NumberLiteral => toks.next().unwrap(),
            _ => return Ok(None),
        },
    };

    Ok(Some(NumberNode::new(i, t.val.clone()).into()))
}
