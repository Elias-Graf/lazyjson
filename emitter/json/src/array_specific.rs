use lazyjson_core::treebuilder::node::ArrayNode;

use crate::EmitJson;

impl EmitJson for ArrayNode {
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

    use crate::{
        testing::{create_arr, create_bool, create_null, create_num, create_str},
        EmitJson,
    };

    #[test]
    fn array_specific_empty() {
        let arr: Node = create_arr(Vec::new()).into();

        assert_eq!(arr.emit_json(0), "[]");
    }

    #[test]
    fn array_specific_not_empty() {
        let arr = create_arr(vec![create_num("0").into(), create_str("foo").into()]);

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
        let arr: Node = create_arr(vec![
            create_arr(vec![create_bool(false).into()]).into(),
            create_arr(vec![create_null().into()]).into(),
        ])
        .into();

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
