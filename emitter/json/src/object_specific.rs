use lazyjson_core::treebuilder::{node, Node};

use crate::EmitJson;

impl EmitJson for node::ObjectSpecific {
    fn emit_json(&self, indent_level: usize) -> String {
        if self.entries.len() == 0 {
            return String::from("{}");
        }

        let entry_indent_level = indent_level + 1;

        let mut entries = self.entries.iter().collect::<Vec<(&String, &Node)>>();
        entries.sort_by_key(|&(key, _)| key);

        let mut entries_str = String::new();

        for i in 0..entries.len() {
            let (key, node) = entries[i];

            entries_str += &format!(
                "\n{}",
                lazyjson_core::emit::get_indentation(entry_indent_level)
            );
            entries_str += &format!("\"{}\": {}", key, node.emit_json(entry_indent_level));

            if i < self.entries.len() - 1 {
                entries_str += ",";
            }
        }

        format!(
            "{{{}\n{}}}",
            entries_str,
            lazyjson_core::emit::get_indentation(indent_level)
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use lazyjson_core::treebuilder::Node;

    use super::*;

    #[test]
    fn object_specific_empty() {
        let obj = Node::new_obj(HashMap::new(), 0, 0);

        assert_eq!(obj.emit_json(0), "{}");
    }

    #[test]
    fn object_specific_not_empty() {
        let mut entries = HashMap::new();
        entries.insert(String::from("bar"), Node::new_null(0, 0));
        entries.insert(String::from("foo"), Node::new_bool(false, 0, 0));

        let obj = Node::new_obj(entries, 0, 0);

        assert_eq!(
            obj.emit_json(0),
            "{
    \"bar\": null,
    \"foo\": false
}"
        );
    }

    #[test]
    fn object_specific_nested() {
        let mut inner_1 = HashMap::new();
        inner_1.insert(String::from("bar"), Node::new_bool(true, 0, 0));

        let mut inner_2 = HashMap::new();
        inner_2.insert(String::from("foo"), Node::new_null(0, 0));

        let mut outer = HashMap::new();
        outer.insert(String::from("inner_1"), Node::new_obj(inner_1, 0, 0));
        outer.insert(String::from("inner_2"), Node::new_obj(inner_2, 0, 0));

        let obj = Node::new_obj(outer, 0, 0);

        assert_eq!(
            obj.emit_json(0),
            "{
    \"inner_1\": {
        \"bar\": true
    },
    \"inner_2\": {
        \"foo\": null
    }
}"
        );
    }
}
