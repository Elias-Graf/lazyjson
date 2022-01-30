use super::{config::Config, error::TreebuilderErr, node::Node, value_consumer::value_consumer};
use crate::tokenizer::{Token, TokenIndices, TokenType};
use std::{collections::HashMap, iter::Peekable};

pub fn object_consumer(
    inp: &mut Peekable<TokenIndices>,
    config: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
    let (opn_i, _) = match consume_obj_opn(inp) {
        None => return Ok(None),
        Some(o) => o,
    };

    let mut entries = HashMap::new();

    // Check if the object is immediately closed again.
    if let Some((cls_i, _)) = consume_obj_cls(inp, opn_i)? {
        return Ok(Some(Node::new_obj(entries, opn_i, cls_i + 1)));
    }

    loop {
        let (key_i, key) = consume_key(inp)?;

        consume_assignment(inp, key_i)?;

        let val = match value_consumer(inp, &Config::DEFAULT)? {
            None => return Err(TreebuilderErr::new_not_a_val(inp.next().unwrap().0)),
            Some(v) => v,
        };
        entries.insert(key, val);

        if let Some((cls_i, _)) = consume_obj_cls(inp, opn_i)? {
            return Ok(Some(Node::new_obj(entries, opn_i, cls_i + 1)));
        }

        consume_val_sep(inp)?;

        // Check if the next token is an object close, if yes, we have a trailing
        // separator.
        if let Some((cls_i, _)) = consume_obj_cls(inp, opn_i)? {
            if config.allow_trailing_commas {
                return Ok(Some(Node::new_obj(entries, opn_i, cls_i + 1)));
            }

            return Err(TreebuilderErr::new_trailing_sep(cls_i - 1));
        }
    }
}

/// Returns the token if a object open delimiter was found.
fn consume_obj_opn<'a>(inp: &'a mut Peekable<TokenIndices>) -> Option<(usize, &'a Token)> {
    let &(_, t) = inp.peek().unwrap();

    if t.typ == TokenType::Delimiter && t.val == "{" {
        return inp.next();
    }

    None
}

fn consume_obj_cls<'a>(
    inp: &'a mut Peekable<TokenIndices>,
    opn_i: usize,
) -> Result<Option<(usize, &'a Token)>, TreebuilderErr> {
    let &(_, t) = inp
        .peek()
        .ok_or(TreebuilderErr::new_unterminated_obj(opn_i))?;

    if t.typ == TokenType::Delimiter && t.val == "}" {
        return Ok(inp.next());
    }

    Ok(None)
}

fn consume_key<'a>(inp: &'a mut Peekable<TokenIndices>) -> Result<(usize, String), TreebuilderErr> {
    let &(i, t) = inp.peek().unwrap();

    if t.typ == TokenType::StringLiteral {
        inp.next();

        return Ok((i, t.val.clone()));
    }

    Err(TreebuilderErr::new_not_a_key(i))
}

fn consume_assignment(
    inp: &mut Peekable<TokenIndices>,
    key_i: usize,
) -> Result<(), TreebuilderErr> {
    let (i, t) = inp
        .next()
        .ok_or(TreebuilderErr::new_unterminated_obj(key_i))?;

    if t.typ != TokenType::JsonAssignmentOperator {
        return Err(TreebuilderErr::new_not_an_assignment(i));
    }

    Ok(())
}

fn consume_val_sep<'a>(inp: &'a mut Peekable<TokenIndices>) -> Result<(), TreebuilderErr> {
    let &(i, t) = inp.peek().unwrap();

    if t.typ != TokenType::Separator || t.val != "," {
        return Err(TreebuilderErr::new_not_a_sep(i));
    }

    inp.next();

    return Ok(());
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Token;

    use super::*;

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
        let e = TreebuilderErr::new_unterminated_obj(0);

        assert_eq!(r, e);
    }

    #[test]
    fn invalid_key() {
        let toks = [
            Token::new_delimiter("{", 0, 0),
            Token::new_kwd("false", 0, 0),
            Token::new_json_assignment_op(0),
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
            Token::new_json_assignment_op(0),
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
            Token::new_json_assignment_op(0),
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
            Token::new_json_assignment_op(0),
            Token::new_str("val1", 0, 0),
            Token::new_str("key2", 0, 0),
            Token::new_json_assignment_op(0),
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
            Token::new_json_assignment_op(0),
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
            Token::new_json_assignment_op(0),
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("]", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_kwd", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_kwd("false", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_num", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_num("123", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_obj", 0, 0),
            Token::new_json_assignment_op(0),
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("key_str", 0, 0),
            Token::new_json_assignment_op(0),
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
