#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TokenType {
    KeywordLiteral,
    Operator,
    Separator,
    StringLiteral,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub val: String,
}
