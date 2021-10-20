use crate::tokenizer::{Token, TokenType};
use std::{
    error::Error,
    fmt::{self, Debug},
};

#[derive(PartialEq, Eq, Debug)]
pub enum TreebuilderErrTyp {
    NotAKey,
    /// TODO: separators are not just `,`, but also `{`, `}`, `[`, or`]`. This error was
    /// explicitly designed for `,`, so keep that in mind when
    /// using it.
    /// The best solution to the problem would probably be to go back to the
    /// tokenizer, and move container characters (such as `{` or `[`) from
    /// being separators, to be delimiters (would be a new type).
    NotASep,
    NotAVal,
    /// TODO: consider renaming to just `NotAnAssignment`. As far as I'm
    /// concerned, assignments are always operations and thus this is redundant.
    NotAnAssignmentOp,
    OutOfBounds,
    TrailingSep,
    UnknownKwd,
    UnterminatedArr,
    UnterminatedObj,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TreebuilderErr {
    pub typ: TreebuilderErrTyp,
    pub from: usize,
    pub to: usize,
}

impl fmt::Display for TreebuilderErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for TreebuilderErr {}

impl TreebuilderErr {
    pub fn new_not_a_key(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAKey,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_not_a_sep(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotASep,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_not_a_val(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAVal,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_not_an_assignment_op(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAnAssignmentOp,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_trailing_sep(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::TrailingSep,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_out_of_bounds() -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::OutOfBounds,
            from: usize::MAX,
            to: usize::MAX,
        }
    }
    pub fn new_unknown_kwd(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnknownKwd,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_unterminated_arr(from: usize, to: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnterminatedArr,
            from,
            to,
        }
    }
    pub fn new_unterminated_obj(from: usize, to: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnterminatedObj,
            from,
            to,
        }
    }

    pub fn msg(&self, toks: &[Token], inp: &str) -> String {
        let tok_from = toks.get(self.from).unwrap();
        let tok_to = toks.get(self.to - 1).unwrap();
        let len = tok_to.to - tok_from.from;

        let mut line_cnt = 0;
        let mut line_start = 0;

        for (i, c) in inp[..tok_from.from].char_indices() {
            if c == '\n' {
                line_cnt += 1;
                line_start = i + 1;
            }
        }

        let mut line_end = inp.len();

        for (i, c) in inp[tok_to.to..].char_indices() {
            if c == '\n' {
                line_end = tok_to.to + i;
                break;
            }
        }

        let char_cnt = tok_from.from - line_start;
        let line = &inp[line_start..line_end];
        let marker = " ".repeat(char_cnt) + &"^".repeat(len);

        if self.typ == TreebuilderErrTyp::UnterminatedArr {
            return format!(
                "array was not terminated, line: {}, char: {}\n\n{}\n{}\n",
                line_cnt + 1,
                char_cnt + 1,
                line,
                marker,
            );
        }

        if self.typ == TreebuilderErrTyp::UnterminatedObj {
            return format!(
                "object was not terminated, line: {}, char: {}\n\n{}\n{}\n",
                line_cnt + 1,
                char_cnt + 1,
                line,
                marker,
            );
        }

        if self.typ == TreebuilderErrTyp::TrailingSep {
            return format!(
                "expected the next value or close (trailing separator not allowed), line: {}, char: {}\n\n{}\n{}\n",
                line_cnt + 1,
                char_cnt + 1,
                line,
                marker,
            );
        }

        if self.typ == TreebuilderErrTyp::UnknownKwd {
            return format!(
                "received an unknown keyword `{}`, line: {}, char: {}\n\n{}\n{}\n",
                tok_from.val,
                line_cnt + 1,
                char_cnt + 1,
                line,
                marker,
            );
        }

        let exp = match self.typ {
            TreebuilderErrTyp::NotAKey => format!("a `{:?}`", TokenType::StringLiteral),
            TreebuilderErrTyp::NotASep => "a `,`".to_string(),
            TreebuilderErrTyp::NotAVal => format!(
                "one of `[`, `{{`, `{:?}`, `{:?}`, or `{:?}`",
                TokenType::KeywordLiteral,
                TokenType::NumberLiteral,
                TokenType::StringLiteral,
            ),
            TreebuilderErrTyp::NotAnAssignmentOp => "a `:`".to_string(),
            _ => unimplemented!(),
        };

        let msg = format!(
            "expected {} but received a `{:?}`, line: {}, char: {}\n\n{}\n{}\n",
            exp,
            tok_from.typ,
            line_cnt + 1,
            char_cnt + 1,
            line,
            marker,
        );

        print!("{}", msg);

        msg
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::TokenType;

    use super::*;

    #[test]
    fn not_a_key_msg() {
        let inp = "{false}";
        let tok = Token::kwd("false", 1, 6);
        let msg = TreebuilderErr::new_not_a_key(0).msg(&[tok], inp);

        assert_eq!(
            msg,
            format!(
                "expected a `{:?}` but received a `{:?}`, line: 1, char: 2\n\n{{false}}\n ^^^^^\n",
                TokenType::StringLiteral,
                TokenType::KeywordLiteral,
            )
        );
    }

    #[test]
    fn not_a_sep_msg() {
        let inp = "[
            0
            1
        ]";
        let tok = Token::num("1", 28, 29);
        let msg = TreebuilderErr::new_not_a_sep(0).msg(&[tok], inp);

        assert_eq!(
            msg,
            format!(
                "expected a `,` but received a `{:?}`, line: 3, char: 13\n\n            1\n            ^\n",
                TokenType::NumberLiteral,
            ),
        );
    }

    #[test]
    fn not_a_val_msg() {
        let inp = "{
            \"city\": ,
        }";
        let tok = Token::sep(",", 22, 23);
        let msg = TreebuilderErr::new_not_a_val(0).msg(&[tok], inp);

        assert_eq!(
            msg,
            format!(
                "expected one of `[`, `{{`, `{:?}`, `{:?}`, or `{:?}` but received a `{:?}`, line: 2, char: 21\n\n            \"city\": ,\n                    ^\n",
                TokenType::KeywordLiteral,
                TokenType::NumberLiteral,
                TokenType::StringLiteral,
                TokenType::Separator,
            ),
        );
    }

    #[test]
    fn not_an_assignment_msg() {
        let inp = "{\"city\", false}";
        let tok = Token::sep(",", 7, 8);
        let msg = TreebuilderErr::new_not_an_assignment_op(0).msg(&[tok], inp);

        assert_eq!(
            msg,
            format!("expected a `:` but received a `{:?}`, line: 1, char: 8\n\n{{\"city\", false}}\n       ^\n", TokenType::Separator)
        );
    }

    #[test]
    fn trailing_sep_msg() {
        let inp = "{\"city\": false,}";
        let tok = Token::sep(",", 14, 15);
        let msg = TreebuilderErr::new_trailing_sep(0).msg(&[tok], inp);

        assert_eq!(
            msg,
            format!("expected the next value or close (trailing separator not allowed), line: 1, char: 15\n\n{{\"city\": false,}}\n              ^\n")
        );
    }

    #[test]
    fn unknown_kwd_msg() {
        let inp = "nil";
        let tok = Token::kwd("nil", 0, 3);
        let msg = TreebuilderErr::new_unknown_kwd(0).msg(&[tok], inp);

        assert_eq!(
            msg,
            format!("received an unknown keyword `nil`, line: 1, char: 1\n\nnil\n^^^\n")
        );
    }

    #[test]
    fn unterminated_arr_msg() {
        let inp = "[false";
        let toks = &[Token::sep("[", 0, 1), Token::kwd("false", 1, 6)];
        let msg = TreebuilderErr::new_unterminated_arr(0, 2).msg(toks, inp);

        assert_eq!(
            msg,
            "array was not terminated, line: 1, char: 1\n\n[false\n^^^^^^\n"
        );
    }

    #[test]
    fn unterminated_obj_msg() {
        let inp = "{\"city\": \"London\"";
        let toks = &[
            Token::sep("{", 0, 1),
            Token::str("city", 1, 7),
            Token::op(":", 7, 8),
            Token::str("London", 9, 17),
        ];
        let msg = TreebuilderErr::new_unterminated_obj(0, 4).msg(toks, inp);

        assert_eq!(
            msg,
            "object was not terminated, line: 1, char: 1\n\n{\"city\": \"London\"\n^^^^^^^^^^^^^^^^^\n",
        )
    }
}
