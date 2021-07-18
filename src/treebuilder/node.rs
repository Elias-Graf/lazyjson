use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeType {
    Object,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ObjectNode {
    entries: HashMap<String, Node>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BoolNode {
    val: bool,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NullNode {}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ValueNode {
    Bool(BoolNode),
    Null(NullNode),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Node {
    Object(ObjectNode),
    Value(ValueNode),
}

impl ObjectNode {
    pub fn new(entries: HashMap<String, Node>) -> ObjectNode {
        ObjectNode { entries }
    }
}

impl BoolNode {
    pub fn new(val: bool) -> BoolNode {
        BoolNode { val }
    }
}

impl NullNode {
    pub fn new() -> NullNode {
        NullNode {}
    }
}
// #[derive(Eq, PartialEq, Debug, Clone)]

// pub enum NodeValueType {
//     VALUE,
//     OBJECT,
//     ARRAY,
// }

// #[derive(Eq, PartialEq, Debug, Clone)]
// pub struct InvalidConversion {
//     requested: NodeValueType,
//     actual: NodeValueType,
// }

// impl InvalidConversion {
//     pub fn new(actual: NodeValueType, requested: NodeValueType) -> InvalidConversion {
//         InvalidConversion { actual, requested }
//     }
// }

// #[derive(Eq, PartialEq, Debug, Clone)]
// pub struct Node {

//     key: String,
//     val: String,
//     val_typ: NodeValueType,
// }

// impl Node {
//     pub fn obj() -> Node {
//         Node {

//         }
//     }

//     pub fn bool(key: &str, val: &str) -> Node {
//         Node {
//             key: key.into(),
//             val: val.into(),
//             val_typ: NodeValueType::BOOL,
//         }
//     }
//     pub fn null(key: &str) -> Node {
//         Node {
//             key: key.into(),
//             // TODO: This may be removed.
//             // Technically not all nodes require a value (for example this one).
//             // For simplicity sake I added one though.
//             val: "".into(),
//             val_typ: NodeValueType::NULL,
//         }
//     }

//     /// Checks if the node value is a [`bool`] and if yes returns it.
//     /// Otherwise throws [`InvalidConversion`].
//     pub fn as_bool(self) -> Result<bool, InvalidConversion> {
//         match self.val_typ {
//             NodeValueType::BOOL => Ok(self.val == "true"),
//             typ => Err(InvalidConversion::new(typ, NodeValueType::BOOL)),
//         }
//     }
// }
