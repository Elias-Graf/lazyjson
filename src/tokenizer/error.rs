use std::fmt;

#[derive(Debug, Clone)]
pub struct TokenizationError {
  pub msg: String,
}

impl fmt::Display for TokenizationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}