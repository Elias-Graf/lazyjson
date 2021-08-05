use std::collections::HashMap;

use super::{
    consumer_response::ConsumerResponse,
    error::{TreebuilderError, UnexpectedToken},
    node::{ContainerNode, Node, NodeType, ObjectNode},
    value_consumer::value_consumer,
};
use crate::tokenizer::{Token, TokenType};

/// Consumes an object composition. Non object compositions (e.g. not object
/// open) are ignored.
pub fn object_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, TreebuilderError> {
    if !is_obj_open(toks.get(offset).unwrap()) {
        return Ok(ConsumerResponse {
            cons: 0,
            node: None,
        });
    }

    let mut cons = 1;
    let mut entries = HashMap::new();

    loop {
        let mut left = match toks.get(cons + offset) {
            None => {
                return Err(TreebuilderError::new_unterminated_cont(NodeType::Object));
            }
            t => t.unwrap(),
        };

        if is_obj_close(left) {
            cons += 1;
            break;
        }

        if cons > 1 {
            if !is_separator(left) {
                return Err(TreebuilderError::new_exp_sep_or_close(left.clone()));
            } else {
                cons += 1;

                left = match toks.get(cons + offset) {
                    None => {
                        return Err(TreebuilderError::new_unterminated_cont(NodeType::Object));
                    }
                    t => t.unwrap(),
                };
            }
        }

        if left.typ != TokenType::StringLiteral {
            return Err(TreebuilderError::UnexpectedToken(UnexpectedToken::new(
                left.clone(),
                vec![Token::str("<string>")],
            )));
        }

        cons += 1;

        if !is_assignment(&toks[cons + offset]) {
            return Err(TreebuilderError::UnexpectedToken(UnexpectedToken::new(
                toks[cons + offset].clone(),
                vec![Token::op(":")],
            )));
        }

        cons += 1;

        let value_consume = value_consumer(toks, cons + offset)?;
        let value = match value_consume.node {
            None => {
                let unexp = toks.get(cons + offset).unwrap();
                return Err(TreebuilderError::new_exp_val_comp(unexp.clone()));
            }
            n => n.unwrap(),
        };

        cons += value_consume.cons;

        entries.insert(left.val.clone(), value);
    }

    Ok(ConsumerResponse {
        cons: cons,
        node: Some(Node::Container(ContainerNode::Object(ObjectNode::new(
            entries,
        )))),
    })
}

fn is_separator(left: &Token) -> bool {
    left.typ == TokenType::Separator && left.val == ","
}

fn is_obj_open(tok: &Token) -> bool {
    tok.typ == TokenType::Separator && tok.val == "{"
}

fn is_obj_close(tok: &Token) -> bool {
    tok.typ == TokenType::Separator && tok.val == "}"
}

