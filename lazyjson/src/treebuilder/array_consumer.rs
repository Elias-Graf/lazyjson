use std::rc::Rc;

use crate::queue::Queue;
use crate::tokenizer::{Token, TokenType};
use crate::treebuilder::variable_definition_consumer::variable_definition_consumer;

use super::config::Config;
use super::node::ArrayNode;
use super::var_dict::VarDict;
use super::{error::TreebuilderErr, node::Node, value_consumer::value_consumer};

pub fn array_consumer(
    inp: &mut Queue<Token>,
    parent_var_dict: &Rc<VarDict>,
    config: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let opn_i = inp.idx();

    if !consume_arr_opn(inp) {
        return Ok(None);
    }

    if consume_arr_cls(inp, opn_i)? {
        return Ok(Some(ArrayNode::new(opn_i, inp.idx(), Vec::new()).into()));
    }

    let mut entries = Vec::new();
    let mut var_dict = VarDict::new_with_parent(parent_var_dict);

    loop {
        if let Some((key, val)) =
            // TODO: figure out how to do this without cloning
            variable_definition_consumer(inp, &Rc::new(var_dict.clone()), config)?
        {
            var_dict.insert(key, val);
        } else {
            let entry = value_consumer(
                inp,
                // TODO: figure out how to do this without cloning
                &Rc::new(var_dict.clone()),
                &Config::DEFAULT,
            )?
            .ok_or(TreebuilderErr::new_not_a_val(
                get_last_tok_idx(&entries) + 1,
            ))?;

            entries.push(entry);
        }

        if consume_arr_cls(inp, opn_i)? {
            return Ok(Some(ArrayNode::new(opn_i, inp.idx(), entries).into()));
        }

        consume_val_sep(inp)?;

        // Check if the next token is an array close, if yes, we have a trailing
        // separator.
        if consume_arr_cls(inp, opn_i)? {
            if !config.allow_trailing_commas {
                return Err(TreebuilderErr::new_trailing_sep(inp.idx() - 2));
            }

            return Ok(Some(ArrayNode::new(opn_i, inp.idx(), entries).into()));
        }
    }
}

fn consume_arr_opn<'a>(inp: &'a mut Queue<Token>) -> bool {
    let t = inp.peek().unwrap();

    if t.typ == TokenType::Delimiter && t.val == "[" {
        inp.next();

        return true;
    }

    false
}

fn consume_arr_cls<'a>(inp: &'a mut Queue<Token>, opn_i: usize) -> Result<bool, TreebuilderErr> {
    let t = inp
        .peek()
        .ok_or(TreebuilderErr::new_unterminated_arr(opn_i))?;

    if t.typ == TokenType::Delimiter && t.val == "]" {
        inp.next();

        return Ok(true);
    }

    Ok(false)
}

fn consume_val_sep(inp: &mut Queue<Token>) -> Result<(), TreebuilderErr> {
    let t = inp.peek().unwrap();

    if t.typ != TokenType::Separator || t.val != "," {
        return Err(TreebuilderErr::new_not_a_sep(inp.idx()));
    }

    inp.next();

    Ok(())
}

