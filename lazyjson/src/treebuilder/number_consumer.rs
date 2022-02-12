use std::rc::Rc;

use crate::{
    queue::Queue,
    tokenizer::{Token, TokenType},
};

use super::{
    config::Config,
    error::TreebuilderErr,
    node::{Node, NumberNode},
    var_dict::VarDict,
};

pub fn number_consumer(
    inp: &mut Queue<Token>,
    _: &Rc<VarDict>,
    _: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let t = inp.peek().ok_or(TreebuilderErr::new_out_of_bounds())?;

    if t.typ != TokenType::NumberLiteral {
        return Ok(None);
    }

    let i = inp.idx();
    let t = inp.next().unwrap();

    Ok(Some(NumberNode::new(i, t.val.clone()).into()))
}
