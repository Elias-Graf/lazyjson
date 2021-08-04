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
pub struct ArrayNode {
    entries: Vec<Node>,
}

impl ArrayNode {
    pub fn new(entries: Vec<Node>) -> ArrayNode {
        ArrayNode {
            entries: entries.clone(),
        }
    }
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
pub enum ContainerNode {
    Array(ArrayNode),
    Object(ObjectNode),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Node {
    Container(ContainerNode),
    Value(ValueNode),
}

impl Node {
    pub fn new_arr(entries: Vec<Node>) -> Node {
        Node::Container(ContainerNode::Array(ArrayNode::new(entries)))
    }

    pub fn new_bool(val: bool) -> Node {
        Node::Value(ValueNode::Bool(BoolNode::new(val)))
    }

    pub fn new_null() -> Node {
        Node::Value(ValueNode::Null(NullNode {}))
    }

    pub fn new_num(val: &str) -> Node {
        Node::Value(ValueNode::Number(NumberNode::new(val)))
    }

    pub fn new_obj(entries: HashMap<String, Node>) -> Node {
        Node::Container(ContainerNode::Object(ObjectNode::new(entries)))
    }

    pub fn new_str(val: &str) -> Node {
        Node::Value(ValueNode::String(StringNode::new(val)))
    }
}
