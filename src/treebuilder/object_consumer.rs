use std::collections::HashMap;

use super::{
    consumer_response::ConsumerResponse,
    error::{TreebuilderError, UnterminatedContainer},
    node::{BoolNode, Node, NodeType, NullNode, ObjectNode, ValueNode},
};
use crate::tokenizer::{Token, TokenType};

pub fn object_consumer(toks: &[Token]) -> Result<ConsumerResponse, TreebuilderError> {
    if !is_obj_open(toks.get(0).unwrap()) {
        return Ok(ConsumerResponse {
            cons: 0,
            node: None,
        });
    }

    let mut cons = 0;
    let mut entries = HashMap::new();

    loop {
        cons += 1;

        let first = match toks.get(cons) {
            None => {
                return Err(TreebuilderError::UnterminatedContainer(
                    UnterminatedContainer::new(NodeType::Object),
                ));
            }
            t => t.unwrap(),
        };

        if is_obj_close(first) {
            cons += 1;
            break;
        }

        cons += 2;

        let third = toks.get(cons).unwrap();
        let node = match third.val.as_str() {
            "false" => ValueNode::Bool(BoolNode::new(false)),
            "true" => ValueNode::Bool(BoolNode::new(true)),
            "null" => ValueNode::Null(NullNode::new()),
            _ => panic!("not a known value \"{}\"", third.val),
        };

        entries.insert(first.val.clone(), Node::Value(node));
    }

    Ok(ConsumerResponse {
        cons: cons,
        node: Some(Node::Object(ObjectNode::new(entries))),
    })
}

fn is_obj_open(tok: &Token) -> bool {
    tok.typ == TokenType::Separator && tok.val == "{"
}

fn is_obj_close(tok: &Token) -> bool {
    tok.typ == TokenType::Separator && tok.val == "}"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn non_object() {
        let r = object_consumer(&[Token::kwd("null")]).unwrap();
        let e = ConsumerResponse {
            cons: 0,
            node: None,
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn empty_object() {
        let r = object_consumer(&[Token::sep("{"), Token::sep("}")]).unwrap();
        let e = ConsumerResponse {
            cons: 2,
            node: Some(Node::Object(ObjectNode::new(HashMap::new()))),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn unterminated_object() {
        let r = object_consumer(&[Token::sep("{")]).unwrap_err();
        let e =
            TreebuilderError::UnterminatedContainer(UnterminatedContainer::new(NodeType::Object));

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_keyword_entry() {
        let r_false = object_consumer(&inp("false")).unwrap();
        let e_false = exp(ValueNode::Bool(BoolNode::new(false)));

        assert_eq!(r_false, e_false);

        let r_true = object_consumer(&inp("true")).unwrap();
        let e_true = exp(ValueNode::Bool(BoolNode::new(true)));

        assert_eq!(r_true, e_true);

        let r_null = object_consumer(&inp("null")).unwrap();
        let e_null = exp(ValueNode::Null(NullNode::new()));

        assert_eq!(r_null, e_null);

        fn inp(keyword: &str) -> Vec<Token> {
            vec![
                Token::sep("{"),
                Token::str("keyword_key"),
                Token::op(":"),
                Token::kwd(keyword),
                Token::sep("}"),
            ]
        }
        fn exp(val_node: ValueNode) -> ConsumerResponse {
            let mut entries = HashMap::new();

            entries.insert("keyword_key".into(), Node::Value(val_node));

            ConsumerResponse {
                cons: 5,
                node: Some(Node::Object(ObjectNode::new(entries))),
            }
        }
    }
}
