use std::fmt;

// TODO: consider removing literal from the types, as it should be clear that
// they are literals.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Delimiter,
    KeywordLiteral,
    LineComment,
    NumberLiteral,
    Operator,
    Separator,
    StringLiteral,
    WhitespaceLiteral,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token {
    pub from: usize,
    pub to: usize,
    pub typ: TokenType,
    pub val: String,
}

impl Token {
    /// Create a new token of the type [`TokenType::Delimiter`].
    pub fn new_delimiter(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::Delimiter,
            val: val.to_string(),
        }
    }
    /// Create a new token of the type [`TokenType::KeywordLiteral`].
    pub fn new_kwd(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::KeywordLiteral,
            val: val.into(),
        }
    }
    /// Create a new token of the type [`TokenType::LineComment`].
    pub fn new_line_comment(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::LineComment,
            val: val.into(),
        }
    }
    /// Create a new token of the type [`TokenType::NumberLiteral`].
    pub fn new_num(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::NumberLiteral,
            val: val.into(),
        }
    }
    /// Create a new token of the type [`TokenType::Operator`].
    pub fn new_op(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::Operator,
            val: val.into(),
        }
    }
    /// Create a new token of the type [`TokenType::Separator`].
    pub fn new_sep(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::Separator,
            val: val.into(),
        }
    }
    /// Create a new token of the type [`TokenType::StringLiteral`].
    pub fn new_str(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::StringLiteral,
            val: val.into(),
        }
    }
    /// Create a new token of the type [`TokenType::WhitespaceLiteral`].
    pub fn new_whitespace(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::WhitespaceLiteral,
            val: val.into(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token(typ: {:?}, val: \"{}\")", self.typ, self.val)
    }
}
