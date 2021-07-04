use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    KeywordLiteral,
    NumberLiteral,
    Operator,
    Separator,
    StringLiteral,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub typ: TokenType,
    pub val: String,
}
