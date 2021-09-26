use std::{collections::HashMap, fmt::Debug};

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
    pub from: usize,
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
    pub fn new_obj(entries: HashMap<String, Node>, from: usize, to: usize) -> Node {
        Node {
            specific: NodeSpecific::Object(ObjectSpecific { entries }),
            from,
            to,
        }
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
}
