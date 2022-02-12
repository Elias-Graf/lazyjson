use std::rc::Rc;

use crate::{
    queue::Queue,
    tokenizer::{Token, TokenType},
};

use super::{value_consumer, Config, Node, TreebuilderErr, VarDict};

pub fn variable_definition_consumer(
    inp: &mut Queue<Token>,
    parent_var_dict: &Rc<VarDict>,
    config: &Config,
) -> Result<Option<(String, Node)>, TreebuilderErr> {
    if !consume_var_kwd(inp) {
        return Ok(None);
    }

    let var_name = consume_var_name(inp)?;

    consume_assignment_op(inp)?;

    let var_value = match inp.peek() {
        None => return Err(TreebuilderErr::new_not_a_val(inp.idx() - 1)),
        Some(_) => value_consumer(inp, parent_var_dict, config)?.unwrap(),
    };

    Ok(Some((var_name, var_value)))
}

/// Returns `true` if the variable keyword was found.
fn consume_var_kwd(inp: &mut Queue<Token>) -> bool {
    let kwd = inp.peek().unwrap();

    if kwd.typ == TokenType::KeywordLiteral && kwd.val == "let" {
        inp.next();
        return true;
    }

    false
}

/// The ok path returns the variable name.
fn consume_var_name(inp: &mut Queue<Token>) -> Result<String, TreebuilderErr> {
    let t = inp
        .peek()
        .ok_or(TreebuilderErr::new_not_var_name(inp.idx() - 1))?;

    if t.typ == TokenType::KeywordLiteral {
        return Ok(inp.next().unwrap().val.clone());
    }

    Err(TreebuilderErr::new_not_var_name(inp.idx()))
}

/// Consumes the assignment operator.
fn consume_assignment_op(inp: &mut Queue<Token>) -> Result<(), TreebuilderErr> {
    let t = inp
        .peek()
        .ok_or(TreebuilderErr::new_not_equals_assignment(inp.idx() - 1))?;
    let i = inp.idx();

    if t.typ == TokenType::EqualAssignmentOperator {
        inp.next();
        return Ok(());
    }

    Err(TreebuilderErr::new_not_equals_assignment(i))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        tokenizer::Token,
        treebuilder::{
            node::{NullNode, NumberNode, ObjectNode},
            testing::{new_equal_assignment_op, new_kwd, new_num},
        },
    };

    use super::*;

    #[test]
    fn not_a_variable() {
        let inp = &mut Queue::new(vec![new_num("1")]);

        assert_eq!(
            variable_definition_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(None)
        );

        // The number should not be consumed, so the next consumer can look at it.
        assert_eq!(inp.next(), Some(&new_num("1")));
    }

    #[test]
    fn missing_variable_name() {
        let inp = &mut Queue::new(vec![new_kwd("let")]);

        assert_eq!(
            variable_definition_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_not_var_name(0)),
        );
    }

    #[test]
    fn not_variable_name() {
        let inp = &mut Queue::new(vec![new_kwd("let"), new_equal_assignment_op()]);

        assert_eq!(
            variable_definition_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_not_var_name(1)),
        );
    }

    #[test]
    fn missing_assignment_op() {
        let inp = &mut Queue::new(vec![new_kwd("let"), new_kwd("num")]);

        assert_eq!(
            variable_definition_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_not_equals_assignment(1))
        );
    }

    #[test]
    fn not_assignment_op() {
        let inp = &mut Queue::new(vec![new_kwd("let"), new_kwd("num"), new_num("10")]);

        assert_eq!(
            variable_definition_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_not_equals_assignment(2)),
        );
    }

    #[test]
    fn missing_value_composition() {
        let inp = &mut Queue::new(vec![
            new_kwd("let"),
            new_kwd("foo"),
            new_equal_assignment_op(),
        ]);

        assert_eq!(
            variable_definition_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_not_a_val(2)),
        );
    }

    #[test]
    fn number_variable() {
        let inp = &mut Queue::new(vec![
            new_kwd("let"),
            new_kwd("num"),
            new_equal_assignment_op(),
            new_num("10"),
        ]);

        assert_eq!(
            variable_definition_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some((
                "num".to_string(),
                NumberNode::new(3, "10".to_owned()).into()
            ))),
        )
    }

    #[test]
    fn object_variable() {
        let inp = &mut Queue::new(vec![
            Token::new_kwd("let", 0, 0),
            Token::new_kwd("obj", 0, 0),
            Token::new_equal_assignment_op(0),
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ]);

        assert_eq!(
            variable_definition_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some((
                "obj".to_string(),
                ObjectNode::new(3, 5, HashMap::new(),).into()
            )))
        )
    }

    #[test]
    fn can_use_variable_from_parent_var_dict() {
        let inp = &mut Queue::new(vec![
            Token::new_kwd("let", 0, 0),
            Token::new_kwd("var", 0, 0),
            Token::new_equal_assignment_op(0),
            Token::new_kwd("parent_var", 0, 0),
        ]);

        let mut parent_var_dict = VarDict::new();
        parent_var_dict.insert("parent_var".into(), NullNode::new(0).into());

        assert_eq!(
            variable_definition_consumer(inp, &Rc::new(parent_var_dict), &Config::DEFAULT),
            Ok(Some(("var".into(), NullNode::new(0).into())))
        );
    }
}
