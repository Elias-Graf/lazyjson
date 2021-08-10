use std::fmt;

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
    pub to: usize,
    pub from: usize,
    pub typ: TokenType,
    #[wasm_bindgen(skip)]
    pub val: String,
}

#[wasm_bindgen]
impl Token {
    /// Create a token of type [`TokenType::KeywordLiteral`].
    pub fn kwd(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::KeywordLiteral,
            val: val.into(),
        }
    }
    /// Create a token of type [`TokenType::NumberLiteral`].
    pub fn num(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::NumberLiteral,
            val: val.into(),
        }
    }
    /// Create a token of type [`TokenType::Operator`].
    pub fn op(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::Operator,
            val: val.into(),
        }
    }
    /// Create a token of type [`TokenType::Separator`].
    pub fn sep(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::Separator,
            val: val.into(),
        }
    }
    /// Create a token of type [`TokenType::StringLiteral`].
    pub fn str(val: &str, from: usize, to: usize) -> Token {
        Token {
            from,
            to,
            typ: TokenType::StringLiteral,
            val: val.into(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn val(&self) -> String {
        self.val.clone()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token(typ: {:?}, val: \"{}\")", self.typ, self.val)
    }
}
