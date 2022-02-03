use std::{iter::Peekable, rc::Rc};

use crate::tokenizer::{TokenIndices, TokenType};

use super::{var_dict::VarDict, Config, Node, TreebuilderErr};

pub fn variable_usage_consumer(
    inp: &mut Peekable<TokenIndices>,
    var_dict: &Rc<VarDict>,
    _: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let &(i, tok) = inp.peek().unwrap();

    if tok.typ != TokenType::KeywordLiteral {
        return Ok(None);
    }

    if let Some(n) = var_dict.get(&tok.val) {
        inp.next();

        return Ok(Some(n.clone()));
    }

    Err(TreebuilderErr::new_undeclared_variable(i))
}

#[cfg(test)]
mod tests {
    use crate::{
        tokenizer::Token,
        treebuilder::{node::BoolNode, var_dict::VarDict, Node, TreebuilderErr},
    };

    use super::*;

    #[test]
    fn non_variable_is_not_consumed() {
        let inp = [Token::new_str("false", 0, 0)];
        let inp = &mut inp.iter().enumerate().peekable();

        assert_eq!(
            variable_usage_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(None)
        );
        assert_eq!(inp.next(), Some((0, &Token::new_str("false", 0, 0))));
    }

    #[test]
    fn unknown_variable_results_in_an_error_pos_0() {
        let inp = [Token::new_kwd("undeclared_var", 0, 0)];
        let inp = &mut inp.iter().enumerate().peekable();

        assert_eq!(
            variable_usage_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_undeclared_variable(0)),
        );
    }

    #[test]
    fn unknown_variable_results_in_an_error_pos_1() {
        let inp = [
            Token::new_kwd("null", 0, 0),
            Token::new_kwd("undeclared_var", 0, 0),
        ];
        let inp = &mut inp.iter().enumerate().peekable();
        inp.next();

        assert_eq!(
            variable_usage_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_undeclared_variable(1)),
        );
    }

    #[test]
    fn known_variable_is_consumed_and_results_in_the_corresponding_node() {
        let inp = [Token::new_kwd("foo", 0, 0)];
        let inp = &mut inp.iter().enumerate().peekable();

        let mut var_dict = VarDict::new();
        var_dict.insert("foo".into(), BoolNode::new(0, true).into());

        assert_eq!(
            variable_usage_consumer(inp, &Rc::new(var_dict), &Config::DEFAULT),
            Ok(Some(BoolNode::new(0, true).into())),
        );
        assert_eq!(inp.next(), None);
    }
}
