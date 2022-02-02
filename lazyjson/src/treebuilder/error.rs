use crate::tokenizer::{Token, TokenType};
use std::{
    error::Error,
    fmt::{self, Debug},
};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TreebuilderErrTyp {
    NotAKey,
    NotASep,
    NotAVal,
    // TODO: rename to NotJsonAssignment
    NotAnAssignment,
    NotEqualAssignment,
    NotVariableName,
    OutOfBounds,
    TrailingSep,
    UndeclaredVariable,
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
    /// Creates a new error of the typ [`TreebuilderErrTyp::ExpectedVariableName`].
    pub fn new_not_var_name(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            from: i,
            to: i + 1,
            typ: TreebuilderErrTyp::NotVariableName,
        }
    }
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
    pub fn new_not_equals_assignment(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotEqualAssignment,
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
    /// Creates a new error of the typ [`TreebuilderErrTyp::UndeclaredVariable`].
    pub fn new_undeclared_variable(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UndeclaredVariable,
            from: i,
            to: i + i,
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
    /// Creates a new error of the typ [`TreebuilderErrTyp::UnterminatedArr`].
    pub fn new_unterminated_arr(from: usize, to: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnterminatedArr,
            from,
            to,
        }
    }
    /// Creates a new error of the typ [`TreebuilderErrTyp::UnterminatedObj`].
    pub fn new_unterminated_obj(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnterminatedObj,
            from: i,
            to: i + 1,
        }
    }

    pub fn msg(&self, toks: &[Token], inp: &str) -> String {
        let toks_of_err = &toks[self.from..self.to];
        let first_err_tok = toks_of_err.first().unwrap();
        let err_from = first_err_tok.from;
        let err_to = toks_of_err.last().unwrap().to;

        let verbal_hint = get_verbal_hint(self.typ, first_err_tok);
        let FilePosition { line, char } = get_err_pos_in_file(inp, err_from);
        let visual_hint = get_visual_hint(inp, err_from, err_to);

        format!(
            "{}, line: {}, char: {}\n\n{}\n",
            verbal_hint, line, char, visual_hint
        )
    }
}

fn get_verbal_hint(typ: TreebuilderErrTyp, err_tok: &Token) -> String {
    match typ {
        TreebuilderErrTyp::NotVariableName => "expected a variable name".to_string(),
        TreebuilderErrTyp::UnterminatedArr => "array was not terminated".to_string(),
        TreebuilderErrTyp::UnterminatedObj => "object was not terminated".to_string(),
        TreebuilderErrTyp::TrailingSep => {
            "expected the next value or close (trailing separator not allowed)".to_string()
        }
        TreebuilderErrTyp::UndeclaredVariable => {
            format!("undeclared variable with name: `{}`", err_tok.val).into()
        }
        TreebuilderErrTyp::NotAKey => format!(
            "expected a `{:?}` but received a `{:?}`",
            TokenType::StringLiteral,
            err_tok.typ,
        ),
        TreebuilderErrTyp::NotASep => format!("expected a `,` but received a `{:?}`", err_tok.typ),
        TreebuilderErrTyp::NotAVal => format!(
            "expected one of `[`, `{{`, `{:?}`, `{:?}`, or `{:?}` but received a `{:?}`",
            TokenType::KeywordLiteral,
            TokenType::NumberLiteral,
            TokenType::StringLiteral,
            err_tok.typ,
        ),
        TreebuilderErrTyp::NotAnAssignment => {
            format!("expected a `:` but received a `{:?}`", err_tok.typ)
        }
        TreebuilderErrTyp::NotEqualAssignment => "expected a assignment operator: '='".to_string(),
        TreebuilderErrTyp::OutOfBounds => {
            ">> INTERNAL ERROR - OUT OF BOUNDS << please submit a bug report with the JSON"
                .to_string()
        }
    }
}

#[derive(Debug)]
struct FilePosition {
    line: usize,
    char: usize,
}

fn get_err_pos_in_file(inp: &str, err_from: usize) -> FilePosition {
    let mut line_cnt = 1;
    let mut line_start = 0;
    for (i, c) in inp[..err_from].char_indices() {
        if c == '\n' {
            line_cnt += 1;
            line_start = i + 1;
        }
    }

    let char_cnt = err_from - line_start + 1;

    FilePosition {
        line: line_cnt,
        char: char_cnt,
    }
}

fn get_visual_hint(inp: &str, err_from: usize, err_to: usize) -> String {
    let err_len = err_to - err_from;
    let err = &inp[err_from..err_to];

    match err.contains('\n') {
        true => get_multi_line_visual_hint(err),
        false => get_single_line_visual_hint(inp, err_from, err_to, err_len),
    }
}

