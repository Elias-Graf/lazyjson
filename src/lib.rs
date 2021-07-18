use wasm_bindgen::prelude::*;

pub mod tokenizer;
pub mod treebuilder;

#[wasm_bindgen]
pub fn tokenize(inp: &str) -> JsValue {
    JsValue::from_serde(&tokenizer::tokenize(inp)).unwrap()
}
