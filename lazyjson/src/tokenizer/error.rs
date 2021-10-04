use core::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenizationErrTyp {
    NoInp,
    OutOfBounds,
    UnterminatedStr,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TokenizationErr {
    pub typ: TokenizationErrTyp,
    pub from: usize,
    pub to: usize,
}

impl TokenizationErr {
    pub fn new_no_inp() -> TokenizationErr {
        TokenizationErr {
            typ: TokenizationErrTyp::NoInp,
            from: usize::MAX,
            to: usize::MAX,
        }
    }
    pub fn new_out_of_bounds() -> TokenizationErr {
        TokenizationErr {
            typ: TokenizationErrTyp::OutOfBounds,
            from: usize::MAX,
            to: usize::MAX,
        }
    }
    pub fn new_unterminated_str(from: usize, to: usize) -> TokenizationErr {
        TokenizationErr {
            typ: TokenizationErrTyp::UnterminatedStr,
            from,
            to,
        }
    }
}

impl fmt::Display for TokenizationErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TokenizationErr {}
