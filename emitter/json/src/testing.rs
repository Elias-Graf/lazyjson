use lazyjson_core::treebuilder::{
    node::{ArrayNode, BoolNode},
    Node, VarDict,
};

pub fn create_arr(toks: Vec<Node>) -> ArrayNode {
    ArrayNode::new(0, 0, toks, VarDict::new())
}

pub fn create_bool(val: bool) -> BoolNode {
    BoolNode::new(0, val)
}

pub fn create_null() -> Node {
    Node::new_null(0, 0)
}