fn get_last_tok_idx(entries: &Vec<Node>) -> usize {
    match entries.last() {
        None => 0,
        Some(e) => e.to(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::treebuilder::{
        node::{ArrayNode, BoolNode, NumberNode, ObjectNode, StringNode},
        testing::{
            new_delimiter, new_equal_assignment_op, new_json_assignment_op, new_kwd, new_num,
            new_sep, new_str,
        },
        Config,
    };

    use super::*;

    #[test]
    fn non_array() {
        let inp = &mut Queue::new(vec![new_num("0")]);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(None)
        );
        assert_eq!(inp.next(), Some(&new_num("0")));
    }

    #[test]
    fn unterminated() {
        let inp = &mut Queue::new(vec![new_delimiter("[")]);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT,),
            Err(TreebuilderErr::new_unterminated_arr(0))
        );
    }

    #[test]
    fn missing_sep() {
        let inp = &mut Queue::new(vec![
            new_delimiter("["),
            new_num("1"),
            new_num("1"),
            new_delimiter("]"),
        ]);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_not_a_sep(2))
        );
    }

    #[test]
    fn invalid_val() {
        let inp = &mut Queue::new(vec![
            new_delimiter("["),
            new_num("1"),
            new_sep(","),
            new_json_assignment_op(),
            new_delimiter("]"),
        ]);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT,),
            Err(TreebuilderErr::new_not_a_val(3))
        );
    }

    #[test]
    fn trailing_sep_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = true;

        let inp = &mut Queue::new(vec![
            new_delimiter("["),
            new_num("123"),
            new_sep(","),
            new_delimiter("]"),
        ]);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &config),
            Ok(Some(
                ArrayNode::new(0, 4, vec![NumberNode::new(1, "123".to_owned()).into()]).into()
            ))
        );
        // The closing bracket should be consumed
        assert_eq!(inp.next(), None);
    }

    #[test]
    fn trailing_sep_not_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = false;

        let inp = &mut Queue::new(vec![
            new_delimiter("["),
            new_num("123"),
            new_sep(","),
            new_delimiter("]"),
        ]);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &config),
            Err(TreebuilderErr::new_trailing_sep(2))
        );
    }

    #[test]
    fn empty() {
        let inp = &mut Queue::new(vec![new_delimiter("["), new_delimiter("]")]);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(ArrayNode::new(0, 2, Vec::new()).into()))
        );
    }

    #[test]
    fn single_entry() {
        let inp = &mut Queue::new(vec![new_delimiter("["), new_num("123"), new_delimiter("]")]);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(
                ArrayNode::new(0, 3, vec![NumberNode::new(1, "123".to_owned()).into()]).into()
            ))
        );
    }

    #[test]
    fn multiple_entries() {
        let inp = &mut Queue::new(vec![
            new_delimiter("["),
            new_delimiter("["),
            new_delimiter("]"),
            new_sep(","),
            new_kwd("false"),
            new_sep(","),
            new_num("123"),
            new_sep(","),
            new_delimiter("{"),
            new_delimiter("}"),
            new_sep(","),
            new_str("Hello, World!"),
            new_delimiter("]"),
        ]);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(
                ArrayNode::new(
                    0,
                    13,
                    vec![
                        ArrayNode::new(1, 3, Vec::new()).into(),
                        BoolNode::new(4, false).into(),
                        NumberNode::new(6, "123".to_owned()).into(),
                        ObjectNode::new(8, 10, HashMap::new()).into(),
                        StringNode::new(11, "Hello, World!".to_owned()).into(),
                    ],
                )
                .into(),
            ))
        );
    }

    #[test]
    fn use_variable_of_parent() {
        let inp = &mut Queue::new(vec![new_delimiter("["), new_kwd("foo"), new_delimiter("]")]);

        let mut var_dict = VarDict::new();
        var_dict.insert("foo".into(), NumberNode::new(0, "10".to_owned()).into());
        let var_dict = &Rc::new(var_dict);

        let exp_arr = ArrayNode::new(0, 3, vec![NumberNode::new(0, "10".to_owned()).into()]);

        assert_eq!(
            array_consumer(inp, var_dict, &Config::DEFAULT),
            Ok(Some(exp_arr.into()))
        )
    }

    #[test]
    fn declare_and_use_variable() {
        let inp = &mut Queue::new(vec![
            new_delimiter("["),
            new_kwd("let"),
            new_kwd("foo"),
            new_equal_assignment_op(),
            new_str("bar"),
            new_sep(","),
            new_kwd("foo"),
            new_delimiter("]"),
        ]);

        let mut exp_var_dict = VarDict::new_with_parent(&Rc::new(VarDict::new()));
        exp_var_dict.insert("foo".into(), StringNode::new(4, "bar".to_owned()).into());

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(
                ArrayNode::new(0, 8, vec![StringNode::new(4, "bar".to_owned()).into()]).into()
            ))
        )
    }

    #[test]
    fn declare_and_use_variable_with_trailing_sep() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = true;

        let inp = &mut Queue::new(vec![
            new_delimiter("["),
            new_kwd("let"),
            new_kwd("foo"),
            new_equal_assignment_op(),
            new_str("bar"),
            new_sep(","),
            new_kwd("foo"),
            new_sep(","),
            new_delimiter("]"),
        ]);

        let mut exp_var_dict = VarDict::new_with_parent(&Rc::new(VarDict::new()));
        exp_var_dict.insert("foo".into(), StringNode::new(4, "bar".to_owned()).into());

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &config),
            Ok(Some(
                ArrayNode::new(0, 9, vec![StringNode::new(4, "bar".to_owned()).into()]).into()
            ))
        )
    }
}
