use std::collections::HashMap;

use super::{
    consumer_response::ConsumerResponse,
    error::TreebuilderError,
    node::{Node, NodeType},
    value_consumer::value_consumer,
};
use crate::tokenizer::{Token, TokenType};

/// Consumes an object composition. Non object compositions (e.g. not object
/// open) are ignored.
pub fn object_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, TreebuilderError> {
    let open = toks.get(offset).unwrap();
    if !is_obj_open(open) {
        return Ok(ConsumerResponse {
            cons: 0,
            node: None,
        });
    }

    let mut consumed_tokens = Vec::new();

    consumed_tokens.push(open.clone());

    let mut entries = HashMap::new();

    loop {
        let mut left = match toks.get(consumed_tokens.len() + offset) {
            None => {
                return Err(TreebuilderError::new_unterminated_cont(NodeType::Object));
            }
            t => t.unwrap(),
        };

        if is_obj_close(left) {
            consumed_tokens.push(left.clone());
            break;
        }

        if consumed_tokens.len() > 1 {
            if !is_separator(left) {
                return Err(TreebuilderError::new_exp_sep_or_close(left.clone()));
            } else {
                consumed_tokens.push(left.clone());

                left = match toks.get(consumed_tokens.len() + offset) {
                    None => {
                        return Err(TreebuilderError::new_unterminated_cont(NodeType::Object));
                    }
                    t => t.unwrap(),
                };
            }
        }

        if left.typ != TokenType::StringLiteral {
            return Err(TreebuilderError::new_exp_obj_key(left.clone()));
        }

        consumed_tokens.push(left.clone());

        let assign = toks.get(consumed_tokens.len() + offset).unwrap();

        if !is_assignment(assign) {
            return Err(TreebuilderError::new_exp_assign(assign.clone()));
        }

        consumed_tokens.push(assign.clone());

        let value_consume = value_consumer(toks, consumed_tokens.len() + offset)?;
        let value = match value_consume.node {
            None => {
                let unexp = toks.get(consumed_tokens.len() + offset).unwrap();
                return Err(TreebuilderError::new_exp_val_comp(unexp.clone()));
            }
            n => n.unwrap(),
        };

        consumed_tokens.append(&mut value.toks());

        entries.insert(left.val.clone(), value);
    }

    Ok(ConsumerResponse {
        cons: consumed_tokens.len(),
        node: Some(Node::new_obj(entries, consumed_tokens)),
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
        let r = object_consumer(&[Token::kwd("null", 0, 0)], 0).unwrap();
        let e = ConsumerResponse {
            cons: 0,
            node: None,
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn empty_object() {
        let inp = vec![Token::sep("{", 0, 0), Token::sep("}", 0, 0)];
        let r = object_consumer(&inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 2,
            node: Some(Node::new_obj(HashMap::new(), inp)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn unterminated_object() {
        let r = object_consumer(&[Token::sep("{", 0, 0)], 0).unwrap_err();
        let e = TreebuilderError::new_unterminated_cont(NodeType::Object);

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_keyword_entry() {
        let false_tok = Token::kwd("false", 0, 0);
        let r_false = object_consumer(&gen_input(false_tok.clone()), 0).unwrap();
        let e_false = gen_exp(Node::new_bool(false, false_tok.clone()), false_tok);

        assert_eq!(r_false, e_false);

        let true_tok = Token::kwd("true", 0, 0);
        let r_true = object_consumer(&gen_input(true_tok.clone()), 0).unwrap();
        let e_true = gen_exp(Node::new_bool(true, true_tok.clone()), true_tok);

        assert_eq!(r_true, e_true);

        let null_tok = Token::kwd("null", 0, 0);
        let r_null = object_consumer(&gen_input(null_tok.clone()), 0).unwrap();
        let e_null = gen_exp(Node::new_null(null_tok.clone()), null_tok);

        assert_eq!(r_null, e_null);

        let string_tok = Token::str("test string", 0, 0);
        let r_string = object_consumer(&gen_input(string_tok.clone()), 0).unwrap();
        let e_string = gen_exp(Node::new_str("test string", string_tok.clone()), string_tok);

        assert_eq!(r_string, e_string);

        fn gen_input(val_tok: Token) -> Vec<Token> {
            vec![
                Token::sep("{", 0, 0),
                Token::str("keyword_key", 0, 0),
                Token::op(":", 0, 0),
                val_tok,
                Token::sep("}", 0, 0),
            ]
        }
        fn gen_exp(val_node: Node, val_tok: Token) -> ConsumerResponse {
            let mut entries = HashMap::new();

            entries.insert("keyword_key".into(), val_node);

            ConsumerResponse {
                cons: 5,
                node: Some(Node::new_obj(entries, gen_input(val_tok))),
            }
        }
    }
    #[test]
    pub fn single_number_entry() {
        let num_tok = Token::num("123.456", 0, 0);
        let inp = vec![
            Token::sep("{", 0, 0),
            Token::str("number_key", 0, 0),
            Token::op(":", 0, 0),
            num_tok.clone(),
            Token::sep("}", 0, 0),
        ];

        let mut entries = HashMap::new();

        entries.insert("number_key".into(), Node::new_num("123.456", num_tok));

        let r = object_consumer(&inp.clone(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            node: Some(Node::new_obj(entries, inp)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_string_entry() {
        let str_tok = Token::str("string_value", 0, 0);
        let inp = vec![
            Token::sep("{", 0, 0),
            Token::str("string_key", 0, 0),
            Token::op(":", 0, 0),
            str_tok.clone(),
            Token::sep("}", 0, 0),
        ];

        let mut entries = HashMap::new();

        entries.insert("string_key".into(), Node::new_str("string_value", str_tok));

        let r = object_consumer(&inp.clone(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            node: Some(Node::new_obj(entries, inp)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_array_entry() {
        let mut inp = vec![
            Token::sep("{", 0, 0),
            Token::str("array_key", 0, 0),
            Token::op(":", 0, 0),
        ];
        let arr_toks = vec![Token::sep("[", 0, 0), Token::sep("]", 0, 0)];

        inp.append(&mut arr_toks.clone());
        inp.push(Token::sep("}", 0, 0));

        let mut entries = HashMap::new();

        entries.insert(
            String::from("array_key"),
            Node::new_arr(Vec::new(), arr_toks),
        );

        let r = object_consumer(&inp.clone(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 6,
            node: Some(Node::new_obj(entries, inp)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn nested_object() {
        let inner_val_tok = Token::str("inner_val", 0, 0);
        let inner_obj_toks = vec![
            Token::sep("{", 0, 0),
            Token::str("inner_key", 0, 0),
            Token::op(":", 0, 0),
            inner_val_tok.clone(),
            Token::sep("}", 0, 0),
        ];
        let mut inp = vec![
            Token::sep("{", 0, 0),
            Token::str("object_key", 0, 0),
            Token::op(":", 0, 0),
        ];

        inp.append(&mut inner_obj_toks.clone());
        inp.push(Token::sep("}", 0, 0));

        let mut inner_entries = HashMap::new();

        inner_entries.insert(
            String::from("inner_key"),
            Node::new_str("inner_val", inner_val_tok),
        );

        let mut entries = HashMap::new();

        entries.insert(
            String::from("object_key"),
            Node::new_obj(inner_entries, inner_obj_toks),
        );

        let r = object_consumer(&inp.clone(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 9,
            node: Some(Node::new_obj(entries, inp)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn multiple_entries() {
        let null_tok = Token::kwd("null", 0, 0);
        let num_tok = Token::num("123.456", 0, 0);
        let str_tok = Token::str("string value", 0, 0);
        let inner_obj_toks = vec![Token::sep("{", 0, 0), Token::sep("}", 0, 0)];
        let mut inp = vec![
            Token::sep("{", 0, 0),
            Token::str("keyword_key", 0, 0),
            Token::op(":", 0, 0),
            null_tok.clone(),
            Token::sep(",", 0, 0),
            Token::str("number_key", 0, 0),
            Token::op(":", 0, 0),
            num_tok.clone(),
            Token::sep(",", 0, 0),
            Token::str("string_key", 0, 0),
            Token::op(":", 0, 0),
            str_tok.clone(),
            Token::sep(",", 0, 0),
            Token::str("object_key", 0, 0),
            Token::op(":", 0, 0),
        ];

        inp.append(&mut inner_obj_toks.clone());
        inp.push(Token::sep("}", 0, 0));

        let r = object_consumer(&inp.clone(), 0).unwrap();

        let mut entries = HashMap::new();

        entries.insert(String::from("keyword_key"), Node::new_null(null_tok));
        entries.insert(
            String::from("number_key"),
            Node::new_num("123.456", num_tok),
        );
        entries.insert(
            String::from("string_key"),
            Node::new_str("string value", str_tok),
        );
        entries.insert(
            String::from("object_key"),
            Node::new_obj(HashMap::new(), inner_obj_toks),
        );

        let e = ConsumerResponse {
            cons: inp.len(),
            node: Some(Node::new_obj(entries, inp)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn invalid_key_token() {
        let inp = &[
            Token::sep("{", 0, 0),
            Token::kwd("null", 0, 0),
            Token::op(":", 0, 0),
            Token::kwd("null", 0, 0),
            Token::sep("}", 0, 0),
        ];
        let r = object_consumer(inp, 0).unwrap_err();
        let e = TreebuilderError::new_exp_obj_key(Token::kwd("null", 0, 0));

        assert_eq!(r, e);
    }
    #[test]
    pub fn invalid_assignment_token() {
        let inp = &[
            Token::sep("{", 0, 0),
            Token::str("key", 0, 0),
            Token::kwd("null", 0, 0),
            Token::kwd("null", 0, 0),
            Token::sep("}", 0, 0),
        ];
        let r = object_consumer(inp, 0).unwrap_err();
        let e = TreebuilderError::new_exp_assign(Token::kwd("null", 0, 0));

        assert_eq!(r, e);
    }
    #[test]
    pub fn invalid_value_token() {
        test(Token::op(":", 0, 0));
        test(Token::sep(",", 0, 0));

        fn test(val_tok: Token) {
            let inp = &[
                Token::sep("{", 0, 0),
                Token::str("key", 0, 0),
                Token::op(":", 0, 0),
                val_tok.clone(),
                Token::sep("}", 0, 0),
            ];

            let r = object_consumer(inp, 0).unwrap_err();
            let e = TreebuilderError::new_exp_val_comp(val_tok);

            assert_eq!(r, e);
        }
    }
}
