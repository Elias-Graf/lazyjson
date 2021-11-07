use crate::char_queue::CharQueue;

use super::{error::TokenizationErr, Token};

const OPENING_QUOTE: usize = 1;

pub fn string_literal_consumer(inp: &mut CharQueue) -> Result<Option<Token>, TokenizationErr> {
    let start = inp.peek().ok_or(TokenizationErr::new_out_of_bounds())?;

    if start != '"' {
        return Ok(None);
    }

    let from = inp.idx();
    let val = read_until_string_end(inp)?;
    let to = inp.idx();

    Ok(Some(Token::new_str(&val, from, to)))
}

fn read_until_string_end(inp: &mut CharQueue) -> Result<String, TokenizationErr> {
    let from = inp.idx();
    let mut str = String::new();

    inp.advance_by(OPENING_QUOTE);

    loop {
        let c = inp
            .next()
            .ok_or(TokenizationErr::new_unterminated_str(from, inp.idx()))?;

        match c {
            '"' => break,
            // If we have an escaped character we push it no matter what (for example
            // an escaped quote).
            '\\' => str.push(inp.next().unwrap()),
            _ => str.push(c),
        }
    }

    Ok(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_string() {
        let inp = &mut CharQueue::new("1");

        string_literal_consumer(inp).unwrap();

        assert_eq!(inp.next(), Some('1'));
    }

    #[test]
    fn unterminated() {
        let inp = &mut CharQueue::new("\"Hello, World!");
        let t = string_literal_consumer(inp).unwrap_err();

        assert_eq!(t, TokenizationErr::new_unterminated_str(0, 14));
    }

    #[test]
    fn empty_string() {
        let inp = &mut CharQueue::new("\"\"");
        let t = string_literal_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_str("", 0, 2)));
    }

    #[test]
    fn normal_string() {
        let inp = &mut CharQueue::new("\"Hello, World 👋\"");
        let t = string_literal_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_str("Hello, World 👋", 0, inp.len())));
    }

    #[test]
    fn containing_quotes() {
        let inp = &mut CharQueue::new("\"Hello, \\\"World\\\" 👋\"");
        let t = string_literal_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_str("Hello, \"World\" 👋", 0, inp.len())));
    }

    #[test]
    fn at_offset() {
        let inp = &mut CharQueue::new("   \"Hello 👋\"");
        inp.advance_by(3);

        let t = string_literal_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_str("Hello 👋", 3, 12)));
    }

    #[test]
    fn is_consumed() {
        let inp = &mut CharQueue::new("\"string literal\"1");

        string_literal_consumer(inp).unwrap();

        assert_eq!(inp.next(), Some('1'));
    }
}
