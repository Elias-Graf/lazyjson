use super::node::Node;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ConsumerResponse {
    pub cons: usize,
    pub node: Option<Node>,
}
