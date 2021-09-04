use crate::tokenizer::Token;
use std::{error::Error, fmt};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExpectedAssignment {
    unexp: Token,
}

impl fmt::Display for ExpectedAssignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expected assignment but received {}", self.unexp)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExpectedObjectKey {
    unexp: Token,
}

impl fmt::Display for ExpectedObjectKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expected object key but received {}", self.unexp)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExpectedSeparatorOrClose {
    unexp: Token,
}

impl fmt::Display for ExpectedSeparatorOrClose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "expected either a separator or the closing of the container but received {}",
            self.unexp
        )
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExpectedValueComposition {
    unexp: Token,
}

impl fmt::Display for ExpectedValueComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "expected the beginning of a value composition but received {}",
            self.unexp
        )
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct UnterminatedArray {}

impl fmt::Display for UnterminatedArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "array was not terminated")
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct UnterminatedObject {}

impl fmt::Display for UnterminatedObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "object was not terminated")
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct UnknownKeyword {
    tok: Token,
}

impl fmt::Display for UnknownKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown keyword '{}'", self.tok)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TreebuilderError {
    ExpectedAssignment(ExpectedAssignment),
    ExpectedObjectKey(ExpectedObjectKey),
    ExpectedSeparatorOrClose(ExpectedSeparatorOrClose),
    ExpectedValueComposition(ExpectedValueComposition),
    UnknownKeyword(UnknownKeyword),
    UnterminatedArray(UnterminatedArray),
    UnterminatedObject(UnterminatedObject),
}

impl Error for TreebuilderError {}

impl fmt::Display for TreebuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TreebuilderError::ExpectedAssignment(e) => e.fmt(f),
            TreebuilderError::ExpectedObjectKey(e) => e.fmt(f),
            TreebuilderError::ExpectedSeparatorOrClose(e) => e.fmt(f),
            TreebuilderError::ExpectedValueComposition(e) => e.fmt(f),
            TreebuilderError::UnknownKeyword(e) => e.fmt(f),
            TreebuilderError::UnterminatedArray(e) => e.fmt(f),
            TreebuilderError::UnterminatedObject(e) => e.fmt(f),
        }
    }
}

impl TreebuilderError {
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::ExpectedAssignment`]
    pub fn new_exp_assign(unexp: Token) -> TreebuilderError {
        TreebuilderError::ExpectedAssignment(ExpectedAssignment { unexp })
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::ExpectedObjectKey`]
    pub fn new_exp_obj_key(unexp: Token) -> TreebuilderError {
        TreebuilderError::ExpectedObjectKey(ExpectedObjectKey { unexp })
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::ExpectedSeparatorOrClose`]
    pub fn new_exp_sep_or_close(unexp: Token) -> TreebuilderError {
        TreebuilderError::ExpectedSeparatorOrClose(ExpectedSeparatorOrClose { unexp })
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::ExpectedValueComposition`]
    pub fn new_exp_val_comp(unexp: Token) -> TreebuilderError {
        TreebuilderError::ExpectedValueComposition(ExpectedValueComposition { unexp })
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::UnknownKeyword`]
    pub fn new_unknown_kwd(tok: Token) -> TreebuilderError {
        TreebuilderError::UnknownKeyword(UnknownKeyword { tok })
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::UnterminatedArray`]
    pub fn new_unterminated_arr() -> TreebuilderError {
        TreebuilderError::UnterminatedArray(UnterminatedArray {})
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::UnterminatedObject`]
    pub fn new_unterminated_obj() -> TreebuilderError {
        TreebuilderError::UnterminatedObject(UnterminatedObject {})
    }
}
