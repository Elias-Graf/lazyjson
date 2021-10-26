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
    NotAnAssignment,
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
    /// Creates a new error of the typ [`TreebuilderErrTyp::NotAKey`].
    pub fn new_not_a_key(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAKey,
            from: i,
            to: i + 1,
        }
    }
    /// Creates a new error of the typ [`TreebuilderErrTyp::NotASep`].
    pub fn new_not_a_sep(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotASep,
            from: i,
            to: i + 1,
        }
    }
    /// Creates a new error of the typ [`TreebuilderErrTyp::NotAVal`].
    pub fn new_not_a_val(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAVal,
            from: i,
            to: i + 1,
        }
    }
    /// Creates a new error of the typ [`TreebuilderErrTyp::NotAnAssignment`].
    pub fn new_not_an_assignment(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAnAssignment,
            from: i,
            to: i + 1,
        }
    }
    /// Creates a new error of the typ [`TreebuilderErrTyp::TrailingSep`].
    pub fn new_trailing_sep(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::TrailingSep,
            from: i,
            to: i + 1,
        }
    }
    /// Creates a new error of the typ [`TreebuilderErrTyp::OutOfBounds`].
    pub fn new_out_of_bounds() -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::OutOfBounds,
            from: usize::MAX,
            to: usize::MAX,
        }
    }
    /// Creates a new error of the typ [`TreebuilderErrTyp::UnknownKwd`].
    pub fn new_unknown_kwd(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnknownKwd,
            from: i,
            to: i + 1,
        }
    }
    /// Creates a new error of the typ [`TreebuilderErrTyp::UnterminatedArr`].
    pub fn new_unterminated_arr(from: usize, to: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnterminatedArr,
            from,
            to,
        }
    }
    /// Creates a new error of the typ [`TreebuilderErrTyp::UnterminatedObj`].
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

        let err_len = tok_to.to - tok_from.from;

        let (line_cnt, idx_of_line_start, idx_of_line_end) = get_err_pos(inp, tok_from, tok_to);
        let err_start_on_line = tok_from.from - idx_of_line_start;
        let pos_info = fmt_pos_info(line_cnt, err_start_on_line);
        let line_info = fmt_line_info(
            inp,
            idx_of_line_start,
            idx_of_line_end,
            err_start_on_line,
            err_len,
        );
        let err_specific_msg = self.get_err_specific_msg(toks);

        format!("{}, {}\n\n{}\n", err_specific_msg, pos_info, line_info)
    }

    fn get_err_specific_msg(&self, toks: &[Token]) -> String {
        let tok = toks.get(self.from).unwrap();

        match self.typ {
            TreebuilderErrTyp::UnterminatedArr => "array was not terminated".to_string(),
            TreebuilderErrTyp::UnterminatedObj => "object was not terminated".to_string(),
            TreebuilderErrTyp::TrailingSep => {
                "expected the next value or close (trailing separator not allowed)".to_string()
            }
            TreebuilderErrTyp::UnknownKwd => format!("received an unknown keyword `{}`", tok.val,),
            TreebuilderErrTyp::NotAKey => format!(
                "expected a `{:?}` but received a `{:?}`",
                TokenType::StringLiteral,
                tok.typ,
            ),
            TreebuilderErrTyp::NotASep => format!("expected a `,` but received a `{:?}`", tok.typ),
            TreebuilderErrTyp::NotAVal => format!(
                "expected one of `[`, `{{`, `{:?}`, `{:?}`, or `{:?}` but received a `{:?}`",
                TokenType::KeywordLiteral,
                TokenType::NumberLiteral,
                TokenType::StringLiteral,
                tok.typ,
            ),
            TreebuilderErrTyp::NotAnAssignment => {
                format!("expected a `:` but received a `{:?}`", tok.typ)
            }
            TreebuilderErrTyp::OutOfBounds => {
                ">> INTERNAL ERROR - OUT OF BOUNDS << please submit a bug report with the JSON"
                    .to_string()
            }
        }
    }
}

fn get_err_pos(inp: &str, tok_from: &Token, tok_to: &Token) -> (usize, usize, usize) {
    let mut line_cnt: usize = 0;
    let mut idx_of_line_start: usize = 0;
    let mut idx_of_line_end: usize = inp.len();

    for (i, c) in inp[..tok_from.from].char_indices() {
        if c == '\n' {
            line_cnt += 1;
            idx_of_line_start = i + 1;
        }
    }

    for (i, c) in inp[tok_to.to..].char_indices() {
        if c == '\n' {
            idx_of_line_end = tok_to.to + i;
            break;
        }
    }

    (line_cnt, idx_of_line_start, idx_of_line_end)
}

fn fmt_pos_info(line: usize, char: usize) -> String {
    format!("line: {}, char: {}", line + 1, char + 1)
}

fn fmt_line_info(
    inp: &str,
    line_start: usize,
    line_end: usize,
    err_start: usize,
    err_len: usize,
) -> String {
    let line = &inp[line_start..line_end];
    let err_marker = " ".repeat(err_start) + &"^".repeat(err_len);

    format!("{}\n{}", line, err_marker)
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::TokenType;

    use super::*;

    #[test]
    fn not_a_key_msg() {
        let inp = "{false}";
        let tok = Token::new_kwd("false", 1, 6);
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
        let tok = Token::new_num("1", 28, 29);
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
        let tok = Token::new_sep(",", 22, 23);
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
        let tok = Token::new_sep(",", 7, 8);
        let msg = TreebuilderErr::new_not_an_assignment(0).msg(&[tok], inp);

        assert_eq!(
            msg,
            format!("expected a `:` but received a `{:?}`, line: 1, char: 8\n\n{{\"city\", false}}\n       ^\n", TokenType::Separator)
        );
    }

    #[test]
    fn trailing_sep_msg() {
        let inp = "{\"city\": false,}";
        let tok = Token::new_sep(",", 14, 15);
        let msg = TreebuilderErr::new_trailing_sep(0).msg(&[tok], inp);

        assert_eq!(
            msg,
            format!("expected the next value or close (trailing separator not allowed), line: 1, char: 15\n\n{{\"city\": false,}}\n              ^\n")
        );
    }

    #[test]
    fn unknown_kwd_msg() {
        let inp = "nil";
        let tok = Token::new_kwd("nil", 0, 3);
        let msg = TreebuilderErr::new_unknown_kwd(0).msg(&[tok], inp);

        assert_eq!(
            msg,
            format!("received an unknown keyword `nil`, line: 1, char: 1\n\nnil\n^^^\n")
        );
    }

    #[test]
    fn unterminated_arr_msg() {
        let inp = "[false";
        let toks = &[Token::new_sep("[", 0, 1), Token::new_kwd("false", 1, 6)];
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
            Token::new_sep("{", 0, 1),
            Token::new_str("city", 1, 7),
            Token::new_op(":", 7, 8),
            Token::new_str("London", 9, 17),
        ];
        let msg = TreebuilderErr::new_unterminated_obj(0, 4).msg(toks, inp);

        assert_eq!(
            msg,
            "object was not terminated, line: 1, char: 1\n\n{\"city\": \"London\"\n^^^^^^^^^^^^^^^^^\n",
        )
    }
}
