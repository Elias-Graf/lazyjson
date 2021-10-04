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

    pub fn msg(&self, inp: &str) -> String {
        match self.typ {
            TokenizationErrTyp::NoInp => "tokenizer did not receive any input".to_string(),
            TokenizationErrTyp::OutOfBounds => {
                "tried to tokenize outside of input bounds (internal error)".to_string()
            }
            TokenizationErrTyp::UnterminatedStr => format!(
                "unterminated string from {}, to {} ('{}'<--)",
                self.from,
                self.to,
                &inp[self.from..self.to]
            ),
        }
    }
}

impl fmt::Display for TokenizationErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TokenizationErr {}

#[cfg(test)]
mod tests {
    use super::TokenizationErr;

    #[test]
    fn no_inp_msg() {
        let msg = TokenizationErr::new_no_inp().msg("");

        assert_eq!(msg, "tokenizer did not receive any input");
    }

    #[test]
    fn out_of_bounds_msg() {
        let msg = TokenizationErr::new_out_of_bounds().msg("");

        assert_eq!(
            msg,
            "tried to tokenize outside of input bounds (internal error)"
        );
    }

    #[test]
    fn unterminated_str_msg() {
        let inp = "\"Hello, World 👋";
        let from = 0;
        let to = inp.len();
        let msg = TokenizationErr::new_unterminated_str(from, to).msg(inp);

        assert_eq!(
            msg,
            format!(
                "unterminated string from {}, to {} ('{}'<--)",
                from, to, "\"Hello, World 👋"
            )
        );
    }
}
