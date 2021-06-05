use std::fmt::{self};

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    MultipleDecimalPoints,
    UnterminatedString,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct TokenizationError {
    pub kind: ErrorKind,
}

impl TokenizationError {
    pub fn new(kind: ErrorKind) -> TokenizationError {
        TokenizationError { kind }
    }
}

impl fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tokenization error \"{}\"", self.kind)
    }
}
