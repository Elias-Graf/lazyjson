use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    WhitespaceLiteral,
    KeywordLiteral,
    NumberLiteral,
    Operator,
    Separator,
    StringLiteral,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token {
    pub to: usize,
    pub from: usize,
    pub typ: TokenType,
    pub val: String,
}

// TODO: prefix the all method that create tokens with "new".
impl Token {
    pub fn new_whitespace(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::WhitespaceLiteral,
            val: val.into(),
        }
    }

    /// Create a token of type [`TokenType::KeywordLiteral`].
    pub fn kwd(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::KeywordLiteral,
            val: val.into(),
        }
    }
    /// Create a token of type [`TokenType::NumberLiteral`].
    pub fn num(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::NumberLiteral,
            val: val.into(),
        }
    }
    /// Create a token of type [`TokenType::Operator`].
    pub fn op(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::Operator,
            val: val.into(),
        }
    }
    /// Create a token of type [`TokenType::Separator`].
    pub fn sep(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::Separator,
            val: val.into(),
        }
    }
    /// Create a token of type [`TokenType::StringLiteral`].
    pub fn str(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::StringLiteral,
            val: val.into(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token(typ: {:?}, val: \"{}\")", self.typ, self.val)
    }
}
