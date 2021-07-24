use std::collections::HashMap;

use super::{
    consumer_response::ConsumerResponse,
    error::{TreebuilderError, UnexpectedToken, UnterminatedContainer},
    node::{BoolNode, Node, NodeType, NullNode, NumberNode, ObjectNode, StringNode, ValueNode},
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

        let left = match toks.get(cons) {
            None => {
                return Err(TreebuilderError::UnterminatedContainer(
                    UnterminatedContainer::new(NodeType::Object),
                ));
            }
            t => t.unwrap(),
        };

        if is_obj_close(left) {
            cons += 1;
            break;
        }

        if left.typ != TokenType::StringLiteral {
            return Err(TreebuilderError::UnexpectedToken(UnexpectedToken::new(
                left.clone(),
                vec![Token::str("<string>")],
            )));
        }

        cons += 1;

        if !is_assignment(&toks[cons]) {
            return Err(TreebuilderError::UnexpectedToken(UnexpectedToken::new(
                toks[cons].clone(),
                vec![Token::op(":")],
            )));
        }

        cons += 1;

        let right = toks.get(cons).unwrap();
        let node = match right.typ {
            TokenType::KeywordLiteral => match right.val.as_str() {
                "false" => ValueNode::Bool(BoolNode::new(false)),
                "true" => ValueNode::Bool(BoolNode::new(true)),
                "null" => ValueNode::Null(NullNode::new()),
                _ => {
                    return Err(TreebuilderError::UnexpectedToken(UnexpectedToken::new(
                        right.clone(),
                        vec![
                            Token::kwd("false"),
                            Token::kwd("null"),
                            Token::kwd("true"),
                            Token::num("<number>"),
                            Token::str("<string>"),
                        ],
                    )))
                }
            },
            TokenType::NumberLiteral => ValueNode::Number(NumberNode::new(right.val.as_str())),
            TokenType::StringLiteral => ValueNode::String(StringNode::new(right.val.as_str())),
            _ => {
                return Err(TreebuilderError::UnexpectedToken(UnexpectedToken::new(
                    right.clone(),
                    vec![
                        Token::kwd("false"),
                        Token::kwd("null"),
                        Token::kwd("true"),
                        Token::num("<number>"),
                        Token::str("<string>"),
                    ],
                )));
            }
        };

        entries.insert(left.val.clone(), Node::Value(node));
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

fn is_assignment(tok: &Token) -> bool {
    tok.typ == TokenType::Operator && tok.val == ":"
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
        let r_false = object_consumer(&gen_input(Token::kwd("false"))).unwrap();
        let e_false = gen_exp(ValueNode::Bool(BoolNode::new(false)));

        assert_eq!(r_false, e_false);

        let r_true = object_consumer(&gen_input(Token::kwd("true"))).unwrap();
        let e_true = gen_exp(ValueNode::Bool(BoolNode::new(true)));

        assert_eq!(r_true, e_true);

        let r_null = object_consumer(&gen_input(Token::kwd("null"))).unwrap();
        let e_null = gen_exp(ValueNode::Null(NullNode::new()));

        assert_eq!(r_null, e_null);

        let r_string = object_consumer(&gen_input(Token::str("test string"))).unwrap();
        let e_string = gen_exp(ValueNode::String(StringNode::new("test string")));

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
        fn gen_exp(val_node: ValueNode) -> ConsumerResponse {
            let mut entries = HashMap::new();

            entries.insert("keyword_key".into(), Node::Value(val_node));

            ConsumerResponse {
                cons: 5,
                node: Some(Node::Object(ObjectNode::new(entries))),
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

        entries.insert(
            "number_key".into(),
            Node::Value(ValueNode::Number(NumberNode::new("123.456"))),
        );

        let r = object_consumer(inp).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            node: Some(Node::Object(ObjectNode::new(entries))),
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

        entries.insert(
            "string_key".into(),
            Node::Value(ValueNode::String(StringNode::new("string_value"))),
        );

        let r = object_consumer(inp).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            node: Some(Node::Object(ObjectNode::new(entries))),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn invlid_key_token() {
        let inp = &[
            Token::sep("{"),
            Token::kwd("null"),
            Token::op(":"),
            Token::kwd("null"),
            Token::sep("}"),
        ];

        let r = object_consumer(inp).unwrap_err();
        let e = TreebuilderError::UnexpectedToken(UnexpectedToken::new(
            Token::kwd("null"),
            vec![Token::str("<string>")],
        ));

        assert_eq!(r, e);
    }
    #[test]
    pub fn invalid_assgnment_token() {
        let inp = &[
            Token::sep("{"),
            Token::str("key"),
            Token::kwd("null"),
            Token::kwd("null"),
            Token::sep("}"),
        ];

        let r = object_consumer(inp).unwrap_err();
        let e = TreebuilderError::UnexpectedToken(UnexpectedToken::new(
            Token::kwd("null"),
            vec![Token::op(":")],
        ));

        assert_eq!(r, e);
    }
    #[test]
    fn invalid_value_token() {
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

            let r = object_consumer(inp).unwrap_err();
            let e = TreebuilderError::UnexpectedToken(UnexpectedToken::new(
                val_tok,
                vec![
                    Token::kwd("false"),
                    Token::kwd("null"),
                    Token::kwd("true"),
                    Token::num("<number>"),
                    Token::str("<string>"),
                ],
            ));

            assert_eq!(r, e);
        }
    }
}
