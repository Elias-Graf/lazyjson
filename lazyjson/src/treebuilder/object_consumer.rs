use super::{
    config::Config,
    error::TreebuilderErr,
    node::{Node, ObjectNode},
    value_consumer::value_consumer,
    var_dict::VarDict,
    variable_definition_consumer::variable_definition_consumer,
};
use crate::tokenizer::{Token, TokenIndices, TokenType};
use std::{collections::HashMap, iter::Peekable, rc::Rc};

pub fn object_consumer(
    inp: &mut Peekable<TokenIndices>,
    var_dict: &Rc<VarDict>,
    config: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let (opn_i, _) = match consume_obj_opn(inp) {
        None => return Ok(None),
        Some(o) => o,
    };

    let mut entries = HashMap::new();
    let mut var_dict = VarDict::new_with_parent(var_dict);

    // Check if the object is immediately closed again (empty).
    if let Some((cls_i, _)) = consume_obj_cls(inp, opn_i)? {
        return Ok(Some(
            ObjectNode::new(opn_i, cls_i + 1, entries, var_dict).into(),
        ));
    }

    loop {
        if let Some((var_key, var_val)) =
            // TODO: figure out how to do this without cloning
            variable_definition_consumer(inp, &Rc::new(var_dict.clone()), config)?
        {
            var_dict.insert(var_key, var_val);
        } else {
            let (key_i, key) = consume_key(inp)?;

            consume_assignment(inp, key_i)?;

            let val = match value_consumer(
                inp,
                // TODO: figure out how to do this without cloning
                &Rc::new(var_dict.clone()),
                &Config::DEFAULT,
            )? {
                None => return Err(TreebuilderErr::new_not_a_val(inp.next().unwrap().0)),
                Some(v) => v,
            };
            entries.insert(key, val);
        }

        if let Some((cls_i, _)) = consume_obj_cls(inp, opn_i)? {
            return Ok(Some(
                ObjectNode::new(opn_i, cls_i + 1, entries, var_dict).into(),
            ));
        }

        consume_val_sep(inp)?;

        // Check if the next token is an object close, if yes, we have a trailing
        // separator.
        if let Some((cls_i, _)) = consume_obj_cls(inp, opn_i)? {
            if !config.allow_trailing_commas {
                return Err(TreebuilderErr::new_trailing_sep(cls_i - 1));
            }

            return Ok(Some(
                ObjectNode::new(opn_i, cls_i + 1, entries, var_dict).into(),
            ));
        }
    }
}

/// Returns the token if a object open delimiter was found.
fn consume_obj_opn<'a>(inp: &'a mut Peekable<TokenIndices>) -> Option<(usize, &'a Token)> {
    let &(_, t) = inp.peek().unwrap();

    if t.typ == TokenType::Delimiter && t.val == "{" {
        return inp.next();
    }

    None
}

fn consume_obj_cls<'a>(
    inp: &'a mut Peekable<TokenIndices>,
    opn_i: usize,
) -> Result<Option<(usize, &'a Token)>, TreebuilderErr> {
    let &(_, t) = inp
        .peek()
        .ok_or(TreebuilderErr::new_unterminated_obj(opn_i))?;

    if t.typ == TokenType::Delimiter && t.val == "}" {
        return Ok(inp.next());
    }

    Ok(None)
}

fn consume_key<'a>(inp: &'a mut Peekable<TokenIndices>) -> Result<(usize, String), TreebuilderErr> {
    let &(i, t) = inp.peek().unwrap();

    if t.typ == TokenType::StringLiteral {
        inp.next();

        return Ok((i, t.val.clone()));
    }

    Err(TreebuilderErr::new_not_a_key(i))
}

fn consume_assignment(
    inp: &mut Peekable<TokenIndices>,
    key_i: usize,
) -> Result<(), TreebuilderErr> {
    let (i, t) = inp
        .next()
        .ok_or(TreebuilderErr::new_unterminated_obj(key_i))?;

    if t.typ != TokenType::JsonAssignmentOperator {
        return Err(TreebuilderErr::new_not_an_assignment(i));
    }

    Ok(())
}