fn is_assignment(tok: &Token) -> bool {
    tok.typ == TokenType::Operator && tok.val == ":"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn non_object() {
        let r = object_consumer(&[Token::kwd("null")], 0).unwrap();
        let e = ConsumerResponse {
            cons: 0,
            node: None,
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn empty_object() {
        let r = object_consumer(&[Token::sep("{"), Token::sep("}")], 0).unwrap();
        let e = ConsumerResponse {
            cons: 2,
            node: Some(Node::Container(ContainerNode::Object(ObjectNode::new(
                HashMap::new(),
            )))),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn unterminated_object() {
        let r = object_consumer(&[Token::sep("{")], 0).unwrap_err();
        let e = TreebuilderError::new_unterminated_cont(NodeType::Object);

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_keyword_entry() {
        let r_false = object_consumer(&gen_input(Token::kwd("false")), 0).unwrap();
        let e_false = gen_exp(Node::new_bool(false));

        assert_eq!(r_false, e_false);

        let r_true = object_consumer(&gen_input(Token::kwd("true")), 0).unwrap();
        let e_true = gen_exp(Node::new_bool(true));

        assert_eq!(r_true, e_true);

        let r_null = object_consumer(&gen_input(Token::kwd("null")), 0).unwrap();
        let e_null = gen_exp(Node::new_null());

        assert_eq!(r_null, e_null);

        let r_string = object_consumer(&gen_input(Token::str("test string")), 0).unwrap();
        let e_string = gen_exp(Node::new_str("test string"));

        assert_eq!(r_string, e_string);

        fn gen_input(val_tok: Token) -> Vec<Token> {
            vec![
                Token::sep("{"),
                Token::str("keyword_key"),
                Token::op(":"),
                val_tok,
                Token::sep("}"),
            ]
        }
        fn gen_exp(val_node: Node) -> ConsumerResponse {
            let mut entries = HashMap::new();

            entries.insert("keyword_key".into(), val_node);

            ConsumerResponse {
                cons: 5,
                node: Some(Node::new_obj(entries)),
            }
        }
    }
    #[test]
    pub fn single_number_entry() {
        let inp = &[
            Token::sep("{"),
            Token::str("number_key"),
            Token::op(":"),
            Token::num("123.456"),
            Token::sep("}"),
        ];

        let mut entries = HashMap::new();

        entries.insert("number_key".into(), Node::new_num("123.456"));

        let r = object_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            node: Some(Node::Container(ContainerNode::Object(ObjectNode::new(
                entries,
            )))),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_string_entry() {
        let inp = &[
            Token::sep("{"),
            Token::str("string_key"),
            Token::op(":"),
            Token::str("string_value"),
            Token::sep("}"),
        ];

        let mut entries = HashMap::new();

        entries.insert("string_key".into(), Node::new_str("string_value"));

        let r = object_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            node: Some(Node::Container(ContainerNode::Object(ObjectNode::new(
                entries,
            )))),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_array_entry() {
        let inp = &[
            Token::sep("{"),
            Token::str("array_key"),
            Token::op(":"),
            Token::sep("["),
            Token::sep("]"),
            Token::sep("}"),
        ];

        let mut entries = HashMap::new();

        entries.insert(String::from("array_key"), Node::new_arr(Vec::new()));

        let r = object_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 6,
            node: Some(Node::new_obj(entries)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn nested_object() {
        let inp = &[
            Token::sep("{"),
            Token::str("object_key"),
            Token::op(":"),
            Token::sep("{"),
            Token::str("inner_key"),
            Token::op(":"),
            Token::str("inner_val"),
            Token::sep("}"),
            Token::sep("}"),
        ];

        let mut inner_entries = HashMap::new();

        inner_entries.insert(String::from("inner_key"), Node::new_str("inner_val"));

        let mut entries = HashMap::new();

        entries.insert(
            String::from("object_key"),
            Node::Container(ContainerNode::Object(ObjectNode::new(inner_entries))),
        );

        let r = object_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 9,
            node: Some(Node::Container(ContainerNode::Object(ObjectNode::new(
                entries,
            )))),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn multiple_entries() {
        let inp = &[
            Token::sep("{"),
            Token::str("keyword_key"),
            Token::op(":"),
            Token::kwd("null"),
            Token::sep(","),
            Token::str("number_key"),
            Token::op(":"),
            Token::num("123.456"),
            Token::sep(","),
            Token::str("string_key"),
            Token::op(":"),
            Token::str("string value"),
            Token::sep(","),
            Token::str("object_key"),
            Token::op(":"),
            Token::sep("{"),
            Token::sep("}"),
            Token::sep("}"),
        ];

        let r = object_consumer(inp, 0).unwrap();

        let mut entries = HashMap::new();

        entries.insert(String::from("keyword_key"), Node::new_null());
        entries.insert(String::from("number_key"), Node::new_num("123.456"));
        entries.insert(String::from("string_key"), Node::new_str("string value"));
        entries.insert(String::from("object_key"), Node::new_obj(HashMap::new()));

        let e = ConsumerResponse {
            cons: inp.len(),
            node: Some(Node::Container(ContainerNode::Object(ObjectNode::new(
                entries,
            )))),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn invalid_key_token() {
        let inp = &[
            Token::sep("{"),
            Token::kwd("null"),
            Token::op(":"),
            Token::kwd("null"),
            Token::sep("}"),
        ];

        let r = object_consumer(inp, 0).unwrap_err();
        let e = TreebuilderError::UnexpectedToken(UnexpectedToken::new(
            Token::kwd("null"),
            vec![Token::str("<string>")],
        ));

        assert_eq!(r, e);
    }
    #[test]
    pub fn invalid_assignment_token() {
        let inp = &[
            Token::sep("{"),
            Token::str("key"),
            Token::kwd("null"),
            Token::kwd("null"),
            Token::sep("}"),
        ];

        let r = object_consumer(inp, 0).unwrap_err();
        let e = TreebuilderError::UnexpectedToken(UnexpectedToken::new(
            Token::kwd("null"),
            vec![Token::op(":")],
        ));

        assert_eq!(r, e);
    }
    #[test]
    pub fn invalid_value_token() {
        test(Token::op(":"));
        test(Token::sep(","));

        fn test(val_tok: Token) {
            let inp = &[
                Token::sep("{"),
                Token::str("key"),
                Token::op(":"),
                val_tok.clone(),
                Token::sep("}"),
            ];

            let r = object_consumer(inp, 0).unwrap_err();
            let e = TreebuilderError::new_exp_val_comp(val_tok);

            assert_eq!(r, e);
        }
    }
}
