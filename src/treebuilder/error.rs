use super::node::NodeType;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct UnterminatedContainer {
    node_typ: NodeType,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TreebuilderError {
    UnterminatedContainer(UnterminatedContainer),
}

impl UnterminatedContainer {
    pub fn new(node_typ: NodeType) -> UnterminatedContainer {
        UnterminatedContainer { node_typ }
    }
}

// pub enum ErrorKind {
//     MissingValue,
//     UnexpectedToken,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct TreebuilderError {
//     pub kind: ErrorKind,
// }
