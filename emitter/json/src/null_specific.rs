use lazyjson_core::treebuilder::node;

use crate::EmitJson;

impl EmitJson for node::NullSpecific {
    fn emit_json(&self, _: usize) -> String {
        String::from("null")
    }
}

#[cfg(test)]
mod tests {
    use lazyjson_core::treebuilder::Node;

    use super::*;

    #[test]
    fn null_specific() {
        let null = Node::new_null(0, 0);

        assert_eq!(null.emit_json(0), "null");
    }
}
