use std::{collections::HashMap, iter::Peekable};

use super::{
    consumer_response::ConsumerResponse,
    error::{OldTreebuilderError, TreebuilderErr},
    node::Node,
    old_node::OldNode,
    string_consumer::string_consumer,
    value_consumer::{old_value_consumer, value_consumer},
};
use crate::{
    tokenizer::{Token, TokenIndices, TokenType},
    treebuilder::node::NodeSpecific,
};

// TODO: refactor this hot garbage 😉
pub fn object_consumer(toks: &mut Peekable<TokenIndices>) -> Result<Option<Node>, TreebuilderErr> {
    let &(opn_i, opn_t) = match toks.peek() {
        None => return Err(TreebuilderErr::new_out_of_bounds()),
        Some(p) => p,
    };

    if opn_t.typ != TokenType::Separator || opn_t.val != "{" {
        return Ok(None);
    }

    toks.next();

    let mut entries = HashMap::new();

    match toks.peek() {
        None => return Err(TreebuilderErr::new_unterminated_obj(opn_i, opn_i + 1)),
        Some(&(cls_i, cls_t)) => {
            if cls_t.typ == TokenType::Separator && cls_t.val == "}" {
                toks.next();
                return Ok(Some(Node::new_obj(entries, opn_i, cls_i + 1)));
            }
        }
    }

    loop {
        match toks.peek() {
            None => return Err(TreebuilderErr::new_unterminated_obj(opn_i, opn_i + 1)),
            Some(&(i, t)) => {
                if t.typ == TokenType::Separator && t.val == "}" {
                    return Err(TreebuilderErr::new_trailing_sep(i - 1));
                }
            }
        }

        let (key_i, key) = match string_consumer(toks)? {
            None => return Err(TreebuilderErr::new_not_a_key(toks.next().unwrap().0)),
            Some(n) => match n.specific {
                NodeSpecific::String(k) => (n.from, k.val),
                _ => panic!(
                    "string_consumer should only return string node but returned {:?}",
                    n
                ),
            },
        };

        match toks.next() {
            None => return Err(TreebuilderErr::new_unterminated_obj(key_i, key_i + 1)),
            Some((i, assign_op_t)) => {
                if assign_op_t.typ != TokenType::Operator || assign_op_t.val != ":" {
                    return Err(TreebuilderErr::new_not_an_assignment_op(i));
                }
            }
        }

        let val = match value_consumer(toks)? {
            None => return Err(TreebuilderErr::new_not_val(toks.next().unwrap().0)),
            Some(v) => v,
        };

        entries.insert(key, val);

        let (sep_or_cls_i, sep_or_cls_t) = match toks.next() {
            None => return Err(TreebuilderErr::new_unterminated_obj(opn_i, opn_i + 1)),
            Some(n) => n,
        };

        if sep_or_cls_t.typ == TokenType::Separator && sep_or_cls_t.val == "}" {
            return Ok(Some(Node::new_obj(entries, opn_i, sep_or_cls_i + 1)));
        } else if sep_or_cls_t.typ != TokenType::Separator || sep_or_cls_t.val != "," {
            return Err(TreebuilderErr::new_not_a_sep(sep_or_cls_i));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end_of_input() {
        let toks = [];
        let r = object_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_object() {
        let toks = [Token::num("123", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = object_consumer(toks_iter).unwrap();
        let e = None;

        assert_eq!(r, e);
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::num("123", 0, 0)));
    }

    #[test]
    fn unterminated() {
        let toks = [Token::sep("{", 0, 0)];
        let r = object_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_unterminated_obj(0, 1);

        assert_eq!(r, e);
    }

    #[test]
    fn invalid_key() {
        let toks = [
            Token::sep("{", 0, 0),
            Token::kwd("false", 0, 0),
            Token::op(":", 0, 0),
            Token::str("val", 0, 0),
            Token::sep("}", 0, 0),
        ];
        let r = object_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_not_a_key(1);

        assert_eq!(r, e);
    }

    #[test]
    fn invalid_assignment() {
        let toks = [
            Token::sep("{", 0, 0),
            Token::str("key", 0, 0),
            Token::str(":", 0, 0),
            Token::str("val", 0, 0),
            Token::sep("}", 0, 0),
        ];
        let r = object_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_not_an_assignment_op(2);

        assert_eq!(r, e);
    }

    #[test]
    fn trailing_sep() {
        let toks = [
            Token::sep("{", 0, 0),
            Token::str("key", 0, 0),
            Token::op(":", 0, 0),
            Token::str("val", 0, 0),
            Token::sep(",", 0, 0),
            Token::sep("}", 0, 0),
        ];

        let mut e_entries = HashMap::new();

        e_entries.insert("key".to_string(), Node::new_str("val", 3, 4));

        let r = object_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_trailing_sep(4);

        assert_eq!(r, e);
    }

    #[test]
    fn missing_comma() {
        let toks = [
            Token::sep("{", 0, 0),
            Token::str("key1", 0, 0),
            Token::op(":", 0, 0),
            Token::str("val1", 0, 0),
            Token::str("key2", 0, 0),
            Token::op(":", 0, 0),
            Token::str("val2", 0, 0),
            Token::sep("}", 0, 0),
        ];

        let mut e_entries = HashMap::new();

        e_entries.insert("key".to_string(), Node::new_str("val", 3, 4));

        let r = object_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_not_a_sep(4);

        assert_eq!(r, e);
    }

    #[test]
    fn empty() {
        let toks = [Token::sep("{", 0, 0), Token::sep("}", 0, 0)];
        let r = object_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_obj(HashMap::new(), 0, 2));

        assert_eq!(r, e);
    }

    #[test]
    fn single_entry() {
        let toks = [
            Token::sep("{", 0, 0),
            Token::str("key", 0, 0),
            Token::op(":", 0, 0),
            Token::str("val", 0, 0),
            Token::sep("}", 0, 0),
        ];

        let mut e_entries = HashMap::new();

        e_entries.insert("key".to_string(), Node::new_str("val", 3, 4));

        let r = object_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_obj(e_entries, 0, 5));

        assert_eq!(r, e);
    }

    #[test]
    fn multiple_entries() {
        let toks = [
            Token::sep("{", 0, 0),
            Token::str("key_arr", 0, 0),
            Token::op(":", 0, 0),
            Token::sep("[", 0, 0),
            Token::sep("]", 0, 0),
            Token::sep(",", 0, 0),
            Token::str("key_kwd", 0, 0),
            Token::op(":", 0, 0),
            Token::kwd("false", 0, 0),
            Token::sep(",", 0, 0),
            Token::str("key_num", 0, 0),
            Token::op(":", 0, 0),
            Token::num("123", 0, 0),
            Token::sep(",", 0, 0),
            Token::str("key_obj", 0, 0),
            Token::op(":", 0, 0),
            Token::sep("{", 0, 0),
            Token::sep("}", 0, 0),
            Token::sep(",", 0, 0),
            Token::str("key_str", 0, 0),
            Token::op(":", 0, 0),
            Token::str("Hello, World!", 0, 0),
            Token::sep("}", 0, 0),
        ];

        let mut e_entries = HashMap::new();

        e_entries.insert("key_arr".to_string(), Node::new_arr(Vec::new(), 3, 5));
        e_entries.insert("key_kwd".to_string(), Node::new_bool(false, 8, 9));
        e_entries.insert("key_num".to_string(), Node::new_num("123", 12, 13));
        e_entries.insert("key_obj".to_string(), Node::new_obj(HashMap::new(), 16, 18));
        e_entries.insert(
            "key_str".to_string(),
            Node::new_str("Hello, World!", 21, 22),
        );

        let r = object_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_obj(e_entries, 0, 23));

        assert_eq!(r, e);
    }
}

