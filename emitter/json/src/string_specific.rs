use lazyjson_core::treebuilder::node::StringNode;

use crate::EmitJson;

impl EmitJson for StringNode {
    fn emit_json(&self, _: usize) -> String {
        format!("\"{}\"", self.val)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::create_str;

    use super::*;

    #[test]
    fn string_specific() {
        for i in 0..10 {
            let str = create_str(&format!("str: {}", i));

            assert_eq!(str.emit_json(0), format!("\"str: {}\"", i));
        }
    }
}
