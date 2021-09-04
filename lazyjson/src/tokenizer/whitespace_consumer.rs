use super::{error::TokenizationError, ConsumerResponse};

pub fn whitespace_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    let mut cons = 0;

    for c in inp[offset..].chars() {
        match c {
            c if c.is_whitespace() => cons += 1,
            _ => break,
        }
    }

    Ok(ConsumerResponse { cons, tok: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn non_whitespace() {
        consume_and_expect_length("false", 0);
    }
    #[test]
    pub fn single_space() {
        consume_and_expect_length(" ", 1);
    }
    #[test]
    pub fn multiple_spaces() {
        consume_and_expect_length("   ", 3);
    }
    #[test]
    pub fn tabs() {
        consume_and_expect_length("\t", 1);
    }
    #[test]
    pub fn newline() {
        consume_and_expect_length("\n", 1);
    }
    #[test]
    pub fn at_offset() {
        let r = whitespace_consumer(&String::from("false "), 5).unwrap();
        let e = ConsumerResponse { cons: 1, tok: None };

        assert_eq!(r, e);
    }

    fn consume_and_expect_length(inp: &str, cons: usize) {
        let r = whitespace_consumer(&String::from(inp), 0).unwrap();
        let e = ConsumerResponse { cons, tok: None };

        assert_eq!(r, e);
    }
}
