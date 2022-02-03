use std::{collections::HashMap, rc::Rc};

use super::Node;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct VarDict {
    dict: HashMap<String, Node>,
    parent: Option<Rc<VarDict>>,
}

impl VarDict {
    pub fn new() -> VarDict {
        VarDict {
            dict: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: &Rc<VarDict>) -> VarDict {
        let mut dict = Self::new();
        dict.parent = Some(Rc::clone(parent));

        dict
    }

    pub fn insert(&mut self, key: String, node: Node) {
        self.dict.insert(key, node);
    }

    pub fn get(&self, key: &str) -> Option<&Node> {
        if let Some(n) = self.dict.get(key) {
            return Some(n);
        }

        if let Some(parent) = &self.parent {
            return parent.get(key);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::treebuilder::node::NullNode;

    use super::*;

    #[test]
    fn nodes_can_be_inserted_and_retrieved() {
        let mut dict = VarDict::new();

        assert_eq!(dict.get("foo"), None);

        dict.insert("foo".to_string(), Node::new_str("bar", 0, 0));

        assert_eq!(dict.get("foo"), Some(&Node::new_str("bar", 0, 0)));
    }

    #[test]
    fn queries_the_parent_var_dict() {
        let mut parent_dict = VarDict::new();
        parent_dict.insert("foo".to_string(), NullNode::new(0).into());

        let dict = VarDict::new_with_parent(&Rc::new(parent_dict));

        assert_eq!(dict.get("foo"), Some(&NullNode::new(0).into()));
    }

    #[test]
    fn current_dict_overrides_parent() {
        let mut parent_dict = VarDict::new();
        parent_dict.insert("foo".to_string(), NullNode::new(0).into());

        let mut dict = VarDict::new_with_parent(&Rc::new(parent_dict));
        dict.insert("foo".to_string(), Node::new_str("bar", 0, 0));

        assert_eq!(dict.get("foo"), Some(&Node::new_str("bar", 0, 0)));
    }
}
