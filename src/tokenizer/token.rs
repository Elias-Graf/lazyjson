#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TokenType {
    KeywordLiteral,
    Operator,
    Separator,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub val: String,
}
