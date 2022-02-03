use lazyjson_core::treebuilder::Node;

mod emit_json;
pub use emit_json::EmitJson;

mod array_specific;
mod bool_specific;
mod null_specific;
mod number_specific;
mod object_specific;
mod string_specific;

#[cfg(test)]
mod testing;

impl EmitJson for Node {
    fn emit_json(&self, indent_level: usize) -> String {
        match &self {
            Node::Array(a) => a.emit_json(indent_level),
            Node::Bool(b) => b.emit_json(indent_level),
            Node::Null(n) => n.emit_json(indent_level),
            Node::Number(n) => n.emit_json(indent_level),
            Node::Object(o) => o.emit_json(indent_level),
            Node::String(s) => s.emit_json(indent_level),
        }
    }
}
