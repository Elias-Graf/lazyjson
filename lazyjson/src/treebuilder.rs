use std::rc::Rc;

use crate::{queue::Queue, tokenizer::Token};

pub mod array_consumer;
pub mod config;
pub mod error;
pub mod keyword_consumer;
pub mod node;
pub mod number_consumer;
pub mod object_consumer;
pub mod string_consumer;
pub mod value_consumer;
pub mod var_dict;

pub use array_consumer::array_consumer;
pub use config::Config;
pub use error::TreebuilderErr;
pub use keyword_consumer::keyword_consumer;
pub use node::Node;
pub use number_consumer::number_consumer;
pub use object_consumer::object_consumer;
pub use string_consumer::string_consumer;
pub use value_consumer::value_consumer;
pub use var_dict::VarDict;

#[cfg(test)]
mod testing;

mod variable_definition_consumer;
mod variable_usage_consumer;

pub fn build(inp: &[Token], config: &Config) -> Result<Option<Node>, TreebuilderErr> {
    value_consumer(
        &mut Queue::new(Vec::from(inp)),
        &Rc::new(VarDict::new()),
        config,
    )
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, rc::Rc};

    use crate::{
        tokenizer::Token,
        treebuilder::{
            node::{ArrayNode, BoolNode, NullNode, NumberNode, ObjectNode, StringNode},
            value_consumer::value_consumer,
            var_dict::VarDict,
        },
    };

    use super::*;

    #[test]
    fn array_of_cities() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("{", 0, 0),
            Token::new_str("name", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_str("Downtown", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("code", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_num("123", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("searchable", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_kwd("true", 0, 0),
            Token::new_delimiter("}", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("{", 0, 0),
            Token::new_str("name", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_str("Uptown", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("code", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_num("456", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("searchable", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_kwd("false", 0, 0),
            Token::new_delimiter("}", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_kwd("null", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];

        let mut downtown_entries = HashMap::new();
        downtown_entries.insert(
            "name".to_string(),
            StringNode::new(4, "Downtown".to_owned()).into(),
        );
        downtown_entries.insert(
            "code".to_string(),
            NumberNode::new(8, "123".to_owned()).into(),
        );
        downtown_entries.insert("searchable".to_string(), BoolNode::new(12, true).into());

        let downtown = ObjectNode::new(1, 14, downtown_entries);

        let mut uptown_entries = HashMap::new();
        uptown_entries.insert(
            "name".to_string(),
            StringNode::new(18, "Uptown".to_owned()).into(),
        );
        uptown_entries.insert(
            "code".to_string(),
            NumberNode::new(22, "456".to_owned()).into(),
        );
        uptown_entries.insert("searchable".to_string(), BoolNode::new(26, false).into());

        let uptown = ObjectNode::new(15, 28, uptown_entries);

        assert_eq!(
            value_consumer(
                &mut Queue::new(Vec::from(toks)),
                &Rc::new(VarDict::new()),
                &Config::DEFAULT,
            ),
            Ok(Some(
                ArrayNode::new(
                    0,
                    31,
                    vec![downtown.into(), uptown.into(), NullNode::new(29).into()],
                )
                .into(),
            ))
        );
    }
}
