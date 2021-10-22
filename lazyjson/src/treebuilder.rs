use std::iter::Peekable;

use crate::tokenizer::TokenIndices;

use self::{config::Config, error::TreebuilderErr, node::Node};

pub mod array_consumer;
pub mod config;
pub mod error;
pub mod keyword_consumer;
pub mod node;
pub mod number_consumer;
pub mod object_consumer;
pub mod string_consumer;
pub mod value_consumer;

type Consumer =
    dyn Fn(&mut Peekable<TokenIndices>, &Config) -> Result<Option<Node>, TreebuilderErr>;

pub const DEFAULT_CONFIG: Config = Config {
    allow_trailing_comma: false,
};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{tokenizer::Token, treebuilder::value_consumer::value_consumer};

    use super::*;

    #[test]
    fn array_of_cities() {
        let toks = [
            Token::sep("[", 0, 0),
            Token::sep("{", 0, 0),
            Token::str("name", 0, 0),
            Token::op(":", 0, 0),
            Token::str("Downtown", 0, 0),
            Token::sep(",", 0, 0),
            Token::str("code", 0, 0),
            Token::op(":", 0, 0),
            Token::num("123", 0, 0),
            Token::sep(",", 0, 0),
            Token::str("searchable", 0, 0),
            Token::op(":", 0, 0),
            Token::kwd("true", 0, 0),
            Token::sep("}", 0, 0),
            Token::sep(",", 0, 0),
            Token::sep("{", 0, 0),
            Token::str("name", 0, 0),
            Token::op(":", 0, 0),
            Token::str("Uptown", 0, 0),
            Token::sep(",", 0, 0),
            Token::str("code", 0, 0),
            Token::op(":", 0, 0),
            Token::num("456", 0, 0),
            Token::sep(",", 0, 0),
            Token::str("searchable", 0, 0),
            Token::op(":", 0, 0),
            Token::kwd("false", 0, 0),
            Token::sep("}", 0, 0),
            Token::sep(",", 0, 0),
            Token::kwd("null", 0, 0),
            Token::sep("]", 0, 0),
        ];

        let r = value_consumer(&mut toks.iter().enumerate().peekable(), &DEFAULT_CONFIG).unwrap();

        let mut downtown_entries = HashMap::new();
        downtown_entries.insert("name".to_string(), Node::new_str("Downtown", 4, 5));
        downtown_entries.insert("code".to_string(), Node::new_num("123", 8, 9));
        downtown_entries.insert("searchable".to_string(), Node::new_bool(true, 12, 13));

        let mut uptown_entries = HashMap::new();
        uptown_entries.insert("name".to_string(), Node::new_str("Uptown", 18, 19));
        uptown_entries.insert("code".to_string(), Node::new_num("456", 22, 23));
        uptown_entries.insert("searchable".to_string(), Node::new_bool(false, 26, 27));

        let e = Some(Node::new_arr(
            vec![
                Node::new_obj(downtown_entries, 1, 14),
                Node::new_obj(uptown_entries, 15, 28),
                Node::new_null(29, 30),
            ],
            0,
            31,
        ));

        assert_eq!(r, e);
    }
}
