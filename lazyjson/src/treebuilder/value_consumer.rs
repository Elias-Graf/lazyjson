use std::{iter::Peekable, rc::Rc};

use crate::tokenizer::TokenIndices;

use super::{
    array_consumer, error::TreebuilderErr, keyword_consumer, node::Node, number_consumer,
    object_consumer, string_consumer, var_dict::VarDict,
    variable_usage_consumer::variable_usage_consumer, Config,
};

type Consumer = dyn Fn(
    &mut Peekable<TokenIndices>,
    &Rc<VarDict>,
    &Config,
) -> Result<Option<Node>, TreebuilderErr>;

/// Consumes all possible forms of "value constellations". For example simple
/// numbers (`1`), or arrays (`[1, 2]`), and so on. This consumer combines other
/// "sub-consumers" to achieve this behavior.
pub fn value_consumer(
    toks: &mut Peekable<TokenIndices>,
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

    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        tokenizer::Token,
        treebuilder::{
            node::{ArrayNode, BoolNode, NullNode, NumberNode, ObjectNode, StringNode},
            testing, value_consumer,
            var_dict::VarDict,
        },
    };

    use super::*;

    #[test]
    fn array() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        assert_eq!(
            value_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(
                ArrayNode::new(0, 2, Vec::new()).into()
            )),
        );
    }

    #[test]
    fn keyword() {
        let toks = [Token::new_kwd("false", 0, 0)];

        assert_eq!(
            value_consumer(
                &mut toks.iter().enumerate().peekable(),
                &Rc::new(VarDict::new()),
                &Config::DEFAULT,
            ),
            Ok(Some(BoolNode::new(0, false).into()))
        );
    }

    #[test]
    fn number() {
        let toks = [Token::new_num("123.456", 0, 0)];
        let inp = &mut testing::inp_from(&toks);

        assert_eq!(
            value_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT,),
            Ok(Some(NumberNode::new(0, "123.456".to_owned()).into()))
        );
    }

    #[test]
    fn object() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let inp = &mut testing::inp_from(&toks);

        assert_eq!(
            value_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT,),
            Ok(Some(
                ObjectNode::new(
                    0,
                    2,
                    HashMap::new(),
                    VarDict::new_with_parent(&Rc::new(VarDict::new()))
                )
                .into()
            ))
        );
    }

    #[test]
    fn string() {
        let toks = [Token::new_str("hello world", 0, 0)];
        let inp = &mut testing::inp_from(&toks);

        assert_eq!(
            value_consumer(inp, &Rc::new(VarDict::new()), &Config::DEFAULT),
            Ok(Some(StringNode::new(0, "hello world".to_owned()).into()))
        );
    }

    #[test]
    fn use_variable() {
        let inp = [Token::new_kwd("variable", 0, 0)];
        let inp = &mut inp.iter().enumerate().peekable();

        let mut var_dict = VarDict::new();
        var_dict.insert("variable".into(), NullNode::new(0).into());

        assert_eq!(
            value_consumer(inp, &Rc::new(var_dict), &Config::DEFAULT),
            Ok(Some(NullNode::new(0).into())),
        )
    }
}
