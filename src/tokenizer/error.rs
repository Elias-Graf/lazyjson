use std::fmt;

pub trait TokenizationError {
    fn kind(&self) -> String;
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tokenization error of kind \"{}\" occurred", self.kind())
    }
}
