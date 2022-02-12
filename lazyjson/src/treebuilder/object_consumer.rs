use super::{
    config::Config,
    error::TreebuilderErr,
    node::{Node, ObjectNode},
    value_consumer::value_consumer,
    var_dict::VarDict,
    variable_definition_consumer::variable_definition_consumer,
};
use crate::{
    queue::Queue,
    tokenizer::{Token, TokenType},
};
use std::{collections::HashMap, rc::Rc};

pub fn object_consumer(
    inp: &mut Queue<Token>,
    var_dict: &Rc<VarDict>,
    config: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let opn_i = inp.idx();

    if !consume_obj_opn(inp) {
        return Ok(None);
    }

    let mut entries = HashMap::new();
    let mut var_dict = VarDict::new_with_parent(var_dict);

    // Check if the object is immediately closed again (empty).
    if consume_obj_cls(inp, opn_i)? {
        return Ok(Some(ObjectNode::new(opn_i, inp.idx(), entries).into()));
    }

    loop {
        if let Some((var_key, var_val)) =
            // TODO: figure out how to do this without cloning
            variable_definition_consumer(inp, &Rc::new(var_dict.clone()), config)?
        {
            var_dict.insert(var_key, var_val);
        } else {
            let key_i = inp.idx();
            let key = consume_key(inp)?;

            consume_assignment(inp, key_i)?;

            let val = match value_consumer(
                inp,
                // TODO: figure out how to do this without cloning
                &Rc::new(var_dict.clone()),
                &Config::DEFAULT,
            )? {
                None => return Err(TreebuilderErr::new_not_a_val(inp.idx())),
                Some(v) => v,
            };
            entries.insert(key, val);
        }

        if consume_obj_cls(inp, opn_i)? {
            return Ok(Some(ObjectNode::new(opn_i, inp.idx(), entries).into()));
        }

        consume_val_sep(inp)?;

        // Check if the next token is an object close, if yes, we have a trailing
        // separator.
        if consume_obj_cls(inp, opn_i)? {
            if !config.allow_trailing_commas {
                return Err(TreebuilderErr::new_trailing_sep(inp.idx() - 2));
            }

            return Ok(Some(ObjectNode::new(opn_i, inp.idx(), entries).into()));
        }
    }
}

/// Returns the token if a object open delimiter was found.
fn consume_obj_opn<'a>(inp: &'a mut Queue<Token>) -> bool {
    let t = inp.peek().unwrap();

    if t.typ == TokenType::Delimiter && t.val == "{" {
        inp.next();

        return true;
    }

    false
}

fn consume_obj_cls<'a>(inp: &'a mut Queue<Token>, opn_i: usize) -> Result<bool, TreebuilderErr> {
    let t = inp
        .peek()
        .ok_or(TreebuilderErr::new_unterminated_obj(opn_i))?;

    if t.typ == TokenType::Delimiter && t.val == "}" {
        inp.next();

        return Ok(true);
    }

    Ok(false)
}

fn consume_key<'a>(inp: &'a mut Queue<Token>) -> Result<String, TreebuilderErr> {
    let t = inp.peek().unwrap();

    if t.typ == TokenType::StringLiteral {
        return Ok(inp.next().unwrap().val.clone());
    }

    Err(TreebuilderErr::new_not_a_key(inp.idx()))
}

fn consume_assignment(inp: &mut Queue<Token>, key_i: usize) -> Result<(), TreebuilderErr> {
    let i = inp.idx();
    let t = inp
        .next()
        .ok_or(TreebuilderErr::new_unterminated_obj(key_i))?;

    if t.typ != TokenType::JsonAssignmentOperator {
        return Err(TreebuilderErr::new_not_an_assignment(i));
    }

    Ok(())
}

fn consume_val_sep(inp: &mut Queue<Token>) -> Result<(), TreebuilderErr> {
    let t = inp.peek().unwrap();

    if t.typ != TokenType::Separator || t.val != "," {
        return Err(TreebuilderErr::new_not_a_sep(inp.idx()));
    }

    inp.next();

    return Ok(());
}

#[cfg(test)]
mod tests {
    use crate::treebuilder::{
        node::{ArrayNode, BoolNode, NumberNode, StringNode},
        testing::{
            new_delimiter, new_equal_assignment_op, new_json_assignment_op, new_kwd, new_num,
            new_sep, new_str,
        },
    };

    use super::*;

