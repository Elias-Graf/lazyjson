use std::fmt;

#[derive(Debug, Clone)]
pub struct TokenizationError;

impl fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "an error occurred during tokenization, no further information"
        )
    }
}
