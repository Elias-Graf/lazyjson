use super::node::Node;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ConsumerResponse {
    pub cons: usize,
    pub node: Option<Node>,
}

impl ConsumerResponse {
    pub fn new(cons: usize, node: Option<Node>) -> ConsumerResponse {
        ConsumerResponse { cons, node }
    }
}
