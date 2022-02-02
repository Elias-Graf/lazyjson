use std::{iter::Peekable, rc::Rc};

use crate::tokenizer::{TokenIndices, TokenType};

use super::{config::Config, error::TreebuilderErr, node::Node, var_dict::VarDict};

pub fn string_consumer(
    toks: &mut Peekable<TokenIndices>,
    _: &Rc<VarDict>,
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
        treebuilder::{error::TreebuilderErr, node::Node, var_dict::VarDict, Config},
    };

    use super::*;

    #[test]
    fn empty_input() {
        let toks = [];
        let r = string_consumer(
            &mut toks.iter().enumerate().peekable(),
            &Rc::new(VarDict::new()),
            &Config::DEFAULT,
        )
        .unwrap_err();
        let e = TreebuilderErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_string() {
        let toks = [Token::new_kwd("false", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = string_consumer(toks_iter, &Rc::new(VarDict::new()), &Config::DEFAULT).unwrap();
        let e = None;

        assert_eq!(r, e);
        assert_eq!(
            toks_iter.next().unwrap(),
            (0, &Token::new_kwd("false", 0, 0))
        );
    }

    #[test]
    fn string() {
        let toks = [Token::new_str("hello world", 0, 0)];
        let r = string_consumer(
            &mut toks.iter().enumerate().peekable(),
            &Rc::new(VarDict::new()),
            &Config::DEFAULT,
        )
        .unwrap();
        let e = Some(Node::new_str("hello world", 0, 1));

        assert_eq!(r, e);
    }
}
