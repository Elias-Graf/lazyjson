use std::collections::HashMap;

use lazyjson_core::treebuilder::{
    node::{ArrayNode, BoolNode, NullNode, NumberNode, ObjectNode, StringNode},
    Node,
};

pub fn create_arr(toks: Vec<Node>) -> ArrayNode {
    ArrayNode::new(0, 0, toks)
}

pub fn create_bool(val: bool) -> BoolNode {
    BoolNode::new(0, val)
}

pub fn create_null() -> NullNode {
    NullNode::new(0)
}

pub fn create_num(val: &str) -> NumberNode {
    NumberNode::new(0, val.to_owned())
}

pub fn create_obj(entries: HashMap<String, Node>) -> ObjectNode {
    ObjectNode::new(0, 0, entries)
}

pub fn create_str(val: &str) -> StringNode {
    StringNode::new(0, val.to_owned())
}
