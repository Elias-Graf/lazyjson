use lazyjson_core::treebuilder::{node::ArrayNode, Node, VarDict};

pub fn create_arr(toks: Vec<Node>) -> ArrayNode {
    ArrayNode::new(0, 0, toks, VarDict::new())
}

pub fn create_bool(val: bool) -> Node {
    Node::new_bool(val, 0, 0)
}

pub fn create_null() -> Node {
    Node::new_null(0, 0)
}
