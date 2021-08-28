use crate::tokenizer::Token;
use std::{error::Error, fmt};

use super::node::NodeType;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExpectedAssignment {
    unexp: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExpectedObjectKey {
    unexp: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExpectedSeparatorOrClose {
    unexp: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExpectedValueComposition {
    unexp: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct UnterminatedContainer {
    node_typ: NodeType,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct UnknownKeyword {
    tok: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TreebuilderError {
    ExpectedAssignment(ExpectedAssignment),
    ExpectedObjectKey(ExpectedObjectKey),
    ExpectedSeparatorOrClose(ExpectedSeparatorOrClose),
    ExpectedValueComposition(ExpectedValueComposition),
    UnknownKeyword(UnknownKeyword),
    UnterminatedContainer(UnterminatedContainer),
}

impl Error for TreebuilderError {}

impl fmt::Display for TreebuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<DISPLAY NOT IMPLEMENTED>")
    }
}

impl TreebuilderError {
    pub fn new_exp_assign(unexp: Token) -> TreebuilderError {
        TreebuilderError::ExpectedAssignment(ExpectedAssignment { unexp })
    }

    pub fn new_exp_obj_key(unexp: Token) -> TreebuilderError {
        TreebuilderError::ExpectedObjectKey(ExpectedObjectKey { unexp })
    }

    pub fn new_exp_sep_or_close(unexp: Token) -> TreebuilderError {
        TreebuilderError::ExpectedSeparatorOrClose(ExpectedSeparatorOrClose { unexp })
    }

    pub fn new_exp_val_comp(unexp: Token) -> TreebuilderError {
        TreebuilderError::ExpectedValueComposition(ExpectedValueComposition { unexp })
    }

    pub fn new_unknown_kwd(tok: Token) -> TreebuilderError {
        TreebuilderError::UnknownKeyword(UnknownKeyword { tok })
    }

    pub fn new_unterminated_cont(node_typ: NodeType) -> TreebuilderError {
        TreebuilderError::UnterminatedContainer(UnterminatedContainer { node_typ })
    }
}
