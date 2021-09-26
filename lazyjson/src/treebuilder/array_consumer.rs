use std::iter::Peekable;

use crate::tokenizer::{TokenIndices, TokenType};

use super::{error::TreebuilderErr, node::Node, value_consumer::value_consumer};

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

    use crate::tokenizer::Token;

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