    #[test]
    fn non_object() {
        let inp = &mut Queue::new(vec![new_num("123")]);

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(None)
        );
        assert_eq!(inp.next(), Some(&new_num("123")));
    }

    #[test]
    fn unterminated() {
        let inp = &mut Queue::new(vec![new_delimiter("{")]);

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_unterminated_obj(0))
        );
    }

    #[test]
    fn invalid_key() {
        let inp = &mut Queue::new(vec![
            new_delimiter("{"),
            new_kwd("false"),
            new_json_assignment_op(),
            new_str("val"),
            new_delimiter("}"),
        ]);

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT,),
            Err(TreebuilderErr::new_not_a_key(1))
        );
    }

    #[test]
    fn invalid_assignment() {
        let inp = &mut Queue::new(vec![
            new_delimiter("{"),
            new_str("key"),
            new_str(":"),
            new_str("val"),
            new_delimiter("}"),
        ]);

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT,),
            Err(TreebuilderErr::new_not_an_assignment(2))
        );
    }

    #[test]
    fn trailing_sep_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = true;

        let inp = &mut Queue::new(vec![
            new_delimiter("{"),
            new_str("key"),
            new_json_assignment_op(),
            new_str("val"),
            new_sep(","),
            new_delimiter("}"),
        ]);

        let mut exp_entries = HashMap::new();
        exp_entries.insert(
            "key".to_string(),
            StringNode::new(3, "val".to_owned()).into(),
        );

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &config),
            Ok(Some(ObjectNode::new(0, 6, exp_entries).into()))
        );
        // It should consume the closing brace
        assert_eq!(inp.next(), None);
    }

    #[test]
    fn trailing_sep_not_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = false;

        let inp = &mut Queue::new(vec![
            new_delimiter("{"),
            new_str("key"),
            new_json_assignment_op(),
            new_str("val"),
            new_sep(","),
            new_delimiter("}"),
        ]);

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &config),
            Err(TreebuilderErr::new_trailing_sep(4))
        );
    }

    #[test]
    fn missing_sep() {
        let inp = &mut Queue::new(vec![
            new_delimiter("{"),
            new_str("key1"),
            new_json_assignment_op(),
            new_str("val1"),
            new_str("key2"),
            new_json_assignment_op(),
            new_str("val2"),
            new_delimiter("}"),
        ]);

        let mut e_entries: HashMap<String, Node> = HashMap::new();

        e_entries.insert(
            "key".to_string(),
            StringNode::new(3, "val".to_owned()).into(),
        );

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_not_a_sep(4))
        );
    }

    #[test]
    fn empty() {
        let inp = &mut Queue::new(vec![new_delimiter("{"), new_delimiter("}")]);

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(ObjectNode::new(0, 2, HashMap::new()).into()))
        );
    }

    #[test]
    fn single_entry() {
        let inp = &mut Queue::new(vec![
            new_delimiter("{"),
            new_str("key"),
            new_json_assignment_op(),
            new_str("val"),
            new_delimiter("}"),
        ]);

        let mut exp_entries = HashMap::new();
        exp_entries.insert(
            "key".to_string(),
            StringNode::new(3, "val".to_owned()).into(),
        );

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT,),
            Ok(Some(ObjectNode::new(0, 5, exp_entries).into()))
        );
    }

    #[test]
    fn multiple_entries() {
        let inp = &mut Queue::new(vec![
            new_delimiter("{"),
            new_str("key_arr"),
            new_json_assignment_op(),
            new_delimiter("["),
            new_delimiter("]"),
            new_sep(","),
            new_str("key_kwd"),
            new_json_assignment_op(),
            new_kwd("false"),
            new_sep(","),
            new_str("key_num"),
            new_json_assignment_op(),
            new_num("123"),
            new_sep(","),
            new_str("key_obj"),
            new_json_assignment_op(),
            new_delimiter("{"),
            new_delimiter("}"),
            new_sep(","),
            new_str("key_str"),
            new_json_assignment_op(),
            new_str("Hello, World!"),
            new_delimiter("}"),
        ]);

        let mut exp_entries = HashMap::new();
        exp_entries.insert("key_arr".into(), ArrayNode::new(3, 5, Vec::new()).into());
        exp_entries.insert("key_kwd".to_string(), BoolNode::new(8, false).into());
        exp_entries.insert(
            "key_num".to_string(),
            NumberNode::new(12, "123".to_owned()).into(),
        );
        exp_entries.insert(
            "key_obj".to_string(),
            ObjectNode::new(16, 18, HashMap::new()).into(),
        );
        exp_entries.insert(
            "key_str".to_string(),
            StringNode::new(21, "Hello, World!".to_owned()).into(),
        );

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(ObjectNode::new(0, 23, exp_entries,).into(),))
        );
    }

    #[test]
    fn declare_variable() {
        let inp = &mut Queue::new(vec![
            new_delimiter("{"),
            new_kwd("let"),
            new_kwd("foo"),
            new_equal_assignment_op(),
            new_str("foo"),
            new_sep(","),
            new_str("bar"),
            new_json_assignment_op(),
            new_str("bar"),
            new_delimiter("}"),
        ]);

        let mut exp_var_dict = VarDict::new_with_parent(&Rc::new(VarDict::new()));
        exp_var_dict.insert("foo".into(), StringNode::new(4, "foo".to_owned()).into());

        let mut exp_entries = HashMap::new();
        exp_entries.insert("bar".into(), StringNode::new(8, "bar".to_owned()).into());

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(ObjectNode::new(0, 10, exp_entries,).into()))
        )
    }

    #[test]
    pub fn declare_and_use_variable() {
        let inp = &mut Queue::new(vec![
            new_delimiter("{"),
            new_kwd("let"),
            new_kwd("var"),
            new_equal_assignment_op(),
            new_num("10"),
            new_sep(","),
            new_str("num"),
            new_json_assignment_op(),
            new_kwd("var"),
            new_delimiter("}"),
        ]);

        let mut exp_entries = HashMap::new();
        exp_entries.insert("num".to_owned(), NumberNode::new(4, "10".to_owned()).into());

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(ObjectNode::new(0, 10, exp_entries).into()))
        );
    }

    #[test]
    pub fn declare_and_use_variable_with_trailing_sep() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = true;

        let inp = &mut Queue::new(vec![
            new_delimiter("{"),
            new_kwd("let"),
            new_kwd("var"),
            new_equal_assignment_op(),
            new_num("10"),
            new_sep(","),
            new_str("num"),
            new_json_assignment_op(),
            new_kwd("var"),
            new_sep(","),
            new_delimiter("}"),
        ]);

        let mut exp_entries = HashMap::new();
        exp_entries.insert("num".to_owned(), NumberNode::new(4, "10".to_owned()).into());

        assert_eq!(
            object_consumer(inp, &Rc::new(VarDict::new()), &config),
            Ok(Some(ObjectNode::new(0, 11, exp_entries).into()))
        );
    }
}
