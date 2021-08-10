use wasm_bindgen::{prelude::*, JsCast};

pub mod tokenizer;
pub mod treebuilder;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Node[]")]
    pub type NodeArray;
}

#[wasm_bindgen]
pub fn tokenize(inp: &str) -> JsValue {
    JsValue::from_serde(&tokenizer::tokenize(inp)).unwrap()
}

#[wasm_bindgen]
pub fn parse(inp: &str) -> NodeArray {
    let toks = tokenizer::tokenize(inp);
    let resp = treebuilder::value_consumer::value_consumer(&toks, 0);

    JsValue::from_serde(&resp.unwrap().node)
        .unwrap()
        .unchecked_into::<NodeArray>()
}
