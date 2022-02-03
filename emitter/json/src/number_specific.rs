use lazyjson_core::treebuilder::node::NumberNode;

use crate::EmitJson;

impl EmitJson for NumberNode {
    fn emit_json(&self, _: usize) -> String {
        self.val.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::create_num;

    use super::*;

    #[test]
    fn number_specific() {
        for i in 0..10 {
            let num = create_num(&i.to_string());

            assert_eq!(num.emit_json(0), i.to_string());
        }
    }
}
