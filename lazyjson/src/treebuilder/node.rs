use std::{collections::HashMap, fmt::Debug};

use super::var_dict::VarDict;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ArraySpecific {
    pub entries: Vec<Node>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BoolSpecific {
    pub val: bool,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NullSpecific {}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NumberSpecific {
    pub val: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ObjectSpecific {
    pub entries: HashMap<String, Node>,
    pub var_dict: VarDict,
    from: usize,
    to: usize,
}

impl ObjectSpecific {
    pub fn new(from: usize, to: usize) -> ObjectSpecific {
        ObjectSpecific {
            from,
            to,
            entries: HashMap::new(),
            var_dict: VarDict::new(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct StringSpecific {
    pub val: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeSpecific {
    Array(ArraySpecific),
    Bool(BoolSpecific),
    Null(NullSpecific),
    Number(NumberSpecific),
    Object(ObjectSpecific),
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
    pub fn new_arr(entries: Vec<Node>, from: usize, to: usize) -> Node {
        Node {
            specific: NodeSpecific::Array(ArraySpecific { entries }),
            from,
            to,
        }
    }
    pub fn new_bool(val: bool, from: usize, to: usize) -> Node {
        Node {
            specific: NodeSpecific::Bool(BoolSpecific { val }),
            from,
            to,
        }
    }
    pub fn new_null(from: usize, to: usize) -> Node {
        Node {
            specific: NodeSpecific::Null(NullSpecific {}),
            from,
            to,
        }
    }
    pub fn new_num(val: &str, from: usize, to: usize) -> Node {
        Node {
            specific: NodeSpecific::Number(NumberSpecific {
                val: val.to_string(),
            }),
            from,
            to,
        }
    }
    pub fn new_obj(entries: HashMap<String, Node>, from: usize, to: usize) -> ObjectSpecific {
        let mut obj = ObjectSpecific::new(from, to);
        obj.entries = entries;

        obj
    }
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

impl From<ObjectSpecific> for Node {
    fn from(object: ObjectSpecific) -> Self {
        Node {
            from: object.from,
            to: object.to,
            specific: NodeSpecific::Object(object),
        }
    }
}
