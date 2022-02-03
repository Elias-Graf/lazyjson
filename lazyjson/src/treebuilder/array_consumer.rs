use std::iter::Peekable;
use std::rc::Rc;

use crate::tokenizer::{Token, TokenIndices, TokenType};
use crate::treebuilder::variable_definition_consumer::variable_definition_consumer;

use super::config::Config;
use super::node::ArrayNode;
use super::var_dict::VarDict;
use super::{error::TreebuilderErr, node::Node, value_consumer::value_consumer};

pub fn array_consumer(
    inp: &mut Peekable<TokenIndices>,
    parent_var_dict: &Rc<VarDict>,
    config: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let (opn_i, _) = match consume_arr_opn(inp) {
        None => return Ok(None),
        Some(o) => o,
    };

    if let Some((cls_i, _)) = consume_arr_cls(inp, opn_i)? {
        return Ok(Some(
            ArrayNode::new(opn_i, cls_i + 1, Vec::new(), VarDict::new()).into(),
        ));
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

        if let Some((cls_i, _)) = consume_arr_cls(inp, opn_i)? {
            return Ok(Some(
                ArrayNode::new(opn_i, cls_i + 1, entries, var_dict).into(),
            ));
        }

        consume_val_sep(inp)?;

        // Check if the next token is an array close, if yes, we have a trailing
        // separator.
        if let Some((cls_i, _)) = consume_arr_cls(inp, opn_i)? {
            if !config.allow_trailing_commas {
                return Err(TreebuilderErr::new_trailing_sep(cls_i - 1));
            }

            return Ok(Some(
                ArrayNode::new(opn_i, cls_i + 1, entries, var_dict).into(),
            ));
        }
    }
}

fn consume_arr_opn<'a>(inp: &'a mut Peekable<TokenIndices>) -> Option<(usize, &'a Token)> {
    let &(_, t) = inp.peek().unwrap();

    if t.typ == TokenType::Delimiter && t.val == "[" {
        return inp.next();
    }

    None
}

fn consume_arr_cls<'a>(
    inp: &'a mut Peekable<TokenIndices>,
    opn_i: usize,
) -> Result<Option<(usize, &'a Token)>, TreebuilderErr> {
    let &(_, t) = inp
        .peek()
        .ok_or(TreebuilderErr::new_unterminated_arr(opn_i))?;

    if t.typ == TokenType::Delimiter && t.val == "]" {
        return Ok(inp.next());
    }

    Ok(None)
}

