use std::{collections::HashMap, fmt::Debug};

use crate::tokenizer::Token;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeType {
    Array,
    Object,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ArrayNode {
    entries: Vec<Node>,
    toks: Vec<Token>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BoolNode {
    val: bool,
    tok: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NullNode {
    tok: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NumberNode {
    val: String,
    tok: Token,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ObjectNode {
    entries: HashMap<String, Node>,
    toks: Vec<Token>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct StringNode {
    val: String,
    tok: Token,
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
    pub fn new_arr(entries: Vec<Node>, toks: Vec<Token>) -> Node {
        Node::Array(ArrayNode { entries, toks })
    }

    pub fn new_bool(val: bool, tok: Token) -> Node {
        Node::Bool(BoolNode { val, tok })
    }

    pub fn new_null(tok: Token) -> Node {
        Node::Null(NullNode { tok })
    }

    pub fn new_num(val: &str, tok: Token) -> Node {
        Node::Number(NumberNode {
            val: val.to_string(),
            tok,
        })
    }

    pub fn new_obj(entries: HashMap<String, Node>, toks: Vec<Token>) -> Node {
        Node::Object(ObjectNode { entries, toks })
    }

    pub fn new_str(val: &str, tok: Token) -> Node {
        Node::String(StringNode {
            val: val.to_string(),
            tok,
        })
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
