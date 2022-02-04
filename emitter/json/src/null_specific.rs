use lazyjson_core::treebuilder::node::NullNode;

use crate::EmitJson;

impl EmitJson for NullNode {
    fn emit_json(&self, _: usize) -> String {
        String::from("null")
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::create_null;

    use super::*;

    #[test]
    fn null_specific() {
        let null = create_null();

        assert_eq!(null.emit_json(0), "null");
    }
}
