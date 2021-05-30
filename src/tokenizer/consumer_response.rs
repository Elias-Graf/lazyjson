use super::token::Token;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ConsumerResponse {
    pub cons: usize,
    pub tok: Option<Token>,
}