/// Consumes an object composition. Non object compositions (e.g. not object
/// open) are ignored.
pub fn old_object_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, OldTreebuilderError> {
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
                return Err(OldTreebuilderError::new_unterminated_obj());
            }
            t => t.unwrap(),
        };

        if is_obj_close(left) {
            consumed_tokens.push(left.clone());
            break;
        }

        if consumed_tokens.len() > 1 {
            if !is_separator(left) {
                return Err(OldTreebuilderError::new_exp_sep_or_close(left.clone()));
            } else {
                consumed_tokens.push(left.clone());

                left = match toks.get(consumed_tokens.len() + offset) {
                    None => {
                        return Err(OldTreebuilderError::new_unterminated_obj());
                    }
                    t => t.unwrap(),
                };
            }
        }

        if left.typ != TokenType::StringLiteral {
            return Err(OldTreebuilderError::new_exp_obj_key(left.clone()));
        }

        consumed_tokens.push(left.clone());

        let assign = toks.get(consumed_tokens.len() + offset).unwrap();

        if !is_assignment(assign) {
            return Err(OldTreebuilderError::new_exp_assign(assign.clone()));
        }

        consumed_tokens.push(assign.clone());

        let value_consume = old_value_consumer(toks, consumed_tokens.len() + offset)?;
        let value = match value_consume.node {
            None => {
                let unexp = toks.get(consumed_tokens.len() + offset).unwrap();
                return Err(OldTreebuilderError::new_exp_val_comp(unexp.clone()));
            }
            n => n.unwrap(),
        };

        consumed_tokens.append(&mut value.toks());

        entries.insert(left.val.clone(), value);
    }

    Ok(ConsumerResponse {
        cons: consumed_tokens.len(),
        node: Some(OldNode::new_obj(entries, consumed_tokens)),
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
mod old_tests {
    use super::*;

    #[test]
    pub fn non_object() {
        let r = old_object_consumer(&[Token::kwd("null", 0, 0)], 0).unwrap();
        let e = ConsumerResponse {
            cons: 0,
            node: None,
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn empty_object() {
        let inp = vec![Token::sep("{", 0, 0), Token::sep("}", 0, 0)];
        let r = old_object_consumer(&inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 2,
            node: Some(OldNode::new_obj(HashMap::new(), inp)),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn unterminated_object() {
        let r = old_object_consumer(&[Token::sep("{", 0, 0)], 0).unwrap_err();
        let e = OldTreebuilderError::new_unterminated_obj();

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_keyword_entry() {
        let false_tok = Token::kwd("false", 0, 0);
        let r_false = old_object_consumer(&gen_input(false_tok.clone()), 0).unwrap();
        let e_false = gen_exp(OldNode::new_bool(false, false_tok.clone()), false_tok);

        assert_eq!(r_false, e_false);

        let true_tok = Token::kwd("true", 0, 0);
        let r_true = old_object_consumer(&gen_input(true_tok.clone()), 0).unwrap();
        let e_true = gen_exp(OldNode::new_bool(true, true_tok.clone()), true_tok);

        assert_eq!(r_true, e_true);

        let null_tok = Token::kwd("null", 0, 0);
        let r_null = old_object_consumer(&gen_input(null_tok.clone()), 0).unwrap();
        let e_null = gen_exp(OldNode::new_null(null_tok.clone()), null_tok);

        assert_eq!(r_null, e_null);

        let string_tok = Token::str("test string", 0, 0);
        let r_string = old_object_consumer(&gen_input(string_tok.clone()), 0).unwrap();
        let e_string = gen_exp(
            OldNode::new_str("test string", string_tok.clone()),
            string_tok,
        );

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
        fn gen_exp(val_node: OldNode, val_tok: Token) -> ConsumerResponse {
            let mut entries = HashMap::new();

            entries.insert("keyword_key".into(), val_node);

            ConsumerResponse {
                cons: 5,
                node: Some(OldNode::new_obj(entries, gen_input(val_tok))),
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

        entries.insert("number_key".into(), OldNode::new_num("123.456", num_tok));

        let r = old_object_consumer(&inp.clone(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            node: Some(OldNode::new_obj(entries, inp)),
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

        entries.insert(
            "string_key".into(),
            OldNode::new_str("string_value", str_tok),
        );

        let r = old_object_consumer(&inp.clone(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 5,
            node: Some(OldNode::new_obj(entries, inp)),
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
            OldNode::new_arr(Vec::new(), arr_toks),
        );

        let r = old_object_consumer(&inp.clone(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 6,
            node: Some(OldNode::new_obj(entries, inp)),
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
            OldNode::new_str("inner_val", inner_val_tok),
        );

        let mut entries = HashMap::new();

        entries.insert(
            String::from("object_key"),
            OldNode::new_obj(inner_entries, inner_obj_toks),
        );

        let r = old_object_consumer(&inp.clone(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 9,
            node: Some(OldNode::new_obj(entries, inp)),
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

        let r = old_object_consumer(&inp.clone(), 0).unwrap();

        let mut entries = HashMap::new();

        entries.insert(String::from("keyword_key"), OldNode::new_null(null_tok));
        entries.insert(
            String::from("number_key"),
            OldNode::new_num("123.456", num_tok),
        );
        entries.insert(
            String::from("string_key"),
            OldNode::new_str("string value", str_tok),
        );
        entries.insert(
            String::from("object_key"),
            OldNode::new_obj(HashMap::new(), inner_obj_toks),
        );

        let e = ConsumerResponse {
            cons: inp.len(),
            node: Some(OldNode::new_obj(entries, inp)),
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
        let r = old_object_consumer(inp, 0).unwrap_err();
        let e = OldTreebuilderError::new_exp_obj_key(Token::kwd("null", 0, 0));

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
        let r = old_object_consumer(inp, 0).unwrap_err();
        let e = OldTreebuilderError::new_exp_assign(Token::kwd("null", 0, 0));

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

            let r = old_object_consumer(inp, 0).unwrap_err();
            let e = OldTreebuilderError::new_exp_val_comp(val_tok);

            assert_eq!(r, e);
        }
    }
}