fn consume_val_sep(inp: &mut Peekable<TokenIndices>) -> Result<(), TreebuilderErr> {
    let &(i, t) = inp.peek().unwrap();

    if t.typ != TokenType::Separator || t.val != "," {
        return Err(TreebuilderErr::new_not_a_sep(i));
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

    use crate::{
        tokenizer::Token,
        treebuilder::{
            node::{ArrayNode, BoolNode, NumberNode, ObjectNode},
            testing::{self, inp_from},
            Config,
        },
    };

    use super::*;

    #[test]
    fn non_array() {
        let toks = [Token::new_num("0", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();

        assert_eq!(
            array_consumer(toks_iter, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(None)
        );
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::new_num("0", 0, 0)));
    }

    #[test]
    fn unterminated() {
        let toks = [Token::new_delimiter("[", 0, 0)];

        assert_eq!(
            array_consumer(
                &mut toks.iter().enumerate().peekable(),
                &Rc::new(VarDict::new()),
                &Config::DEFAULT,
            ),
            Err(TreebuilderErr::new_unterminated_arr(0))
        );
    }

    #[test]
    fn missing_sep() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_num("1", 0, 0),
            Token::new_num("1", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];

        assert_eq!(
            array_consumer(
                &mut toks.iter().enumerate().peekable(),
                &Rc::new(VarDict::new()),
                &Config::DEFAULT,
            ),
            Err(TreebuilderErr::new_not_a_sep(2))
        );
    }

    #[test]
    fn invalid_val() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_num("1", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_delimiter("]", 0, 0),
        ];

        assert_eq!(
            array_consumer(
                &mut toks.iter().enumerate().peekable(),
                &Rc::new(VarDict::new()),
                &Config::DEFAULT,
            ),
            Err(TreebuilderErr::new_not_a_val(3))
        );
    }

    #[test]
    fn trailing_sep_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = true;

        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_num("123", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let inp = &mut toks.iter().enumerate().peekable();

        let exp_var_dict = VarDict::new_with_parent(&Rc::new(VarDict::new()));
        let exp_arr = ArrayNode::new(
            0,
            4,
            vec![NumberNode::new(1, "123".to_owned()).into()],
            exp_var_dict,
        );

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &config),
            Ok(Some(exp_arr.into()))
        );
        // The closing bracket should be consumed
        assert_eq!(inp.next(), None);
    }

    #[test]
    fn trailing_sep_not_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = false;

        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_num("123", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];

        assert_eq!(
            array_consumer(
                &mut toks.iter().enumerate().peekable(),
                &Rc::new(VarDict::new()),
                &config,
            ),
            Err(TreebuilderErr::new_trailing_sep(2))
        );
    }

    #[test]
    fn empty() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT,),
            Ok(Some(
                ArrayNode::new(0, 2, Vec::new(), VarDict::new()).into()
            ))
        );
    }

    #[test]
    fn single_entry() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_num("123", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let inp = &mut inp_from(&toks);

        let exp_var_dict = VarDict::new_with_parent(&Rc::new(VarDict::new()));
        let exp_arr = ArrayNode::new(
            0,
            3,
            vec![NumberNode::new(1, "123".to_owned()).into()],
            exp_var_dict,
        );

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(exp_arr.into()))
        );
    }

    #[test]
    fn multiple_entries() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("]", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_kwd("false", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_num("123", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("Hello, World!", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        let a = array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT);
        let b = Ok(Some(
            ArrayNode::new(
                0,
                13,
                vec![
                    ArrayNode::new(1, 3, Vec::new(), VarDict::new()).into(),
                    BoolNode::new(4, false).into(),
                    NumberNode::new(6, "123".to_owned()).into(),
                    ObjectNode::new(
                        8,
                        10,
                        HashMap::new(),
                        VarDict::new_with_parent(&Rc::new(VarDict::new_with_parent(&Rc::new(
                            VarDict::new(),
                        )))),
                    )
                    .into(),
                    Node::new_str("Hello, World!", 11, 12),
                ],
                VarDict::new_with_parent(&Rc::new(VarDict::new())),
            )
            .into(),
        ));

        dbg!(&a, &b);

        assert_eq!(a, b);
    }

    #[test]
    fn declare_variable() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_kwd("let", 0, 0),
            Token::new_kwd("foo", 0, 0),
            Token::new_equal_assignment_op(0),
            Token::new_str("foo", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("bar", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        let mut exp_entries = Vec::new();
        exp_entries.push(Node::new_str("bar", 6, 7));

        let mut exp_var_dict = VarDict::new_with_parent(&Rc::new(VarDict::new()));
        exp_var_dict.insert("foo".into(), Node::new_str("foo", 4, 5));

        let exp_arr = ArrayNode::new(0, 8, exp_entries, exp_var_dict);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(exp_arr.into()))
        )
    }

    #[test]
    fn declare_variable_with_trailing_sep() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = true;

        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_kwd("let", 0, 0),
            Token::new_kwd("foo", 0, 0),
            Token::new_equal_assignment_op(0),
            Token::new_str("foo", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        let mut exp_var_dict = VarDict::new_with_parent(&Rc::new(VarDict::new()));
        exp_var_dict.insert("foo".into(), Node::new_str("foo", 4, 5));

        let exp_arr = ArrayNode::new(0, 7, Vec::new(), exp_var_dict);

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &config),
            Ok(Some(exp_arr.into())),
        )
    }

    #[test]
    fn use_variable_of_parent() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_kwd("foo", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        let mut var_dict = VarDict::new();
        var_dict.insert("foo".into(), NumberNode::new(0, "10".to_owned()).into());
        let var_dict = &Rc::new(var_dict);

        let exp_arr = ArrayNode::new(
            0,
            3,
            vec![NumberNode::new(0, "10".to_owned()).into()],
            VarDict::new_with_parent(var_dict),
        );

        assert_eq!(
            array_consumer(inp, var_dict, &Config::DEFAULT),
            Ok(Some(exp_arr.into()))
        )
    }

    #[test]
    fn use_variable_of_self() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_kwd("let", 0, 0),
            Token::new_kwd("foo", 0, 0),
            Token::new_equal_assignment_op(0),
            Token::new_str("bar", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_kwd("foo", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        let mut exp_var_dict = VarDict::new_with_parent(&Rc::new(VarDict::new()));
        exp_var_dict.insert("foo".into(), Node::new_str("bar", 4, 5).into());

        assert_eq!(
            array_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(
                ArrayNode::new(0, 8, vec![Node::new_str("bar", 4, 5)], exp_var_dict).into()
            ))
        )
    }
}
