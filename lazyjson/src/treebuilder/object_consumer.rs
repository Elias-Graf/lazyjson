use std::{collections::HashMap, iter::Peekable};

use super::{
    config::Config, error::TreebuilderErr, node::Node, string_consumer::string_consumer,
    value_consumer::value_consumer,
};
use crate::{
    tokenizer::{Token, TokenIndices, TokenType},
    treebuilder::node::NodeSpecific,
};

// TODO: refactor this hot garbage 😉
pub fn object_consumer(
    toks: &mut Peekable<TokenIndices>,
    config: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let &(opn_i, opn_t) = match toks.peek() {
        None => return Err(TreebuilderErr::new_out_of_bounds()),
        Some(p) => p,
    };

    if !is_obj_opn(opn_t) {
        return Ok(None);
    }

    toks.next();

    let mut entries = HashMap::new();

    match toks.peek() {
        None => return Err(TreebuilderErr::new_unterminated_obj(opn_i, opn_i + 1)),
        Some(&(cls_i, cls_t)) => {
            if is_obj_cls(cls_t) {
                toks.next();
                return Ok(Some(Node::new_obj(entries, opn_i, cls_i + 1)));
            }
        }
    }

    loop {
        match toks.peek() {
            None => return Err(TreebuilderErr::new_unterminated_obj(opn_i, opn_i + 1)),
            Some(&(i, t)) => {
                if is_obj_cls(t) {
                    if config.allow_trailing_commas {
                        toks.next();
                        return Ok(Some(Node::new_obj(entries, opn_i, i + 1)));
                    }

                    return Err(TreebuilderErr::new_trailing_sep(i - 1));
                }
            }
        }

        let (key_i, key) = match string_consumer(toks, &Config::DEFAULT)? {
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
                    return Err(TreebuilderErr::new_not_an_assignment(i));
                }
            }
        }

        let val = match value_consumer(toks, &Config::DEFAULT)? {
            None => return Err(TreebuilderErr::new_not_a_val(toks.next().unwrap().0)),
            Some(v) => v,
        };

        entries.insert(key, val);

        let (sep_or_cls_i, sep_or_cls_t) = match toks.next() {
            None => return Err(TreebuilderErr::new_unterminated_obj(opn_i, opn_i + 1)),
            Some(n) => n,
        };

        if is_obj_cls(sep_or_cls_t) {
            return Ok(Some(Node::new_obj(entries, opn_i, sep_or_cls_i + 1)));
        } else if sep_or_cls_t.typ != TokenType::Separator || sep_or_cls_t.val != "," {
            return Err(TreebuilderErr::new_not_a_sep(sep_or_cls_i));
        }
    }
}

fn is_obj_opn(t: &Token) -> bool {
    t.typ == TokenType::Delimiter && t.val == "{"
}

fn is_obj_cls(t: &Token) -> bool {
    t.typ == TokenType::Delimiter && t.val == "}"
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Token;

    use super::*;

    #[test]
    fn end_of_input() {
        let toks = [];
        let r =
            object_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap_err();
        let e = TreebuilderErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_object() {
        let toks = [Token::new_num("123", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = object_consumer(toks_iter, &Config::DEFAULT).unwrap();
        let e = None;

        assert_eq!(r, e);
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::new_num("123", 0, 0)));
    }

    #[test]
    fn unterminated() {
        let toks = [Token::new_delimiter("{", 0, 0)];
        let r =
            object_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap_err();
        let e = TreebuilderErr::new_unterminated_obj(0, 1);

        assert_eq!(r, e);
    }

    #[test]
    fn invalid_key() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_kwd("false", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_str("val", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let r =
            object_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap_err();
        let e = TreebuilderErr::new_not_a_key(1);

        assert_eq!(r, e);
    }

    #[test]
    fn invalid_assignment() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key", 0, 0),
            Token::new_str(":", 0, 0),
            Token::new_str("val", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let r =
            object_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap_err();
        let e = TreebuilderErr::new_not_an_assignment(2);

        assert_eq!(r, e);
    }

    #[test]
    fn trailing_sep_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = true;

        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_str("val", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];

        let mut entries = HashMap::new();
        entries.insert("key".to_string(), Node::new_str("val", 3, 4));

        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = object_consumer(toks_iter, &config).unwrap();
        let e = Some(Node::new_obj(entries, 0, 6));

        assert_eq!(r, e);
        // It should consume the closing brace
        assert_eq!(toks_iter.next(), None);
    }

    #[test]
    fn trailing_sep_not_allowed() {
        let mut config = Config::DEFAULT;
        config.allow_trailing_commas = false;

        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_str("val", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];

        let r = object_consumer(&mut toks.iter().enumerate().peekable(), &config).unwrap_err();
        let e = TreebuilderErr::new_trailing_sep(4);

        assert_eq!(r, e);
    }

    #[test]
    fn missing_sep() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key1", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_str("val1", 0, 0),
            Token::new_str("key2", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_str("val2", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];

        let mut e_entries = HashMap::new();

        e_entries.insert("key".to_string(), Node::new_str("val", 3, 4));

        let r =
            object_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap_err();
        let e = TreebuilderErr::new_not_a_sep(4);

        assert_eq!(r, e);
    }

    #[test]
    fn empty() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];
        let r = object_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
        let e = Some(Node::new_obj(HashMap::new(), 0, 2));

        assert_eq!(r, e);
    }

    #[test]
    fn single_entry() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_str("val", 0, 0),
            Token::new_delimiter("}", 0, 0),
        ];

        let mut e_entries = HashMap::new();

        e_entries.insert("key".to_string(), Node::new_str("val", 3, 4));

        let r = object_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
        let e = Some(Node::new_obj(e_entries, 0, 5));

        assert_eq!(r, e);
    }

    #[test]
    fn multiple_entries() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_str("key_arr", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("]", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_kwd", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_kwd("false", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_num", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_num("123", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_obj", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_str", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_str("Hello, World!", 0, 0),
            Token::new_delimiter("}", 0, 0),
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

        let r = object_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
        let e = Some(Node::new_obj(e_entries, 0, 23));

        assert_eq!(r, e);
    }
}
