use crate::tokenizer::Token;
use std::{error::Error, fmt};

#[derive(PartialEq, Eq, Debug)]
pub enum TreebuilderErrTyp {
    NotAKey,
    NotASep,
    NotAVal,
    NotAnAssignmentOp,
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

impl TreebuilderErr {
    pub fn new_not_a_key(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAKey,
            from: i,
            to: i + i,
        }
    }
    pub fn new_not_a_sep(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotASep,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_not_val(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAVal,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_not_an_assignment_op(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAnAssignmentOp,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_trailing_sep(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::TrailingSep,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_out_of_bounds() -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::OutOfBounds,
            from: usize::MAX,
            to: usize::MAX,
        }
    }
    pub fn new_unknown_kwd(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnknownKwd,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_unterminated_arr(from: usize, to: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnterminatedArr,
            from,
            to,
        }
    }
    pub fn new_unterminated_obj(from: usize, to: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnterminatedObj,
            from,
            to,
        }
    }

    pub fn gen_msg(&self, toks: &[Token]) -> String {
        match self.typ {
            TreebuilderErrTyp::NotAKey => {
                format!("expected a key but found {:?}", &toks[self.from..self.to])
            }
            TreebuilderErrTyp::NotASep => format!(
                "expected a separator but found {:?}",
                &toks[self.from..self.to]
            ),
            TreebuilderErrTyp::NotAVal => format!(
                "expected a valid value[composition] but found {:?}",
                &toks[self.from..self.to]
            ),
            TreebuilderErrTyp::NotAnAssignmentOp => format!(
                "expected the assignment operator ':' but received {:?}",
                &toks[self.from..self.to]
            ),
            TreebuilderErrTyp::TrailingSep => {
                format!("found trailing separator which is not allowed")
            }
            TreebuilderErrTyp::OutOfBounds => {
                format!("tried to consume a none value - reached the end of the tokens")
            }
            TreebuilderErrTyp::UnknownKwd => {
                format!("unknown keyword {:?}", &toks[self.from..self.to])
            }
            TreebuilderErrTyp::UnterminatedArr => {
                format!("unterminated array {:?}", &toks[self.from..self.to])
            }
            TreebuilderErrTyp::UnterminatedObj => {
                format!("unterminated object {:?}", &toks[self.from..self.to])
            }
        }
    }
}

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
pub enum OldTreebuilderError {
    ExpectedAssignment(ExpectedAssignment),
    ExpectedObjectKey(ExpectedObjectKey),
    ExpectedSeparatorOrClose(ExpectedSeparatorOrClose),
    ExpectedValueComposition(ExpectedValueComposition),
    UnknownKeyword(UnknownKeyword),
    UnterminatedArray(UnterminatedArray),
    UnterminatedObject(UnterminatedObject),
}

impl Error for OldTreebuilderError {}

impl fmt::Display for OldTreebuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OldTreebuilderError::ExpectedAssignment(e) => e.fmt(f),
            OldTreebuilderError::ExpectedObjectKey(e) => e.fmt(f),
            OldTreebuilderError::ExpectedSeparatorOrClose(e) => e.fmt(f),
            OldTreebuilderError::ExpectedValueComposition(e) => e.fmt(f),
            OldTreebuilderError::UnknownKeyword(e) => e.fmt(f),
            OldTreebuilderError::UnterminatedArray(e) => e.fmt(f),
            OldTreebuilderError::UnterminatedObject(e) => e.fmt(f),
        }
    }
}

impl OldTreebuilderError {
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::ExpectedAssignment`]
    pub fn new_exp_assign(unexp: Token) -> OldTreebuilderError {
        OldTreebuilderError::ExpectedAssignment(ExpectedAssignment { unexp })
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::ExpectedObjectKey`]
    pub fn new_exp_obj_key(unexp: Token) -> OldTreebuilderError {
        OldTreebuilderError::ExpectedObjectKey(ExpectedObjectKey { unexp })
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::ExpectedSeparatorOrClose`]
    pub fn new_exp_sep_or_close(unexp: Token) -> OldTreebuilderError {
        OldTreebuilderError::ExpectedSeparatorOrClose(ExpectedSeparatorOrClose { unexp })
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::ExpectedValueComposition`]
    pub fn new_exp_val_comp(unexp: Token) -> OldTreebuilderError {
        OldTreebuilderError::ExpectedValueComposition(ExpectedValueComposition { unexp })
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::UnknownKeyword`]
    pub fn new_unknown_kwd(tok: Token) -> OldTreebuilderError {
        OldTreebuilderError::UnknownKeyword(UnknownKeyword { tok })
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::UnterminatedArray`]
    pub fn new_unterminated_arr() -> OldTreebuilderError {
        OldTreebuilderError::UnterminatedArray(UnterminatedArray {})
    }
    /// Creates a new treebuilder error of type
    /// [`TreebuilderError::UnterminatedObject`]
    pub fn new_unterminated_obj() -> OldTreebuilderError {
        OldTreebuilderError::UnterminatedObject(UnterminatedObject {})
    }
}
