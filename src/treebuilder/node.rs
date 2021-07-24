use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeType {
    Object,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ObjectNode {
    entries: HashMap<String, Node>,
}

impl ObjectNode {
    pub fn new(entries: HashMap<String, Node>) -> ObjectNode {
        ObjectNode { entries }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BoolNode {
    val: bool,
}

impl BoolNode {
    pub fn new(val: bool) -> BoolNode {
        BoolNode { val }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NullNode {}

impl NullNode {
    pub fn new() -> NullNode {
        NullNode {}
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NumberNode {
    val: String,
}

impl NumberNode {
    pub fn new(val: &str) -> NumberNode {
        NumberNode { val: val.into() }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct StringNode {
    val: String,
}

impl StringNode {
    pub fn new(val: &str) -> StringNode {
        StringNode { val: val.into() }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ValueNode {
    Bool(BoolNode),
    Null(NullNode),
    Number(NumberNode),
    String(StringNode),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Node {
    Object(ObjectNode),
    Value(ValueNode),
}
