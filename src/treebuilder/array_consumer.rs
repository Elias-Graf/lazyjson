use crate::{
    tokenizer::{Token, TokenType},
    treebuilder::{error::UnexpectedToken, node::StringNode},
};

use super::{
    consumer_response::ConsumerResponse,
    error::TreebuilderError,
    node::{BoolNode, Node, NodeType, NullNode, NumberNode, ValueNode},
    object_consumer::object_consumer,
};

pub fn array_consumer(toks: &[Token], offset: usize) -> Result<ConsumerResponse, TreebuilderError> {
    if !is_array_open(toks.get(offset).unwrap()) {
        return Ok(ConsumerResponse {
            cons: 0,
            node: None,
        });
    }

    let mut cons = 1;
    let mut entries: Vec<Node> = Vec::new();

    let maybe_close = toks.get(cons + offset).unwrap();

    if maybe_close.typ == TokenType::Separator && maybe_close.val == "]" {
        cons += 1;

        return Ok(ConsumerResponse {
            cons,
            node: Some(Node::new_arr(entries)),
        });
    }

    loop {
        let to_consume = match toks.get(cons + offset) {
            None => {
                return Err(TreebuilderError::new_unterminated_cont(NodeType::Array));
            }
            t => t.unwrap(),
        };

        let child = match to_consume.typ {
            TokenType::KeywordLiteral => match to_consume.val.as_str() {
                "false" => {
                    cons += 1;
                    Node::Value(ValueNode::Bool(BoolNode::new(false)))
                }
                "true" => {
                    cons += 1;
                    Node::Value(ValueNode::Bool(BoolNode::new(true)))
                }
                "null" => {
                    cons += 1;
                    Node::Value(ValueNode::Null(NullNode::new()))
                }
                _ => {
                    return Err(TreebuilderError::UnexpectedToken(UnexpectedToken::new(
                        to_consume.clone(),
                        vec![
                            Token::kwd("false"),
                            Token::kwd("null"),
                            Token::kwd("true"),
                            Token::num("<number>"),
                            Token::sep("{"),
                            Token::str("<string>"),
                        ],
                    )))
                }
            },
            TokenType::NumberLiteral => {
                cons += 1;
                Node::Value(ValueNode::Number(NumberNode::new(to_consume.val.as_str())))
            }
            TokenType::Separator => match to_consume.val.as_str() {
                "{" => {
                    let inner = object_consumer(toks, cons + offset)?;
                    cons += inner.cons;

                    inner.node.unwrap()
                }
                "[" => {
                    let inner = array_consumer(toks, cons + offset)?;
                    cons += inner.cons;

                    inner.node.unwrap()
                }
                _ => {
                    return Err(TreebuilderError::new_unexp_tok(
                        vec![
                            Token::kwd("false"),
                            Token::kwd("null"),
                            Token::kwd("true"),
                            Token::num("<number>"),
                            Token::sep("{"),
                            Token::sep("["),
                            Token::str("<string>"),
                        ],
                        to_consume.clone(),
                    ))
                }
            },
            TokenType::StringLiteral => {
                cons += 1;
                Node::Value(ValueNode::String(StringNode::new(to_consume.val.as_str())))
            }
            _ => {
                return Err(TreebuilderError::UnexpectedToken(UnexpectedToken::new(
                    to_consume.clone(),
                    vec![
                        Token::kwd("false"),
                        Token::kwd("null"),
                        Token::kwd("true"),
                        Token::num("<number>"),
                        Token::sep("{"),
                        Token::str("<string>"),
                    ],
                )));
            }
        };

        entries.push(child);

        let comma_or_close = toks.get(cons + offset).unwrap();

        if comma_or_close.typ == TokenType::Separator && comma_or_close.val == "," {
            cons += 1;
        } else if comma_or_close.typ == TokenType::Separator && comma_or_close.val == "]" {
            cons += 1;
            break;
        } else {
            return Err(TreebuilderError::new_unexp_tok(
                vec![Token::sep(","), Token::sep("]")],
                comma_or_close.clone(),
            ));
        }
    }

    Ok(ConsumerResponse {
        cons,
        node: Some(Node::new_arr(entries)),
    })
}

fn is_array_open(tok: &Token) -> bool {
    tok.typ == TokenType::Separator && tok.val == "["
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::tokenizer::Token;

    use super::*;

    #[test]
    pub fn non_array() {
        let inp = &[Token::kwd("null")];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 0,
            node: None,
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn empty_array() {
        let inp = &[Token::sep("["), Token::sep("]")];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 2,
            node: Some(Node::new_arr(Vec::new())),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_keyword() {
        let inp = &[Token::sep("["), Token::kwd("false"), Token::sep("]")];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(Node::new_arr(vec![Node::new_bool(false)])),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_number() {
        let inp = &[Token::sep("["), Token::num("1"), Token::sep("]")];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(Node::new_arr(vec![Node::new_num("1")])),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_string() {
        let inp = &[Token::sep("["), Token::str("test_string"), Token::sep("]")];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(Node::new_arr(vec![Node::new_str("test_string")])),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_object() {
        let inp = &[
            Token::sep("["),
            Token::sep("{"),
            Token::sep("}"),
            Token::sep("]"),
        ];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 4,
            node: Some(Node::new_arr(vec![Node::new_obj(HashMap::new())])),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn nested_array() {
        let inp = &[
            Token::sep("["),
            Token::sep("["),
            Token::sep("]"),
            Token::sep("]"),
        ];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 4,
            node: Some(Node::new_arr(vec![Node::new_arr(Vec::new())])),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn multiple_entries() {
        let inp = &[
            Token::sep("["),
            Token::kwd("null"),
            Token::sep(","),
            Token::kwd("false"),
            Token::sep(","),
            Token::kwd("true"),
            Token::sep(","),
            Token::num("123.456"),
            Token::sep(","),
            Token::str("test_string"),
            Token::sep(","),
            Token::sep("["),
            Token::sep("]"),
            Token::sep(","),
            Token::sep("{"),
            Token::sep("}"),
            Token::sep("]"),
        ];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: inp.len(),
            node: Some(Node::new_arr(vec![
                Node::new_null(),
                Node::new_bool(false),
                Node::new_bool(true),
                Node::new_num("123.456"),
                Node::new_str("test_string"),
                Node::new_arr(Vec::new()),
                Node::new_obj(HashMap::new()),
            ])),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn missing_comma() {
        let inp = &[
            Token::sep("["),
            Token::kwd("null"),
            Token::kwd("null"),
            Token::sep("]"),
        ];

        let r = array_consumer(inp, 0).unwrap_err();
        let e = TreebuilderError::new_unexp_tok(
            vec![Token::sep(","), Token::sep("]")],
            Token::kwd("null"),
        );

        assert_eq!(r, e);
    }
}
