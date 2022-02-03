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
pub use node::NodeSpecific;
pub use number_consumer::number_consumer;
pub use object_consumer::object_consumer;
pub use string_consumer::string_consumer;
pub use value_consumer::value_consumer;
pub use var_dict::VarDict;

#[cfg(test)]
mod testing;

mod variable_definition_consumer;
mod variable_usage_consumer;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        tokenizer::Token,
        treebuilder::{
            node::{ArraySpecific, ObjectSpecific},
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

        let mut downtown = ObjectSpecific::new(1, 14);
        downtown
            .entries
            .insert("name".to_string(), Node::new_str("Downtown", 4, 5));
        downtown
            .entries
            .insert("code".to_string(), Node::new_num("123", 8, 9));
        downtown
            .entries
            .insert("searchable".to_string(), Node::new_bool(true, 12, 13));

        downtown.var_dict =
            VarDict::new_with_parent(&Rc::new(VarDict::new_with_parent(&Rc::new(VarDict::new()))));

        let mut uptown = ObjectSpecific::new(15, 28);
        uptown
            .entries
            .insert("name".to_string(), Node::new_str("Uptown", 18, 19));
        uptown
            .entries
            .insert("code".to_string(), Node::new_num("456", 22, 23));
        uptown
            .entries
            .insert("searchable".to_string(), Node::new_bool(false, 26, 27));

        uptown.var_dict =
            VarDict::new_with_parent(&Rc::new(VarDict::new_with_parent(&Rc::new(VarDict::new()))));

        assert_eq!(
            value_consumer(
                &mut toks.iter().enumerate().peekable(),
                &Rc::new(VarDict::new()),
                &Config::DEFAULT,
            ),
            Ok(Some(
                ArraySpecific::new(
                    0,
                    31,
                    vec![downtown.into(), uptown.into(), Node::new_null(29, 30)],
                    VarDict::new_with_parent(&Rc::new(VarDict::new())),
                )
                .into(),
            ))
        );
    }
}
