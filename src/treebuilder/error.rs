use std::fmt::{self};

use crate::tokenizer::Token;

use super::node::NodeType;

#[deprecated]
#[derive(Eq, PartialEq, Debug, Clone)]
/// This error does not describe the origin/causation very well, thus it should
/// be considered using a different error, or creating a new one.
pub struct UnexpectedToken {
    // TODO: also accept token types (if possible) as, for example, if the value
    // must be a string, it doesn't matter what the actual value is.
    exp: Vec<Token>,
    rec: Token,
}

impl UnexpectedToken {
    pub fn new(rec: Token, exp: Vec<Token>) -> UnexpectedToken {
        UnexpectedToken { exp, rec }
    }
}

impl fmt::Display for UnexpectedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let UnexpectedToken { exp, rec } = self;

        if exp.len() == 1 {
            write!(f, "unexpected token: {:?}, expected: {:?}", rec, exp[0])
        } else {
            write!(f, "unexpected token: {:?}, expected one of {:?}", rec, exp)
        }
    }
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
    ExpectedSeparatorOrClose(ExpectedSeparatorOrClose),
    ExpectedValueComposition(ExpectedValueComposition),
    #[deprecated]
    /// See [`UnexpectedToken`] for more information.
    UnexpectedToken(UnexpectedToken),
    UnknownKeyword(UnknownKeyword),
    UnterminatedContainer(UnterminatedContainer),
}

impl TreebuilderError {
    pub fn new_exp_sep_or_close(unexp: Token) -> TreebuilderError {
        TreebuilderError::ExpectedSeparatorOrClose(ExpectedSeparatorOrClose { unexp })
    }

    pub fn new_exp_val_comp(unexp: Token) -> TreebuilderError {
        TreebuilderError::ExpectedValueComposition(ExpectedValueComposition { unexp })
    }

    #[deprecated]
    /// See [`UnexpectedToken`] for more information.
    pub fn new_unexp_tok(exp: Vec<Token>, rec: Token) -> TreebuilderError {
        TreebuilderError::UnexpectedToken(UnexpectedToken { exp, rec })
    }

    pub fn new_unknown_kwd(tok: Token) -> TreebuilderError {
        TreebuilderError::UnknownKeyword(UnknownKeyword { tok })
    }

    pub fn new_unterminated_cont(node_typ: NodeType) -> TreebuilderError {
        TreebuilderError::UnterminatedContainer(UnterminatedContainer { node_typ })
    }
}

impl UnterminatedContainer {
    #[deprecated(note = "Use [`TreebuilderError::new_unterminated_container`] instead")]
    /// Please use [`TreebuilderError::new_unterminated_container()`] instead.
    pub fn new(node_typ: NodeType) -> UnterminatedContainer {
        UnterminatedContainer { node_typ }
    }
}
