use lazyjson_core::{
    tokenizer,
    treebuilder::{self, error::TreebuilderErr},
};
use lazyjson_emitter_json::EmitJson;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod utils;

#[wasm_bindgen]
pub struct Config {
    pub allow_trailing_commas: bool,
    pub allow_line_comments: bool,
}

#[wasm_bindgen]
impl Config {
    pub fn new() -> Config {
        Config {
            allow_line_comments: false,
            allow_trailing_commas: false,
        }
    }
}

impl Into<treebuilder::Config> for Config {
    fn into(self) -> treebuilder::Config {
        treebuilder::Config {
            allow_line_comments: self.allow_line_comments,
            allow_trailing_commas: self.allow_trailing_commas,
        }
    }
}

#[wasm_bindgen]
pub struct Error {
    pub from: usize,
    pub to: usize,
    #[wasm_bindgen(getter_with_clone)]
    pub msg: String,
}

impl From<tokenizer::TokenizationErr> for Error {
    fn from(e: tokenizer::TokenizationErr) -> Self {
        // TODO: the tokenization error should offer a way to get a simple error
        // message, that does not require and external arguments (like `inp`).
        let msg = match e.typ {
            tokenizer::error::TokenizationErrTyp::LineCommentsNotAllowed => {
                "line comments not allowed"
            }
            tokenizer::error::TokenizationErrTyp::NoInp => "no input",
            // TODO: investigate if this error really is necessary, or if out of
            // bounds can simply be prevented by having a solid test suite.
            tokenizer::error::TokenizationErrTyp::OutOfBounds => {
                "out of bounds - almost certainly a bug"
            }
            tokenizer::error::TokenizationErrTyp::UnterminatedStr => "unterminated string",
        };

        Error {
            from: e.from,
            msg: msg.to_string(),
            to: e.to,
        }
    }
}

#[wasm_bindgen]
pub fn parse_and_emit(inp: &str, config: Config) -> Result<JsValue, JsValue> {
    utils::init_panic_hook();

    let config: treebuilder::Config = config.into();

    let toks = tokenizer::tokenize(inp, &config).map_err(|e| Error::from(e))?;

    let tree =
        treebuilder::build(&toks, &config).map_err(|e| JsValue::from_str(&e.msg(&toks, inp)))?;

    if let Some(tree) = tree {
        return Ok(tree.emit_json(0).into());
    } else {
        return Err("no tree was returned - most likely a bug".into());
    }
}
