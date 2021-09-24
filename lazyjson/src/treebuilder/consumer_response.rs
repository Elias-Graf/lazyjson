use super::old_node::OldNode;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ConsumerResponse {
    pub cons: usize,
    pub node: Option<OldNode>,
}

impl ConsumerResponse {
    pub fn new(cons: usize, node: Option<OldNode>) -> ConsumerResponse {
        ConsumerResponse { cons, node }
    }
}
