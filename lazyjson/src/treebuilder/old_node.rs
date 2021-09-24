use core::fmt;
use std::{collections::HashMap, fmt::Debug};

use crate::tokenizer::Token;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct OldArrayNode {
    pub entries: Vec<OldNode>,
    toks: Vec<Token>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct OldBoolNode {
    pub val: bool,
    tok: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct OldNullNode {
    tok: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct OldNumberNode {
    pub val: String,
    tok: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct OldObjectNode {
    pub entries: HashMap<String, OldNode>,
    toks: Vec<Token>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct OldStringNode {
    pub val: String,
    tok: Token,
}

impl fmt::Display for OldStringNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum OldNode {
    Array(OldArrayNode),
    Bool(OldBoolNode),
    Null(OldNullNode),
    Number(OldNumberNode),
    Object(OldObjectNode),
    String(OldStringNode),
}

impl OldNode {
    /// Create a new Node of type [`Node::Array`].
    pub fn new_arr(entries: Vec<OldNode>, toks: Vec<Token>) -> OldNode {
        OldNode::Array(OldArrayNode { entries, toks })
    }
    /// Create a new Node of type [`Node::Bool`].
    pub fn new_bool(val: bool, tok: Token) -> OldNode {
        OldNode::Bool(OldBoolNode { val, tok })
    }
    /// Create a new Node of type [`Node::Null`].
    pub fn new_null(tok: Token) -> OldNode {
        OldNode::Null(OldNullNode { tok })
    }
    /// Create a new Node of type [`Node::Number`].
    pub fn new_num(val: &str, tok: Token) -> OldNode {
        OldNode::Number(OldNumberNode {
            val: val.to_string(),
            tok,
        })
    }
    /// Create a new Node of type [`Node::Object`].
    pub fn new_obj(entries: HashMap<String, OldNode>, toks: Vec<Token>) -> OldNode {
        OldNode::Object(OldObjectNode { entries, toks })
    }
    /// Create a new Node of type [`Node::String`].
    pub fn new_str(val: &str, tok: Token) -> OldNode {
        OldNode::String(OldStringNode {
            val: val.to_string(),
            tok,
        })
    }

    /// Get the string representation of a node type.
    pub fn get_typ_str(&self) -> &str {
        match self {
            OldNode::Array(_) => "array",
            OldNode::Bool(_) => "bool",
            OldNode::Null(_) => "null",
            OldNode::Number(_) => "number",
            OldNode::Object(_) => "object",
            OldNode::String(_) => "string",
        }
    }
    /// Get the tokens that were consumed to create this array.
    pub fn toks(&self) -> Vec<Token> {
        match self {
            OldNode::Array(n) => n.toks.clone(),
            OldNode::Bool(n) => vec![n.tok.clone()],
            OldNode::Null(n) => vec![n.tok.clone()],
            OldNode::Number(n) => vec![n.tok.clone()],
            OldNode::Object(n) => n.toks.clone(),
            OldNode::String(n) => vec![n.tok.clone()],
        }
    }
}
