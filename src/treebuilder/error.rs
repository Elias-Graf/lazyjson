use std::fmt::{self};

use crate::tokenizer::Token;

use super::node::NodeType;

#[derive(Eq, PartialEq, Debug, Clone)]
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
pub struct UnterminatedContainer {
    node_typ: NodeType,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TreebuilderError {
    UnexpectedToken(UnexpectedToken),
    UnterminatedContainer(UnterminatedContainer),
}

impl TreebuilderError {
    pub fn new_unexp_tok(exp: Vec<Token>, rec: Token) -> TreebuilderError {
        TreebuilderError::UnexpectedToken(UnexpectedToken { exp, rec })
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
