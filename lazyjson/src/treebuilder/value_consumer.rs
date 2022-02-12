use std::rc::Rc;

use crate::{queue::Queue, tokenizer::Token};

use super::{
    array_consumer, error::TreebuilderErr, keyword_consumer, node::Node, number_consumer,
    object_consumer, string_consumer, var_dict::VarDict,
    variable_usage_consumer::variable_usage_consumer, Config,
};

type Consumer =
    dyn Fn(&mut Queue<Token>, &Rc<VarDict>, &Config) -> Result<Option<Node>, TreebuilderErr>;

/// Consumes all possible forms of "value constellations". For example simple
/// numbers (`1`), or arrays (`[1, 2]`), and so on. This consumer combines other
/// "sub-consumers" to achieve this behavior.
pub fn value_consumer(
    toks: &mut Queue<Token>,
    var_dict: &Rc<VarDict>,
    config: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let consumers: &[&Consumer] = &[
        &array_consumer,
        &keyword_consumer,
        &variable_usage_consumer,
        &number_consumer,
        &object_consumer,
        &string_consumer,
    ];

    for consumer in consumers {
        let res = consumer(toks, var_dict, config)?;

        if res.is_some() {
            return Ok(res);
        }
    }

    Err(TreebuilderErr::new_not_a_val(toks.idx()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::treebuilder::{
        node::{ArrayNode, BoolNode, NullNode, NumberNode, ObjectNode, StringNode},
        testing::{new_delimiter, new_kwd, new_num, new_str},
        value_consumer,
        var_dict::VarDict,
    };

    use super::*;

    #[test]
    fn not_a_value() {
        let inp = &mut Queue::new(vec![new_delimiter("}")]);

        assert_eq!(
            value_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Err(TreebuilderErr::new_not_a_val(0))
        );
    }

    #[test]
    fn array() {
        let inp = &mut Queue::new(vec![new_delimiter("["), new_delimiter("]")]);

        assert_eq!(
            value_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(ArrayNode::new(0, 2, Vec::new()).into())),
        );
    }

    #[test]
    fn keyword() {
        let inp = &mut Queue::new(vec![new_kwd("false")]);

        assert_eq!(
            value_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(BoolNode::new(0, false).into()))
        );
    }

    #[test]
    fn number() {
        let inp = &mut Queue::new(vec![new_num("123.456")]);

        assert_eq!(
            value_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(NumberNode::new(0, "123.456".to_owned()).into()))
        );
    }

    #[test]
    fn object() {
        let inp = &mut Queue::new(vec![new_delimiter("{"), new_delimiter("}")]);

        assert_eq!(
            value_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(ObjectNode::new(0, 2, HashMap::new()).into()))
        );
    }

    #[test]
    fn string() {
        let inp = &mut Queue::new(vec![new_str("hello world")]);

        assert_eq!(
            value_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(StringNode::new(0, "hello world".to_owned()).into()))
        );
    }

    #[test]
    fn use_variable() {
        let inp = &mut Queue::new(vec![new_kwd("variable")]);

        let mut var_dict = VarDict::new();
        var_dict.insert("variable".into(), NullNode::new(0).into());

        assert_eq!(
            value_consumer(inp, &Rc::new(var_dict), &Config::DEFAULT),
            Ok(Some(NullNode::new(0).into())),
        )
    }
}
