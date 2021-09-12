use core::fmt;
use std::error::Error;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MultipleDecimalPoints {
    msg: String,
}

impl fmt::Display for MultipleDecimalPoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NoInput {}

impl fmt::Display for NoInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tokenizer did not receive any input")
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OutOfBounds {}

impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tried to read the next character but there was none")
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UnhandledCharacter {
    msg: String,
}

impl fmt::Display for UnhandledCharacter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UnterminatedString {
    msg: String,
}

impl fmt::Display for UnterminatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenizationError {
    MultipleDecimalPoints(MultipleDecimalPoints),
    NoInput(NoInput),
    OutOfBounds(OutOfBounds),
    UnhandledCharacter(UnhandledCharacter),
    UnterminatedString(UnterminatedString),
}

impl Error for TokenizationError {}

impl fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenizationError::MultipleDecimalPoints(e) => e.fmt(f),
            TokenizationError::NoInput(e) => e.fmt(f),
            TokenizationError::OutOfBounds(o) => o.fmt(f),
            TokenizationError::UnhandledCharacter(e) => e.fmt(f),
            TokenizationError::UnterminatedString(e) => e.fmt(f),
        }
    }
}

impl TokenizationError {
    /// Creates a new tokenization error of type
    /// [`TokenizationError::MultipleDecimalPoints`].
    pub fn new_multiple_decimal_points(inp: &str, idx: usize) -> TokenizationError {
        TokenizationError::MultipleDecimalPoints(MultipleDecimalPoints {
            msg: format!(
                "multiple decimal points at {} ('{}')",
                idx,
                inp.chars().nth(idx).unwrap(),
            ),
        })
    }
    /// Creates a new tokenization error of type
    /// [`TokenizationError::NoInput`].
    pub fn new_no_input() -> TokenizationError {
        TokenizationError::NoInput(NoInput {})
    }
    /// Creates a new tokenization error of type
    /// [`TokenizationError::OutOfBounds`].
    pub fn new_out_of_bounds() -> TokenizationError {
        TokenizationError::OutOfBounds(OutOfBounds {})
    }
    /// Creates a new tokenization error of type
    /// [`TokenizationError::UnhandledCharacter`].
    pub fn new_unhandled_character(inp: &str, idx: usize) -> TokenizationError {
        TokenizationError::UnhandledCharacter(UnhandledCharacter {
            msg: format!(
                "unhandled character at {} ('{}')",
                idx,
                inp.chars().nth(idx).unwrap(),
            ),
        })
    }
    /// Creates a new tokenization error of type
    /// [`TokenizationError::UnterminatedString`].
    pub fn new_unterminated_string(inp: &str, idx: usize) -> TokenizationError {
        let prev_idx = idx - 1;

        TokenizationError::UnterminatedString(UnterminatedString {
            msg: format!(
                "unterminated string after {} ('{}')",
                prev_idx,
                inp.chars().nth(prev_idx).unwrap(),
            ),
        })
    }
}
