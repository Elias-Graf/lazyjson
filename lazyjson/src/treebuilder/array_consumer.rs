use std::iter::Peekable;

use crate::tokenizer::{Token, TokenIndices, TokenType};

use super::{error::TreebuilderErr, node::Node, value_consumer::value_consumer};

pub fn array_consumer(toks: &mut Peekable<TokenIndices>) -> Result<Option<Node>, TreebuilderErr> {
    let &(opn_i, opn_tok) = toks.peek().ok_or(TreebuilderErr::new_out_of_bounds())?;

    if !is_arr_opn(opn_tok) {
        return Ok(None);
    } else {
        toks.next();
    }

    let &(peek_i, peek_t) = toks
        .peek()
        .ok_or(TreebuilderErr::new_unterminated_arr(opn_i, opn_i + 1))?;

    if is_arr_cls(peek_t) {
        toks.next();

        return Ok(Some(Node::new_arr(Vec::new(), opn_i, peek_i + 1)));
    }

    let mut entries = Vec::new();

    loop {
        check_for_trailing_sep(toks, opn_i, &entries)?;

        let entry = value_consumer(toks)?.ok_or(TreebuilderErr::new_not_a_val(
            get_last_tok_idx(&entries) + 1,
        ))?;

        entries.push(entry);

        let (_, peek_t) = toks.next().ok_or(TreebuilderErr::new_unterminated_arr(
            opn_i,
            get_last_tok_idx(&entries),
        ))?;

        if is_arr_cls(peek_t) {
            let to = get_last_tok_idx(&entries) + 1;
            return Ok(Some(Node::new_arr(entries, opn_i, to)));
        } else if peek_t.typ != TokenType::Separator || peek_t.val != "," {
            return Err(TreebuilderErr::new_not_a_sep(get_last_tok_idx(&entries)));
        }
    }
}

fn is_arr_opn(opn_tok: &Token) -> bool {
    opn_tok.typ == TokenType::Separator && opn_tok.val == "["
}

fn is_arr_cls(t: &Token) -> bool {
    t.typ == TokenType::Separator && t.val == "]"
}

fn check_for_trailing_sep(
    toks: &mut Peekable<TokenIndices>,
    opn_i: usize,
    entries: &Vec<Node>,
) -> Result<bool, TreebuilderErr> {
    let &(cls_i, cls_t) = toks.peek().ok_or(TreebuilderErr::new_unterminated_arr(
        opn_i,
        get_last_tok_idx(&entries),
    ))?;

    if is_arr_cls(cls_t) {
        return Err(TreebuilderErr::new_trailing_sep(cls_i - 1));
    }

    return Ok(false);
}

fn get_last_tok_idx(entries: &Vec<Node>) -> usize {
    match entries.last() {
        None => 0,
        Some(e) => e.to,
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
    fn invalid_val() {
        let toks = [
            Token::sep("[", 0, 0),
            Token::num("1", 0, 0),
            Token::sep(",", 0, 0),
            Token::op(":", 0, 0),
            Token::sep("]", 0, 0),
        ];
        let r = array_consumer(&mut toks.iter().enumerate().peekable()).unwrap_err();
        let e = TreebuilderErr::new_not_a_val(3);

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
