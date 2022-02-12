use std::rc::Rc;

use crate::{
    queue::Queue,
    tokenizer::{Token, TokenType},
    treebuilder::error::TreebuilderErr,
};

use super::{
    config::Config,
    node::{BoolNode, Node, NullNode},
    var_dict::VarDict,
};

pub fn keyword_consumer(
    toks: &mut Queue<Token>,
    _: &Rc<VarDict>,
    _: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let i = toks.idx();
    let t = toks.peek().unwrap();

    if t.typ != TokenType::KeywordLiteral {
        return Ok(None);
    }

    let n = match t.val.as_str() {
        "false" => BoolNode::new(i, false).into(),
        "null" => NullNode::new(i).into(),
        "true" => BoolNode::new(i, true).into(),
        _ => return Ok(None),
    };

    toks.next();

    Ok(Some(n))
}

#[cfg(test)]
mod tests {
    use crate::{
        tokenizer::Token,
        treebuilder::{testing::new_num, Config},
    };

    use super::*;

    #[test]
    pub fn non_keyword() {
        let inp = &mut Queue::new(vec![new_num("123")]);

        assert_eq!(
            keyword_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(None)
        );
        assert_eq!(inp.next(), Some(&Token::new_num("123", 0, 0)));
    }

    #[test]
    pub fn consume_false() {
        assert_correct_consume(
            Token::new_kwd("false", 0, 0),
            BoolNode::new(0, false).into(),
        );
    }

    #[test]
    pub fn consume_null() {
        assert_correct_consume(Token::new_kwd("null", 0, 0), NullNode::new(0).into());
    }

    #[test]
    pub fn consume_true() {
        assert_correct_consume(Token::new_kwd("true", 0, 0), BoolNode::new(0, true).into());
    }

    fn assert_correct_consume(tok: Token, exp: Node) {
        let inp = &mut Queue::new(vec![tok]);

        assert_eq!(
            keyword_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(exp))
        );
        assert_eq!(inp.next(), None);
    }
}
