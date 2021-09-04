use core::fmt;
use std::{collections::HashMap, fmt::Debug};

use crate::tokenizer::Token;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ArrayNode {
    pub entries: Vec<Node>,
    toks: Vec<Token>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BoolNode {
    pub val: bool,
    tok: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NullNode {
    tok: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NumberNode {
    pub val: String,
    tok: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ObjectNode {
    pub entries: HashMap<String, Node>,
    toks: Vec<Token>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct StringNode {
    pub val: String,
    tok: Token,
}

impl fmt::Display for StringNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Node {
    Array(ArrayNode),
    Bool(BoolNode),
    Null(NullNode),
    Number(NumberNode),
    Object(ObjectNode),
    String(StringNode),
}

impl Node {
    /// Create a new Node of type [`Node::Array`].
    pub fn new_arr(entries: Vec<Node>, toks: Vec<Token>) -> Node {
        Node::Array(ArrayNode { entries, toks })
    }
    /// Create a new Node of type [`Node::Bool`].
    pub fn new_bool(val: bool, tok: Token) -> Node {
        Node::Bool(BoolNode { val, tok })
    }
    /// Create a new Node of type [`Node::Null`].
    pub fn new_null(tok: Token) -> Node {
        Node::Null(NullNode { tok })
    }
    /// Create a new Node of type [`Node::Number`].
    pub fn new_num(val: &str, tok: Token) -> Node {
        Node::Number(NumberNode {
            val: val.to_string(),
            tok,
        })
    }
    /// Create a new Node of type [`Node::Object`].
    pub fn new_obj(entries: HashMap<String, Node>, toks: Vec<Token>) -> Node {
        Node::Object(ObjectNode { entries, toks })
    }
    /// Create a new Node of type [`Node::String`].
    pub fn new_str(val: &str, tok: Token) -> Node {
        Node::String(StringNode {
            val: val.to_string(),
            tok,
        })
    }

    /// Get the string representation of a node type.
    pub fn get_typ_str(&self) -> &str {
        match self {
            Node::Array(_) => "array",
            Node::Bool(_) => "bool",
            Node::Null(_) => "null",
            Node::Number(_) => "number",
            Node::Object(_) => "object",
            Node::String(_) => "string",
        }
    }
    /// Get the tokens that were consumed to create this array.
    pub fn toks(&self) -> Vec<Token> {
        match self {
            Node::Array(n) => n.toks.clone(),
            Node::Bool(n) => vec![n.tok.clone()],
            Node::Null(n) => vec![n.tok.clone()],
            Node::Number(n) => vec![n.tok.clone()],
            Node::Object(n) => n.toks.clone(),
            Node::String(n) => vec![n.tok.clone()],
        }
    }
}
