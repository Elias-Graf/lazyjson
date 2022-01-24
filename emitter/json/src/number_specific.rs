use lazyjson_core::treebuilder::node;

use crate::EmitJson;

impl EmitJson for node::NumberSpecific {
    fn emit_json(&self, _: usize) -> String {
        self.val.to_string()
    }
}

#[cfg(test)]
mod tests {
    use lazyjson_core::treebuilder::Node;

    use super::*;

    #[test]
    fn number_specific() {
        for i in 0..10 {
            let num = Node::new_num(&i.to_string(), 0, 0);

            assert_eq!(num.emit_json(0), i.to_string());
        }
    }
}
