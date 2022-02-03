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
pub struct StringSpecific {
    pub val: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeSpecific {
    Array(ArrayNode),
    Bool(BoolNode),
    Null(NullNode),
    Number(NumberNode),
    Object(ObjectNode),
    String(StringSpecific),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Node {
    pub specific: NodeSpecific,
    #[deprecated(note = "use the method `from()` instead")]
    pub from: usize,
    #[deprecated(note = "use the method `to()` instead")]
    pub to: usize,
}

impl Node {
    pub fn new_str(val: &str, from: usize, to: usize) -> Node {
        Node {
            specific: NodeSpecific::String(StringSpecific {
                val: val.to_string(),
            }),
            from,
            to,
        }
    }

    /// Returns the index of the starting token that was used to create this node.
    /// **The index is inclusive.**
    pub fn from(&self) -> usize {
        self.from
    }
    /// Returns the index of the ending token that was used to create this node.
    /// **The index is *NON* inclusive.**
    pub fn to(&self) -> usize {
        self.to
    }
}

impl From<ArrayNode> for Node {
    fn from(arr: ArrayNode) -> Self {
        Node {
            from: arr.from,
            to: arr.to,
            specific: NodeSpecific::Array(arr),
        }
    }
}

impl From<BoolNode> for Node {
    fn from(bl: BoolNode) -> Self {
        Node {
            from: bl.from,
            to: bl.to,
            specific: NodeSpecific::Bool(bl),
        }
    }
}

impl From<NullNode> for Node {
    fn from(null: NullNode) -> Self {
        Node {
            from: null.from,
            to: null.to,
            specific: NodeSpecific::Null(null),
        }
    }
}

impl From<NumberNode> for Node {
    fn from(num: NumberNode) -> Self {
        Node {
            from: num.from,
            to: num.to,
            specific: NodeSpecific::Number(num),
        }
    }
}

impl From<ObjectNode> for Node {
    fn from(obj: ObjectNode) -> Self {
        Node {
            from: obj.from,
            to: obj.to,
            specific: NodeSpecific::Object(obj),
        }
    }
}
