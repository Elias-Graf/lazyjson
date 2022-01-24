pub trait EmitJson {
    fn emit_json(&self, indent_level: usize) -> String;
}
