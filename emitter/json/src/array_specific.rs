use lazyjson_core::treebuilder::node;

use crate::EmitJson;

impl EmitJson for node::ArraySpecific {
    fn emit_json(&self, indent_level: usize) -> String {
        if self.entries.len() == 0 {
            return String::from("[]");
        }

        let entry_indent_level = indent_level + 1;

        let mut entries_str = String::new();
        for i in 0..self.entries.len() {
            let node = &self.entries[i];

            entries_str += &format!(
                "\n{}",
                lazyjson_core::emit::get_indentation(entry_indent_level)
            );
            entries_str += &node.emit_json(indent_level + 1);

            if i < self.entries.len() - 1 {
                entries_str += ",";
            }
        }

        format!(
            "[{}\n{}]",
            entries_str,
            lazyjson_core::emit::get_indentation(indent_level)
        )
    }
}

#[cfg(test)]
mod tests {
    use lazyjson_core::treebuilder::Node;

    use crate::EmitJson;

    #[test]
    fn array_specific_empty() {
        let array = Node::new_arr(Vec::new(), 0, 0);

        assert_eq!(array.emit_json(0), "[]");
    }

    #[test]
    fn array_specific_not_empty() {
        let arr = Node::new_arr(
            vec![Node::new_num("0", 0, 0), Node::new_str("foo", 0, 0)],
            0,
            0,
        );

        assert_eq!(
            arr.emit_json(0),
            "[
    0,
    \"foo\"
]"
        )
    }

    #[test]
    fn array_specific_nested() {
        let arr = Node::new_arr(
            vec![
                Node::new_arr(vec![Node::new_bool(false, 0, 0)], 0, 0),
                Node::new_arr(vec![Node::new_null(0, 0)], 0, 0),
            ],
            0,
            0,
        );

        assert_eq!(
            arr.emit_json(0),
            "[
    [
        false
    ],
    [
        null
    ]
]"
        );
    }
}
