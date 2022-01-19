pub trait Emit {
    fn emit(&self, indent_level: usize) -> String;

    fn get_indentation(&self, level: usize) -> String {
        "    ".repeat(level)
    }
}