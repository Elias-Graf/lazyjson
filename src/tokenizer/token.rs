use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Copy)]
pub enum TokenType {
    KeywordLiteral,
    NumberLiteral,
    Operator,
    Separator,
    StringLiteral,
}

#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub typ: TokenType,
    #[wasm_bindgen(skip)]
    pub val: String,
}

#[wasm_bindgen]
impl Token {
    #[wasm_bindgen(getter)]
    pub fn val(&self) -> String {
        self.val.clone()
    }
}
