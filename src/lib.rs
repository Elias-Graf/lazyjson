mod tokenizer;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(inp: &str) -> JsValue {
    JsValue::from_serde(&tokenizer::tokenize(inp)).unwrap()
}
