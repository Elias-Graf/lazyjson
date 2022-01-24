use lazyjson_core::treebuilder::{Node, NodeSpecific};

mod emit_json;
pub use emit_json::EmitJson;

mod array_specific;
mod bool_specific;
mod null_specific;
mod number_specific;
mod object_specific;
mod string_specific;

impl EmitJson for Node {
    fn emit_json(&self, indent_level: usize) -> String {
        match &self.specific {
            NodeSpecific::Array(a) => a.emit_json(indent_level),
            NodeSpecific::Bool(b) => b.emit_json(indent_level),
            NodeSpecific::Null(n) => n.emit_json(indent_level),
            NodeSpecific::Number(n) => n.emit_json(indent_level),
            NodeSpecific::Object(o) => o.emit_json(indent_level),
            NodeSpecific::String(s) => s.emit_json(indent_level),
        }
    }
}
