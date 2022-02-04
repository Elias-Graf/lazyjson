use lazyjson_core::treebuilder::node::BoolNode;

use crate::EmitJson;

impl EmitJson for BoolNode {
    fn emit_json(&self, _: usize) -> String {
        if self.val {
            return String::from("true");
        }

        String::from("false")
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::create_bool;

    use super::*;

    #[test]
    fn bool_specific_false() {
        let bl_false = create_bool(false);

        assert_eq!(bl_false.emit_json(0), "false");
    }

    #[test]
    fn bool_specific_true() {
        let bl_false = create_bool(true);

        assert_eq!(bl_false.emit_json(0), "true");
    }
}