fn consume_val_sep(inp: &mut Peekable<TokenIndices>) -> Result<(), TreebuilderErr> {
    let &(i, t) = inp.peek().unwrap();

    if t.typ != TokenType::Separator || t.val != "," {
        return Err(TreebuilderErr::new_not_a_sep(i));
    }

    inp.next();

    return Ok(());
}

#[cfg(test)]
mod tests {
    use crate::{
        tokenizer::Token,
        treebuilder::{
            node::{ArrayNode, BoolNode, NumberNode, StringNode},
            testing,
        },
    };

    use super::*;

    #[test]
    fn non_object() {
        let toks = [Token::new_num("123", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = object_consumer(toks_iter, &Rc::new(VarDict::new()), &Config::DEFAULT).unwrap();
        let e = None;

        assert_eq!(r, e);
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::new_num("123", 0, 0)));
    }

    #[test]
    fn unterminated() {
        let toks = [Token::new_delimiter("{", 0, 0)];
        let r = object_consumer(
            &mut toks.iter().enumerate().peekable(),
            &Rc::new(VarDict::new()),
            &Config::DEFAULT,
        )
        .unwrap_err();
        let e = TreebuilderErr::new_unterminated_obj(0);

        assert_eq!(r, e);
    }

    #[test]
    fn invalid_key() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_kwd("false", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_str("val", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let r = object_consumer(
            &mut toks.iter().enumerate().peekable(),
            &Rc::new(VarDict::new()),
            &Config::DEFAULT,
        )
        .unwrap_err();
        let e = TreebuilderErr::new_not_a_key(1);

        assert_eq!(r, e);
    }

    #[test]
    fn invalid_assignment() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key", 0, 0),
            Token::new_str(":", 0, 0),
            Token::new_str("val", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let r = object_consumer(
            &mut toks.iter().enumerate().peekable(),
            &Rc::new(VarDict::new()),
            &Config::DEFAULT,
        )
        .unwrap_err();
        let e = TreebuilderErr::new_not_an_assignment(2);

        assert_eq!(r, e);
    }

    #[test]
    fn trailing_sep_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = true;

        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_str("val", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        let mut exp_entries = HashMap::new();
        exp_entries.insert(
            "key".to_string(),
            StringNode::new(3, "val".to_owned()).into(),
        );

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &config),
            Ok(Some(
                ObjectNode::new(
                    0,
                    6,
                    exp_entries,
                    VarDict::new_with_parent(&Rc::new(VarDict::new()))
                )
                .into()
            ))
        );
        // It should consume the closing brace
        assert_eq!(inp.next(), None);
    }

    #[test]
    fn trailing_sep_not_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = false;

        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_str("val", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &config),
            Err(TreebuilderErr::new_trailing_sep(4))
        );
    }

    #[test]
    fn missing_sep() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key1", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_str("val1", 0, 0),
            Token::new_str("key2", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_str("val2", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];

        let mut e_entries: HashMap<String, Node> = HashMap::new();

        e_entries.insert(
            "key".to_string(),
            StringNode::new(3, "val".to_owned()).into(),
        );

        let r = object_consumer(
            &mut toks.iter().enumerate().peekable(),
            &Rc::new(VarDict::new()),
            &Config::DEFAULT,
        )
        .unwrap_err();
        let e = TreebuilderErr::new_not_a_sep(4);

        assert_eq!(r, e);
    }

    #[test]
    fn empty() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT,),
            Ok(Some(
                ObjectNode::new(
                    0,
                    2,
                    HashMap::new(),
                    VarDict::new_with_parent(&Rc::new(VarDict::new()))
                )
                .into(),
            ))
        );
    }

    #[test]
    fn single_entry() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_str("val", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];

        let mut exp_entries = HashMap::new();
        exp_entries.insert(
            "key".to_string(),
            StringNode::new(3, "val".to_owned()).into(),
        );

        assert_eq!(
            object_consumer(
                &mut toks.iter().enumerate().peekable(),
                &Rc::new(VarDict::new()),
                &Config::DEFAULT,
            ),
            Ok(Some(
                ObjectNode::new(
                    0,
                    5,
                    exp_entries,
                    VarDict::new_with_parent(&Rc::new(VarDict::new()))
                )
                .into()
            ))
        );
    }

    #[test]
    fn multiple_entries() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key_arr", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("]", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_kwd", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_kwd("false", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_num", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_num("123", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_obj", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_str", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_str("Hello, World!", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        let mut exp_entries = HashMap::new();
        exp_entries.insert(
            "key_arr".into(),
            ArrayNode::new(3, 5, Vec::new(), VarDict::new()).into(),
        );
        exp_entries.insert("key_kwd".to_string(), BoolNode::new(8, false).into());
        exp_entries.insert(
            "key_num".to_string(),
            NumberNode::new(12, "123".to_owned()).into(),
        );
        exp_entries.insert(
            "key_obj".to_string(),
            ObjectNode::new(
                16,
                18,
                HashMap::new(),
                VarDict::new_with_parent(&Rc::new(VarDict::new_with_parent(&Rc::new(
                    VarDict::new(),
                )))),
            )
            .into(),
        );
        exp_entries.insert(
            "key_str".to_string(),
            StringNode::new(21, "Hello, World!".to_owned()).into(),
        );

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(
                ObjectNode::new(
                    0,
                    23,
                    exp_entries,
                    VarDict::new_with_parent(&Rc::new(VarDict::new())),
                )
                .into(),
            ))
        );
    }

    #[test]
    fn declare_variable() {
        let inp = [
            Token::new_delimiter("{", 0, 0),
            Token::new_kwd("let", 0, 0),
            Token::new_kwd("foo", 0, 0),
            Token::new_equal_assignment_op(0),
            Token::new_str("foo", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("bar", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_str("bar", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let inp = &mut inp.iter().enumerate().peekable();

        let mut exp_var_dict = VarDict::new_with_parent(&Rc::new(VarDict::new()));
        exp_var_dict.insert("foo".into(), StringNode::new(4, "foo".to_owned()).into());

        let mut exp_entries = HashMap::new();
        exp_entries.insert("bar".into(), StringNode::new(8, "bar".to_owned()).into());

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(
                ObjectNode::new(0, 10, exp_entries, exp_var_dict,).into()
            ))
        )
    }

    #[test]
    fn declare_variable_with_trailing_sep() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = true;

        let inp = [
            Token::new_delimiter("{", 0, 0),
            Token::new_kwd("let", 0, 0),
            Token::new_kwd("num", 0, 0),
            Token::new_equal_assignment_op(0),
            Token::new_num("10", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let inp = &mut inp.iter().enumerate().peekable();

        let mut exp_var_dict = VarDict::new_with_parent(&Rc::new(VarDict::new()));
        exp_var_dict.insert("num".into(), NumberNode::new(4, "10".to_owned()).into());

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &config),
            Ok(Some(
                ObjectNode::new(0, 7, HashMap::new(), exp_var_dict).into()
            ))
        );
    }

    #[test]
    fn use_variable() {
        let inp = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("foo", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_kwd("bar", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let inp = &mut inp.iter().enumerate().peekable();

        let mut var_dict = VarDict::new();
        var_dict.insert("bar".into(), NumberNode::new(0, "5".to_owned()).into());
        let var_dict = Rc::new(var_dict);

        let mut exp_entries = HashMap::new();
        exp_entries.insert("foo".into(), NumberNode::new(0, "5".to_owned()).into());

        let exp_var_dict = VarDict::new_with_parent(&var_dict);

        assert_eq!(
            object_consumer(inp, &var_dict, &Config::DEFAULT),
            Ok(Some(
                ObjectNode::new(0, 5, exp_entries, exp_var_dict).into()
            ))
        );
    }
}
