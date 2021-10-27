use std::iter::Peekable;

use crate::tokenizer::{Token, TokenIndices, TokenType};

use super::config::Config;
use super::{error::TreebuilderErr, node::Node, value_consumer::value_consumer};

pub fn array_consumer(
    toks: &mut Peekable<TokenIndices>,
    config: &Config,
) -> Result<Option<Node>, TreebuilderErr> {
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
        let &(cls_i, cls_t) = toks.peek().ok_or(TreebuilderErr::new_unterminated_arr(
            opn_i,
            get_last_tok_idx(&entries),
        ))?;

        // If the array is now closed, we have a trailing separator
        if is_arr_cls(cls_t) {
            if config.allow_trailing_comma {
                toks.next();
                return Ok(Some(Node::new_arr(entries, opn_i, cls_i + 1)));
            } else {
                return Err(TreebuilderErr::new_trailing_sep(cls_i - 1));
            }
        }

        let entry = value_consumer(toks, &Config::DEFAULT)?.ok_or(
            TreebuilderErr::new_not_a_val(get_last_tok_idx(&entries) + 1),
        )?;

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
    opn_tok.typ == TokenType::Delimiter && opn_tok.val == "["
}

fn is_arr_cls(t: &Token) -> bool {
    t.typ == TokenType::Delimiter && t.val == "]"
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

    use crate::{tokenizer::Token, treebuilder::Config};

    use super::*;

    #[test]
    fn empty_input() {
        let toks = [];
        let r =
            array_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap_err();
        let e = TreebuilderErr::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_array() {
        let toks = [Token::new_num("0", 0, 0)];
        let toks_iter = &mut toks.iter().enumerate().peekable();
        let r = array_consumer(toks_iter, &Config::DEFAULT).unwrap();
        let e = None;

        assert_eq!(r, e);
        assert_eq!(toks_iter.next().unwrap(), (0, &Token::new_num("0", 0, 0)));
    }

    #[test]
    fn unterminated() {
        let toks = [Token::new_delimiter("[", 0, 0)];
        let r =
            array_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap_err();
        let e = TreebuilderErr::new_unterminated_arr(0, 1);

        assert_eq!(r, e);
    }

    #[test]
    fn missing_sep() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_num("1", 0, 0),
            Token::new_num("1", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let r =
            array_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap_err();
        let e = TreebuilderErr::new_not_a_sep(2);

        assert_eq!(r, e);
    }

    #[test]
    fn invalid_val() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_num("1", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_op(":", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let r =
            array_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap_err();
        let e = TreebuilderErr::new_not_a_val(3);

        assert_eq!(r, e);
    }

    #[test]
    fn trailing_sep_allowed() {
        let config = Config {
            allow_trailing_comma: true,
        };
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_num("123", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let toks_iter = &mut toks.iter().enumerate().peekable();

        let r = array_consumer(toks_iter, &config).unwrap();
        let e = Some(Node::new_arr(vec![Node::new_num("123", 1, 2)], 0, 4));

        assert_eq!(r, e);
        // The closing bracket should be consumed
        assert_eq!(toks_iter.next(), None);
    }

    #[test]
    fn trailing_sep_not_allowed() {
        let config = Config {
            allow_trailing_comma: false,
        };
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_num("123", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let r = array_consumer(&mut toks.iter().enumerate().peekable(), &config).unwrap_err();
        let e = TreebuilderErr::new_trailing_sep(2);

        assert_eq!(r, e);
    }

    #[test]
    fn empty() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let r = array_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
        let e = Some(Node::new_arr(Vec::new(), 0, 2));

        assert_eq!(r, e);
    }

    #[test]
    fn single_entry() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_num("123", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let r = array_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
        let e = Some(Node::new_arr(vec![Node::new_num("123", 1, 2)], 0, 3));

        assert_eq!(r, e);
    }

    #[test]
    fn multiple_entries() {
        let toks = [
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("[", 0, 0),
            Token::new_delimiter("]", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_kwd("false", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_num("123", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_delimiter("{", 0, 0),
            Token::new_delimiter("}", 0, 0),
            Token::new_sep(",", 0, 0),
            Token::new_str("Hello, World!", 0, 0),
            Token::new_delimiter("]", 0, 0),
        ];
        let r = array_consumer(&mut toks.iter().enumerate().peekable(), &Config::DEFAULT).unwrap();
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
