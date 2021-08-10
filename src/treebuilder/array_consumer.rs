use crate::tokenizer::{Token, TokenType};

use super::{
    consumer_response::ConsumerResponse, error::TreebuilderError, node::Node,
    value_consumer::value_consumer,
};

/// Consumes an array composition. Non array composition (e.g. not array open)
/// are ignored.
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
        let child_consume = value_consumer(toks, cons + offset)?;
        let child = match child_consume.node {
            None => {
                let unexp = toks.get(cons + offset).unwrap().clone();
                return Err(TreebuilderError::new_exp_val_comp(unexp));
            }
            n => n.unwrap(),
        };

        cons += child_consume.cons;
        entries.push(child);

        let sep_or_close = toks.get(cons + offset).unwrap();

        if sep_or_close.typ == TokenType::Separator && sep_or_close.val == "," {
            cons += 1;
        } else if sep_or_close.typ == TokenType::Separator && sep_or_close.val == "]" {
            cons += 1;
            break;
        } else {
            return Err(TreebuilderError::new_exp_sep_or_close(sep_or_close.clone()));
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
        let inp = &[Token::kwd("null", 0, 0)];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 0,
            node: None,
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn empty_array() {
        let inp = &[Token::sep("[", 0, 0), Token::sep("]", 0, 0)];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 2,
            node: Some(Node::new_arr(Vec::new())),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_keyword() {
        let inp = &[
            Token::sep("[", 0, 0),
            Token::kwd("false", 0, 0),
            Token::sep("]", 0, 0),
        ];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(Node::new_arr(vec![Node::new_bool(false)])),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_number() {
        let inp = &[
            Token::sep("[", 0, 0),
            Token::num("1", 0, 0),
            Token::sep("]", 0, 0),
        ];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(Node::new_arr(vec![Node::new_num("1")])),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_string() {
        let inp = &[
            Token::sep("[", 0, 0),
            Token::str("test_string", 0, 0),
            Token::sep("]", 0, 0),
        ];

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
            Token::sep("[", 0, 0),
            Token::sep("{", 0, 0),
            Token::sep("}", 0, 0),
            Token::sep("]", 0, 0),
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
            Token::sep("[", 0, 0),
            Token::sep("[", 0, 0),
            Token::sep("]", 0, 0),
            Token::sep("]", 0, 0),
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
            Token::sep("[", 0, 0),
            Token::kwd("null", 0, 0),
            Token::sep(",", 0, 0),
            Token::kwd("false", 0, 0),
            Token::sep(",", 0, 0),
            Token::kwd("true", 0, 0),
            Token::sep(",", 0, 0),
            Token::num("123.456", 0, 0),
            Token::sep(",", 0, 0),
            Token::str("test_string", 0, 0),
            Token::sep(",", 0, 0),
            Token::sep("[", 0, 0),
            Token::sep("]", 0, 0),
            Token::sep(",", 0, 0),
            Token::sep("{", 0, 0),
            Token::sep("}", 0, 0),
            Token::sep("]", 0, 0),
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
            Token::sep("[", 0, 0),
            Token::kwd("null", 0, 0),
            Token::kwd("null", 0, 0),
            Token::sep("]", 0, 0),
        ];
        let r = array_consumer(inp, 0).unwrap_err();
        let e = TreebuilderError::new_exp_sep_or_close(Token::kwd("null", 0, 0));

        assert_eq!(r, e);
    }
    #[test]
    pub fn invalid_value() {
        let inp = &[
            Token::sep("[", 0, 0),
            Token::sep(",", 0, 0),
            Token::sep("]", 0, 0),
        ];
        let r = array_consumer(inp, 0).unwrap_err();
        let e = TreebuilderError::new_exp_val_comp(Token::sep(",", 0, 0));

        assert_eq!(r, e);
    }
}
