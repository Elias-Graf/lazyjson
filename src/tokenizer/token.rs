#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TokenType {
    KeywordLit,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub val: String,
}
