use lazyjson_core::treebuilder::{node, Node, NodeSpecific};

mod emit;
pub use emit::Emit;

impl Emit for Node {
    fn emit(&self, indent_level: usize) -> String {
        match &self.specific {
            NodeSpecific::Array(a) => a.emit(indent_level),
            NodeSpecific::Bool(b) => b.emit(indent_level),
            NodeSpecific::Null(n) => n.emit(indent_level),
            NodeSpecific::Number(n) => n.emit(indent_level),
            NodeSpecific::String(s) => s.emit(indent_level),
            NodeSpecific::Object(o) => o.emit(indent_level),
        }
    }
}

impl Emit for node::ArraySpecific {
    fn emit(&self, indent_level: usize) -> String {
        if self.entries.len() == 0 {
            return String::from("[]");
        }

        let entry_indent_level = indent_level + 1;

        let mut entries_str = String::new();
        for i in 0..self.entries.len() {
            let node = &self.entries[i];

            entries_str += &format!("\n{}", self.get_indentation(entry_indent_level));
            entries_str += &node.emit(indent_level + 1);

            if i < self.entries.len() - 1 {
                entries_str += ",";
            }
        }

        format!("[{}\n{}]", entries_str, self.get_indentation(indent_level))
    }
}

impl Emit for node::BoolSpecific {
    fn emit(&self, _: usize) -> String {
        if self.val {
            return String::from("true");
        }

        String::from("false")
    }
}

impl Emit for node::NullSpecific {
    fn emit(&self, _: usize) -> String {
        String::from("null")
    }
}

impl Emit for node::NumberSpecific {
    fn emit(&self, _: usize) -> String {
        self.val.to_string()
    }
}

impl Emit for node::StringSpecific {
    fn emit(&self, _: usize) -> String {
        format!("\"{}\"", self.val)
    }
}

impl Emit for node::ObjectSpecific {
    fn emit(&self, indent_level: usize) -> String {
        if self.entries.len() == 0 {
            return String::from("{}");
        }

        let entry_indent_level = indent_level + 1;

        let mut entries = self.entries.iter().collect::<Vec<(&String, &Node)>>();
        entries.sort_by_key(|&(key, _)| key);

        let mut entries_str = String::new();

        for i in 0..entries.len() {
            let (key, node) = entries[i];

            entries_str += &format!("\n{}", self.get_indentation(entry_indent_level));
            entries_str += &format!("\"{}\": {}", key, node.emit(entry_indent_level));
       
            if i < self.entries.len() - 1 {
                entries_str += ",";
            }
        }

        format!("{{{}\n{}}}", entries_str, self.get_indentation(indent_level))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn array_specific_empty() {
        let array = Node::new_arr(Vec::new(), 0, 0);

        assert_eq!(array.emit(0), "[]");
    }

    #[test]
    fn array_specific_not_empty() {
        let arr = Node::new_arr(
            vec![Node::new_num("0", 0, 0), Node::new_str("foo", 0, 0)],
            0,
            0,
        );

        assert_eq!(
            arr.emit(0),
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
            arr.emit(0),
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

    #[test]
    fn bool_specific_false() {
        let n_false = Node::new_bool(false, 0, 0);

        assert_eq!(n_false.emit(0), "false");
    }

    #[test]
    fn bool_specific_true() {
        let n_true = Node::new_bool(true, 0, 0);

        assert_eq!(n_true.emit(0), "true");
    }

    #[test]
    fn null_specific() {
        let null = Node::new_null(0, 0);

        assert_eq!(null.emit(0), "null");
    }

    #[test]
    fn number_specific() {
        for i in 0..10 {
            let num = Node::new_num(&i.to_string(), 0, 0);

            assert_eq!(num.emit(0), i.to_string());
        }
    }

    #[test]
    fn string_specific() {
        for i in 0..10 {
            let str = Node::new_str(&format!("str: {}", i), 0, 0);

            assert_eq!(str.emit(0), format!("\"str: {}\"", i));
        }
    }

    #[test]
    fn object_specific_empty() {
        let obj = Node::new_obj(HashMap::new(), 0, 0);

        assert_eq!(obj.emit(0), "{}");
    }

    #[test]
    fn object_specific_not_empty() {
        let mut entries = HashMap::new();
        entries.insert(String::from("bar"), Node::new_null(0, 0));
        entries.insert(String::from("foo"), Node::new_bool(false, 0, 0));

        let obj = Node::new_obj(entries, 0, 0);

        assert_eq!(
            obj.emit(0),
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
            obj.emit(0),
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
