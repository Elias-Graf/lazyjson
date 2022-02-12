use std::rc::Rc;

use crate::{
    queue::Queue,
    tokenizer::{Token, TokenType},
};

use super::{
    config::Config,
    error::TreebuilderErr,
    node::{Node, StringNode},
    var_dict::VarDict,
};

pub fn string_consumer(
    toks: &mut Queue<Token>,
    _: &Rc<VarDict>,
    _: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let i = toks.idx();
    let t = match toks.peek() {
        None => return Err(TreebuilderErr::new_out_of_bounds()),
        Some(t) => match t.typ {
            TokenType::StringLiteral => toks.next().unwrap(),
            _ => return Ok(None),
        },
    };

    Ok(Some(StringNode::new(i, t.val.clone()).into()))
}

#[cfg(test)]
mod tests {
    use crate::treebuilder::{
        error::TreebuilderErr,
        testing::{new_kwd, new_str},
        var_dict::VarDict,
        Config,
    };

    use super::*;

    #[test]
    fn empty_input() {
        let mut toks = Queue::new(Vec::new());

        assert_eq!(
            string_consumer(&mut toks, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_out_of_bounds())
        );
    }

    #[test]
    fn non_string() {
        let mut toks = Queue::new(vec![new_kwd("false")]);

        assert_eq!(
            string_consumer(&mut toks, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(None)
        );
        assert_eq!(toks.next(), Some(&new_kwd("false")));
    }

    #[test]
    fn string() {
        let mut toks = Queue::new(vec![new_str("hello world")]);

        assert_eq!(
            string_consumer(&mut toks, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(StringNode::new(0, "hello world".to_owned()).into()))
        );
    }
}
