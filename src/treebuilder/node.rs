use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeType {
    Array,
    Object,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BoolNode {
    val: bool,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NullNode {}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NumberNode {
    val: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct StringNode {
    val: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ArrayNode {
    entries: Vec<Node>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ObjectNode {
    entries: HashMap<String, Node>,
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
    pub fn new_arr(entries: Vec<Node>) -> Node {
        Node::Array(ArrayNode {entries})
    }

    pub fn new_bool(val: bool) -> Node {
        Node::Bool(BoolNode{val})
    }

    pub fn new_null() -> Node {
        Node::Null(NullNode{})
    }

    pub fn new_num(val: &str) -> Node {
        Node::Number(NumberNode {val: val.to_string()})
    }

    pub fn new_obj(entries: HashMap<String, Node>) -> Node {
        Node::Object(ObjectNode {entries})
    }

    pub fn new_str(val: &str) -> Node {
        Node::String(StringNode {val: val.to_string()})
    }
}
