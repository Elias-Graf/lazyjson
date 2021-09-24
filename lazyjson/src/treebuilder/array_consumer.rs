use std::iter::Peekable;

use crate::tokenizer::{Token, TokenIndices, TokenType};

use super::{
    consumer_response::ConsumerResponse,
    error::{OldTreebuilderError, TreebuilderErr},
    node::Node,
    old_node::OldNode,
    value_consumer::{old_value_consumer, value_consumer},
};

// TODO: refactor this hot garbage 😉
pub fn array_consumer(toks: &mut Peekable<TokenIndices>) -> Result<Option<Node>, TreebuilderErr> {
    if toks.peek().is_none() {
        return Err(TreebuilderErr::new_out_of_bounds());
    }

    let (_, opn_tok) = toks.peek().unwrap();

    if opn_tok.typ != TokenType::Separator || opn_tok.val != "[" {
        return Ok(None);
    }

    let (opn_i, _) = toks.next().unwrap();
    let mut entries = Vec::new();
    let mut last_i = opn_i;

    let (peek_i, peek_t) = match toks.peek() {
        None => return Err(TreebuilderErr::new_unterminated_arr(opn_i, last_i + 1)),
        Some(p) => p,
    };

    if peek_t.typ == TokenType::Separator && peek_t.val == "]" {
        last_i = *peek_i;

        toks.next();

        return Ok(Some(Node::new_arr(entries, opn_i, last_i + 1)));
    }

    loop {
        match toks.peek() {
            None => return Err(TreebuilderErr::new_unterminated_arr(opn_i, last_i + 1)),
            Some(&(i, t)) => {
                if t.typ == TokenType::Separator && t.val == "]" {
                    return Err(TreebuilderErr::new_trailing_sep(i - 1));
                }
            }
        };

        match value_consumer(toks)? {
            None => panic!(
                "array consumer expected some sort of value[composition], but received {:?}",
                toks.peek()
            ),
            Some(entry) => {
                last_i = entry.to;

                entries.push(entry);
            }
        };

        let (peek_i, peek_t) = match toks.peek() {
            None => return Err(TreebuilderErr::new_unterminated_arr(opn_i, last_i + 1)),
            Some(p) => p,
        };

        if peek_t.typ == TokenType::Separator && peek_t.val == "]" {
            last_i = *peek_i;

            toks.next();

            return Ok(Some(Node::new_arr(entries, opn_i, last_i + 1)));
        } else if peek_t.typ != TokenType::Separator || peek_t.val != "," {
            return Err(TreebuilderErr::new_not_a_sep(last_i));
        } else {
            last_i = *peek_i;

            toks.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn empty_input() {
        let toks = [];
        let r = array_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_array() {
        let toks = [Token::num("0", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = array_consumer(toks_iter).unwrap();
        let e = None;

        assert_eq!(r, e);
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::num("0", 0, 0)));
    }

    #[test]
    fn unterminated() {
        let toks = [Token::sep("[", 0, 0)];
        let r = array_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_unterminated_arr(0, 1);

        assert_eq!(r, e);
    }

    #[test]
    fn missing_sep() {
        let toks = [
            Token::sep("[", 0, 0),
            Token::num("1", 0, 0),
            Token::num("1", 0, 0),
            Token::sep("]", 0, 0),
        ];
        let r = array_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_not_a_sep(2);

        assert_eq!(r, e);
    }

    #[test]
    fn trailing_sep() {
        let toks = [
            Token::sep("[", 0, 0),
            Token::num("123", 0, 0),
            Token::sep(",", 0, 0),
            Token::sep("]", 0, 0),
        ];
        let r = array_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_trailing_sep(2);

        assert_eq!(r, e);
    }

    #[test]
    fn empty() {
        let toks = [Token::sep("[", 0, 0), Token::sep("]", 0, 0)];
        let r = array_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_arr(Vec::new(), 0, 2));

        assert_eq!(r, e);
    }

    #[test]
    fn single_entry() {
        let toks = [
            Token::sep("[", 0, 0),
            Token::num("123", 0, 0),
            Token::sep("]", 0, 0),
        ];
        let r = array_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_arr(vec![Node::new_num("123", 1, 2)], 0, 3));

        assert_eq!(r, e);
    }

    #[test]
    fn multiple_entries() {
        let toks = [
            Token::sep("[", 0, 0),
            Token::sep("[", 0, 0),
            Token::sep("]", 0, 0),
            Token::sep(",", 0, 0),
            Token::kwd("false", 0, 0),
            Token::sep(",", 0, 0),
            Token::num("123", 0, 0),
            Token::sep(",", 0, 0),
            Token::sep("{", 0, 0),
            Token::sep("}", 0, 0),
            Token::sep(",", 0, 0),
            Token::str("Hello, World!", 0, 0),
            Token::sep("]", 0, 0),
        ];
        let r = array_consumer(&mut toks.iter().enumerate().peekable()).unwrap();
        let e = Some(Node::new_arr(
            vec![
                Node::new_arr(Vec::new(), 1, 3),
                Node::new_bool(false, 4, 5),
                Node::new_num("123", 6, 7),
                Node::new_obj(HashMap::new(), 8, 10),
                Node::new_str("Hello, World!", 11, 12),
            ],
            0,
            13,
        ));

        assert_eq!(r, e);
    }
}

/// Consumes an array composition. Non array composition (e.g. not array open)
/// are ignored.
pub fn old_array_consumer(
    toks: &[Token],
    offset: usize,
) -> Result<ConsumerResponse, OldTreebuilderError> {
    let first = toks.get(offset).unwrap();

    if !is_array_open(first) {
        return Ok(ConsumerResponse {
            cons: 0,
            node: None,
        });
    }

    let mut consumed_toks = Vec::new();

    consumed_toks.push(first.clone());

    let mut entries: Vec<OldNode> = Vec::new();

    let maybe_close = match toks.get(consumed_toks.len() + offset) {
        None => {
            return Err(OldTreebuilderError::new_unterminated_arr());
        }
        t => t.unwrap(),
    };

    if maybe_close.typ == TokenType::Separator && maybe_close.val == "]" {
        consumed_toks.push(maybe_close.clone());

        return Ok(ConsumerResponse {
            cons: consumed_toks.len(),
            node: Some(OldNode::new_arr(entries, consumed_toks)),
        });
    }

    loop {
        let child_consume = old_value_consumer(toks, consumed_toks.len() + offset)?;
        let child = match child_consume.node {
            None => {
                let unexp = toks.get(consumed_toks.len() + offset).unwrap().clone();
                return Err(OldTreebuilderError::new_exp_val_comp(unexp));
            }
            n => n.unwrap(),
        };

        let mut other = child.toks().clone();

        consumed_toks.append(&mut other);
        entries.push(child);

        let sep_or_close = match toks.get(consumed_toks.len() + offset) {
            None => {
                return Err(OldTreebuilderError::new_unterminated_arr());
            }
            t => t.unwrap(),
        };

        if sep_or_close.typ == TokenType::Separator && sep_or_close.val == "," {
            consumed_toks.push(sep_or_close.clone());
        } else if sep_or_close.typ == TokenType::Separator && sep_or_close.val == "]" {
            consumed_toks.push(sep_or_close.clone());
            break;
        } else {
            return Err(OldTreebuilderError::new_exp_sep_or_close(
                sep_or_close.clone(),
            ));
        }
    }

    Ok(ConsumerResponse {
        cons: consumed_toks.len(),
        node: Some(OldNode::new_arr(entries, consumed_toks)),
    })
}

fn is_array_open(tok: &Token) -> bool {
    tok.typ == TokenType::Separator && tok.val == "["
}

#[cfg(test)]
mod old_tests {
    use std::collections::HashMap;

    use crate::tokenizer::Token;

    use super::*;

    #[test]
    pub fn non_array() {
        let inp = &[Token::kwd("null", 0, 0)];

        let r = old_array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 0,
            node: None,
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn empty_array() {
        let inp = &[Token::sep("[", 0, 0), Token::sep("]", 0, 0)];

        let r = old_array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 2,
            node: Some(OldNode::new_arr(Vec::new(), inp.to_vec())),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_keyword() {
        let kwd = Token::kwd("false", 0, 0);
        let inp = &[Token::sep("[", 0, 0), kwd.clone(), Token::sep("]", 0, 0)];

        let r = old_array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(OldNode::new_arr(
                vec![OldNode::new_bool(false, kwd)],
                inp.to_vec(),
            )),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_number() {
        let num = Token::num("1", 0, 0);
        let inp = &[Token::sep("[", 0, 0), num.clone(), Token::sep("]", 0, 0)];

        let r = old_array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(OldNode::new_arr(
                vec![OldNode::new_num("1", num)],
                inp.to_vec(),
            )),
        };

        assert_eq!(r, e);
    }
    #[test]
    pub fn single_string() {
        let str = Token::str("test_string", 0, 0);
        let inp = &[Token::sep("[", 0, 0), str.clone(), Token::sep("]", 0, 0)];

        let r = old_array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 3,
            node: Some(OldNode::new_arr(
                vec![OldNode::new_str("test_string", str)],
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

        let r = old_array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 4,
            node: Some(OldNode::new_arr(
                vec![OldNode::new_obj(HashMap::new(), inp[1..3].to_vec())],
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

        let r = old_array_consumer(inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: 4,
            node: Some(OldNode::new_arr(
                vec![OldNode::new_arr(Vec::new(), inp[1..3].to_vec())],
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

        let r = old_array_consumer(&inp, 0).unwrap();
        let e = ConsumerResponse {
            cons: inp.len(),
            node: Some(OldNode::new_arr(
                vec![
                    OldNode::new_null(null_tok),
                    OldNode::new_bool(false, false_tok),
                    OldNode::new_bool(true, true_tok),
                    OldNode::new_num("123.456", num_tok),
                    OldNode::new_str("test_string", str_tok),
                    OldNode::new_arr(Vec::new(), arr_toks),
                    OldNode::new_obj(HashMap::new(), obj_toks),
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
        let r = old_array_consumer(inp, 0).unwrap_err();
        let e = OldTreebuilderError::new_exp_sep_or_close(Token::kwd("null", 0, 0));

        assert_eq!(r, e);
    }
    #[test]
    pub fn unterminated() {
        let inputs = &[
            vec![Token::sep("[", 0, 0)],
            vec![Token::sep("[", 0, 0), Token::kwd("null", 0, 0)],
        ];

        for inp in inputs {
            let r = old_array_consumer(inp, 0).unwrap_err();
            let e = OldTreebuilderError::new_unterminated_arr();

            assert_eq!(r, e);
        }
    }
    #[test]
    pub fn invalid_value() {
        let inp = &[
            Token::sep("[", 0, 0),
            Token::sep(",", 0, 0),
            Token::sep("]", 0, 0),
        ];
        let r = old_array_consumer(inp, 0).unwrap_err();
        let e = OldTreebuilderError::new_exp_val_comp(Token::sep(",", 0, 0));

        assert_eq!(r, e);
    }
}
