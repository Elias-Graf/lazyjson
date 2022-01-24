use lazyjson_core::treebuilder::node;

use crate::EmitJson;

impl EmitJson for node::BoolSpecific {
    fn emit_json(&self, _: usize) -> String {
        if self.val {
            return String::from("true");
        }

        String::from("false")
    }
}

#[cfg(test)]
mod tests {
    use lazyjson_core::treebuilder::Node;

    use super::*;

    #[test]
    fn bool_specific_false() {
        let n_false = Node::new_bool(false, 0, 0);

        assert_eq!(n_false.emit_json(0), "false");
    }

    #[test]
    fn bool_specific_true() {
        let n_true = Node::new_bool(true, 0, 0);

        assert_eq!(n_true.emit_json(0), "true");
    }
}
