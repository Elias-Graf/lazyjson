use std::{collections::HashMap, fmt::Debug};

use super::var_dict::VarDict;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ArrayNode {
    pub entries: Vec<Node>,
    pub var_dict: VarDict,
    pub from: usize,
    pub to: usize,
}

impl ArrayNode {
    pub fn new(from: usize, to: usize, entries: Vec<Node>, var_dict: VarDict) -> ArrayNode {
        ArrayNode {
            entries,
            var_dict,
            from,
            to,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BoolNode {
    pub val: bool,
    pub from: usize,
    pub to: usize,
}

impl BoolNode {
    pub fn new(i: usize, val: bool) -> BoolNode {
        BoolNode {
            from: i,
            to: i + 1,
            val,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NullNode {
    pub from: usize,
    pub to: usize,
}

impl NullNode {
    pub fn new(i: usize) -> NullNode {
        NullNode { from: i, to: i + 1 }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NumberNode {
    pub val: String,
    pub from: usize,
    pub to: usize,
}

impl NumberNode {
    pub fn new(i: usize, val: String) -> NumberNode {
        NumberNode {
            from: i,
            to: i + 1,
            val,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ObjectNode {
    pub entries: HashMap<String, Node>,
    pub var_dict: VarDict,
    from: usize,
    to: usize,
}

impl ObjectNode {
    pub fn new(
        from: usize,
        to: usize,
        entries: HashMap<String, Node>,
        var_dict: VarDict,
    ) -> ObjectNode {
        ObjectNode {
            from,
            to,
            entries,
            var_dict,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct StringNode {
    pub val: String,
    pub from: usize,
    pub to: usize,
}

impl StringNode {
    pub fn new(i: usize, val: String) -> StringNode {
        StringNode {
            val,
            from: i,
            to: i + 1,
        }
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
    /// Returns the index of the starting token that was used to create this node.
    /// **The index is inclusive.**
    pub fn from(&self) -> usize {
        match &self {
            Node::Array(a) => a.from,
            Node::Bool(b) => b.from,
            Node::Null(n) => n.from,
            Node::Number(n) => n.from,
            Node::Object(o) => o.from,
            Node::String(s) => s.from,
        }
    }
    /// Returns the index of the ending token that was used to create this node.
    /// **The index is *NON* inclusive.**
    pub fn to(&self) -> usize {
        match &self {
            Node::Array(a) => a.to,
            Node::Bool(b) => b.to,
            Node::Null(n) => n.to,
            Node::Number(n) => n.to,
            Node::Object(o) => o.to,
            Node::String(s) => s.to,
        }
    }
}

impl From<ArrayNode> for Node {
    fn from(arr: ArrayNode) -> Self {
        Node::Array(arr)
    }
}

impl From<BoolNode> for Node {
    fn from(bl: BoolNode) -> Self {
        Node::Bool(bl)
    }
}

impl From<NullNode> for Node {
    fn from(null: NullNode) -> Self {
        Node::Null(null)
    }
}

impl From<NumberNode> for Node {
    fn from(num: NumberNode) -> Self {
        Node::Number(num)
    }
}

impl From<ObjectNode> for Node {
    fn from(obj: ObjectNode) -> Self {
        Node::Object(obj)
    }
}

impl From<StringNode> for Node {
    fn from(str: StringNode) -> Self {
        Node::String(str)
    }
}
