use crate::tokenizer::{Token, TokenType};

use super::{
    consumer_response::ConsumerResponse, error::TreebuilderError, node::Node,
    value_consumer::value_consumer,
};

/// Consumes an array composition. Non array composition (e.g. not array open)
/// are ignored.
pub fn array_consumer(toks: &[Token], offset: usize) -> Result<ConsumerResponse, TreebuilderError> {
    let first = toks.get(offset).unwrap();

    if !is_array_open(first) {
        return Ok(ConsumerResponse {
            cons: 0,
            node: None,
        });
    }

    let mut consumed_toks = Vec::new();

    consumed_toks.push(first.clone());

    let mut entries: Vec<Node> = Vec::new();

    let maybe_close = toks.get(consumed_toks.len() + offset).unwrap();

    if maybe_close.typ == TokenType::Separator && maybe_close.val == "]" {
        consumed_toks.push(maybe_close.clone());

        return Ok(ConsumerResponse {
            cons: consumed_toks.len(),
            node: Some(Node::new_arr(entries, consumed_toks)),
        });
    }

    loop {
        let child_consume = value_consumer(toks, consumed_toks.len() + offset)?;
        let child = match child_consume.node {
            None => {
                let unexp = toks.get(consumed_toks.len() + offset).unwrap().clone();
                return Err(TreebuilderError::new_exp_val_comp(unexp));
            }
            n => n.unwrap(),
        };

        let mut other = child.toks().clone();

        consumed_toks.append(&mut other);
        entries.push(child);

        let sep_or_close = toks.get(consumed_toks.len() + offset).unwrap();

        if sep_or_close.typ == TokenType::Separator && sep_or_close.val == "," {
            consumed_toks.push(sep_or_close.clone());
        } else if sep_or_close.typ == TokenType::Separator && sep_or_close.val == "]" {
            consumed_toks.push(sep_or_close.clone());
            break;
        } else {
            return Err(TreebuilderError::new_exp_sep_or_close(sep_or_close.clone()));
        }
    }

    Ok(ConsumerResponse {
        cons: consumed_toks.len(),
        node: Some(Node::new_arr(entries, consumed_toks)),
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
            node: Some(Node::new_arr(Vec::new(), inp.to_vec())),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_keyword() {
        let kwd = Token::kwd("false", 0, 0);
        let inp = &[Token::sep("[", 0, 0), kwd.clone(), Token::sep("]", 0, 0)];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(Node::new_arr(
                vec![Node::new_bool(false, kwd)],
                inp.to_vec(),
            )),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_number() {
        let num = Token::num("1", 0, 0);
        let inp = &[Token::sep("[", 0, 0), num.clone(), Token::sep("]", 0, 0)];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(Node::new_arr(vec![Node::new_num("1", num)], inp.to_vec())),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_string() {
        let str = Token::str("test_string", 0, 0);
        let inp = &[Token::sep("[", 0, 0), str.clone(), Token::sep("]", 0, 0)];

        let r = array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(Node::new_arr(
                vec![Node::new_str("test_string", str)],
                inp.to_vec(),
            )),
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
            node: Some(Node::new_arr(
                vec![Node::new_obj(HashMap::new(), inp[1..3].to_vec())],
                inp.to_vec(),
            )),
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
            node: Some(Node::new_arr(
                vec![Node::new_arr(Vec::new(), inp[1..3].to_vec())],
                inp.to_vec(),
            )),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn multiple_entries() {
        let null_tok = Token::kwd("null", 0, 0);
        let false_tok = Token::kwd("false", 0, 0);
        let true_tok = Token::kwd("true", 0, 0);
        let num_tok = Token::num("123.456", 0, 0);
        let str_tok = Token::str("test_string", 0, 0);
        let arr_toks = vec![Token::sep("[", 0, 0), Token::sep("]", 0, 0)];
        let obj_toks = vec![Token::sep("{", 0, 0), Token::sep("}", 0, 0)];
        let mut inp = vec![
            Token::sep("[", 0, 0),
            null_tok.clone(),
            Token::sep(",", 0, 0),
            false_tok.clone(),
            Token::sep(",", 0, 0),
            true_tok.clone(),
            Token::sep(",", 0, 0),
            num_tok.clone(),
            Token::sep(",", 0, 0),
            str_tok.clone(),
            Token::sep(",", 0, 0),
        ];

        inp.append(&mut arr_toks.clone());
        inp.push(Token::sep(",", 0, 0));
        inp.append(&mut obj_toks.clone());
        inp.push(Token::sep("]", 0, 0));

        let r = array_consumer(&inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: inp.len(),
            node: Some(Node::new_arr(
                vec![
                    Node::new_null(null_tok),
                    Node::new_bool(false, false_tok),
                    Node::new_bool(true, true_tok),
                    Node::new_num("123.456", num_tok),
                    Node::new_str("test_string", str_tok),
                    Node::new_arr(Vec::new(), arr_toks),
                    Node::new_obj(HashMap::new(), obj_toks),
                ],
                inp.to_vec(),
            )),
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
