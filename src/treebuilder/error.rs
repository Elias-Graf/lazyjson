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

impl UnterminatedContainer {
    pub fn new(node_typ: NodeType) -> UnterminatedContainer {
        UnterminatedContainer { node_typ }
    }
}
