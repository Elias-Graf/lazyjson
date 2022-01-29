use std::iter::Peekable;

use crate::tokenizer::{TokenIndices, TokenType};

use super::{value_consumer, Config, Node, TreebuilderErr};

pub fn variable_consumer(
    inp: &mut Peekable<TokenIndices>,
    config: &Config,
) -> Result<Option<(String, Node)>, TreebuilderErr> {
    if !consume_var_kwd(inp) {
        return Ok(None);
    }

    let var_name = consume_var_name(inp)?;

    consume_assignment_op(inp)?;

    let var_value = value_consumer(inp, config)?.unwrap();

    Ok(Some((var_name, var_value)))
}

/// Returns `true` if the variable keyword was found.
fn consume_var_kwd(inp: &mut Peekable<TokenIndices>) -> bool {
    let &(_, kwd) = inp.peek().unwrap();

    if kwd.typ == TokenType::KeywordLiteral && kwd.val == "let" {
        inp.next();
        return true;
    }

    false
}

/// The ok path returns the variable name.
fn consume_var_name(inp: &mut Peekable<TokenIndices>) -> Result<String, TreebuilderErr> {
    let &(i, t) = inp.peek().unwrap();

    if t.typ == TokenType::KeywordLiteral {
        inp.next();
        return Ok(t.val.clone());
    }

    Err(TreebuilderErr::new_not_var_name(i))
}

/// Consumes the assignment operator.
fn consume_assignment_op(inp: &mut Peekable<TokenIndices>) -> Result<(), TreebuilderErr> {
    let &(i, t) = inp.peek().unwrap();

    if t.typ == TokenType::EqualAssignmentOperator {
        inp.next();
        return Ok(());
    }

    Err(TreebuilderErr::new_not_equals_assignment(i))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{tokenizer::Token, treebuilder::Node};

    use super::*;

    #[test]
    fn not_a_variable() {
        let inp = [Token::new_num("1", 0, 0)];
        let inp = &mut inp.iter().enumerate().peekable();

        assert_eq!(variable_consumer(inp, &Config::DEFAULT), Ok(None));

        // The number should not be consumed, so the next consumer can look at it.
        assert_eq!(inp.next(), Some((0, &Token::new_num("1", 0, 0))));
    }

    #[test]
    fn missing_variable_name() {
        let inp = [
            Token::new_kwd("let", 0, 0),
            Token::new_equal_assignment_op(0),
        ];
        let inp = &mut inp.iter().enumerate().peekable();

        assert_eq!(
            variable_consumer(inp, &Config::DEFAULT),
            Err(TreebuilderErr::new_not_var_name(1)),
        );
    }

    #[test]
    fn missing_assignment_op() {
        let inp = [
            Token::new_kwd("let", 0, 0),
            Token::new_kwd("num", 0, 0),
            Token::new_num("10", 0, 0),
        ];
        let inp = &mut inp.iter().enumerate().peekable();

        assert_eq!(
            variable_consumer(inp, &Config::DEFAULT),
            Err(TreebuilderErr::new_not_equals_assignment(2)),
        );
    }

    #[test]
    fn number_variable() {
        let inp = [
            Token::new_kwd("let", 0, 0),
            Token::new_kwd("num", 0, 0),
            Token::new_equal_assignment_op(0),
            Token::new_num("10", 0, 0),
        ];
        let inp = &mut inp.iter().enumerate().peekable();

        assert_eq!(
            variable_consumer(inp, &Config::DEFAULT),
            Ok(Some(("num".to_string(), Node::new_num("10", 3, 4)))),
        )
    }

    #[test]
    fn object_variable() {
        let inp = [
            Token::new_kwd("let", 0, 0),
            Token::new_kwd("obj", 0, 0),
            Token::new_equal_assignment_op(0),
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let inp = &mut inp.iter().enumerate().peekable();

        assert_eq!(
            variable_consumer(inp, &Config::DEFAULT),
            Ok(Some((
                "obj".to_string(),
                Node::new_obj(HashMap::new(), 3, 5)
            )))
        )
    }
}