fn get_multi_line_visual_hint(err: &str) -> String {
    err.split('\n')
        .enumerate()
        .map(|(i, line)| {
            let marker = "^".repeat(line.len());
            let mut line_with_marker = String::new();

            if i > 0 {
                line_with_marker.push('\n');
            }

            line_with_marker.push_str(line);
            line_with_marker.push('\n');
            line_with_marker.push_str(&marker);

            line_with_marker
        })
        .collect::<String>()
}

fn get_single_line_visual_hint(
    inp: &str,
    err_from: usize,
    err_to: usize,
    err_len: usize,
) -> String {
    let (err_line_from, err_line_to) = get_err_line_bounds(inp, err_from, err_to);

    let offset_to_err = err_from - err_line_from;

    let line = &inp[err_line_from..err_line_to];
    let padding = " ".repeat(offset_to_err);
    let marker = "^".repeat(err_len);

    format!("{}\n{}{}", line, padding, marker)
}

fn get_err_line_bounds(inp: &str, err_from: usize, err_to: usize) -> (usize, usize) {
    let chars = inp.char_indices().collect::<Vec<(usize, char)>>();

    let mut up_to = chars[..err_to].to_owned();

    // Revert it so the first newline will be the first one before the error
    up_to.reverse();

    let mut line_start = 0;
    for (i, c) in up_to {
        if c == '\n' {
            line_start = i + 1;
            break;
        }
    }

    let following = &chars[err_from..];
    let line_end = following
        .iter()
        .find(|(_, c)| c == &'\n')
        .unwrap_or(&(inp.len(), '\n'))
        .0;

    (line_start, line_end)
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::TokenType;

    use super::*;

    #[test]
    fn not_a_key_msg() {
        let inp = "{false}";
        let toks = [
            Token::new_delimiter("{", 0, 1),
            Token::new_kwd("false", 1, 6),
            Token::new_delimiter("}", 6, 7),
        ];
        let msg = TreebuilderErr::new_not_a_key(1).msg(&toks, inp);

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
        let toks = [
            Token::new_delimiter("[", 0, 1),
            Token::new_num("0", 14, 15),
            Token::new_num("1", 28, 29),
            Token::new_delimiter("]", 38, 39),
        ];
        let msg = TreebuilderErr::new_not_a_sep(2).msg(&toks, inp);

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
        let toks = [
            Token::new_delimiter("{", 0, 1),
            Token::new_str("city", 14, 20),
            Token::new_json_assignment_op(20),
            Token::new_sep(",", 22, 23),
            Token::new_delimiter("}", 32, 33),
        ];

        assert_eq!(
            TreebuilderErr::new_not_a_val(3).msg(&toks, inp),
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
        let toks = [
            Token::new_delimiter("{", 0, 1),
            Token::new_str("city", 1, 7),
            Token::new_sep(",", 7, 8),
            Token::new_kwd("false", 9, 14),
            Token::new_delimiter("}", 14, 15),
        ];

        assert_eq!(
            TreebuilderErr::new_not_an_assignment(2).msg(&toks, inp),
            format!("expected a `:` but received a `{:?}`, line: 1, char: 8\n\n{{\"city\", false}}\n       ^\n", TokenType::Separator)
        );
    }

    #[test]
    fn trailing_sep_msg() {
        let inp = "{\n\"city\": false,}";
        let toks = [
            Token::new_delimiter("{", 0, 1),
            Token::new_str("city", 2, 8),
            Token::new_json_assignment_op(9),
            Token::new_kwd("false", 10, 15),
            Token::new_sep(",", 15, 16),
            Token::new_delimiter("}", 16, 17),
        ];

        assert_eq!(
            TreebuilderErr::new_trailing_sep(4).msg(&toks, inp),
            format!("expected the next value or close (trailing separator not allowed), line: 2, char: 14\n\n\"city\": false,}}\n             ^\n")
        );
    }

    #[test]
    fn unterminated_arr_msg() {
        let inp = "[false";
        let toks = [
            Token::new_delimiter("[", 0, 1),
            Token::new_kwd("false", 1, 6),
        ];

        assert_eq!(
            TreebuilderErr::new_unterminated_arr(0, 2).msg(&toks, inp),
            "array was not terminated, line: 1, char: 1\n\n[false\n^^^^^^\n"
        );
    }

    #[test]
    fn unterminated_obj_msg() {
        let inp_str = "{\n    \"city\": \"London\"\n";
        let toks = [
            Token::new_delimiter("{", 0, 1),
            Token::new_str("city", 6, 12),
            Token::new_json_assignment_op(12),
            Token::new_str("London", 14, 22),
        ];

        assert_eq!(
            TreebuilderErr::new_unterminated_obj(3).msg(&toks, inp_str),
            "object was not terminated, line: 2, char: 13\n\n    \"city\": \"London\"\n            ^^^^^^^^\n",
        )
    }
}
