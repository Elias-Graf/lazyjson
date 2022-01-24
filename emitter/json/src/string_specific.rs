use lazyjson_core::treebuilder::node;

use crate::EmitJson;

impl EmitJson for node::StringSpecific {
    fn emit_json(&self, _: usize) -> String {
        format!("\"{}\"", self.val)
    }
}

#[cfg(test)]
mod tests {
    use lazyjson_core::treebuilder::Node;

    use super::*;

    #[test]
    fn string_specific() {
        for i in 0..10 {
            let str = Node::new_str(&format!("str: {}", i), 0, 0);

            assert_eq!(str.emit_json(0), format!("\"str: {}\"", i));
        }
    }
}
