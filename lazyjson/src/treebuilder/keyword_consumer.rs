use std::{iter::Peekable, rc::Rc};

use crate::{
    tokenizer::{TokenIndices, TokenType},
    treebuilder::error::TreebuilderErr,
};

use super::{config::Config, node::Node, var_dict::VarDict};

pub fn keyword_consumer(
    toks: &mut Peekable<TokenIndices>,
    _: &Rc<VarDict>,
    _: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let &(i, t) = toks.peek().unwrap();

    if t.typ != TokenType::KeywordLiteral {
        return Ok(None);
    }

    let n = match t.val.as_str() {
        "false" => Node::new_bool(false, i, i + 1),
        "null" => Node::new_null(i, i + 1),
        "true" => Node::new_bool(true, i, i + 1),
        _ => return Ok(None),
    };

    toks.next();

    Ok(Some(n))
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer::Token, treebuilder::Config};

    use super::*;

    #[test]
    pub fn non_keyword() {
        let toks = [Token::new_num("123", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = keyword_consumer(toks_iter, &Rc::new(VarDict::new()), &Config::DEFAULT).unwrap();

        assert_eq!(r, None);
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::new_num("123", 0, 0)));
    }

    #[test]
    pub fn consume_false() {
        assert_correct_consume(Token::new_kwd("false", 0, 0), Node::new_bool(false, 0, 1));
    }

    #[test]
    pub fn consume_null() {
        assert_correct_consume(Token::new_kwd("null", 0, 0), Node::new_null(0, 1));
    }

    #[test]
    pub fn consume_true() {
        assert_correct_consume(Token::new_kwd("true", 0, 0), Node::new_bool(true, 0, 1));
    }

    fn assert_correct_consume(tok: Token, exp: Node) {
        let toks = [tok];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = keyword_consumer(toks_iter, &Rc::new(VarDict::new()), &Config::DEFAULT).unwrap();
        let e = Some(exp);

        assert_eq!(r, e);
        assert_eq!(toks_iter.next(), None);
    }
}
