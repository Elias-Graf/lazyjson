use std::{iter::Peekable, str::CharIndices};

use super::{error::TokenizationError, Token};

pub fn string_literal_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationError> {
    if inp.peek().is_none() {
        return Err(TokenizationError::new_out_of_bounds());
    }

    if !is_start_of_string(inp.peek()) {
        return Ok(None);
    }

    return read_until_string_end(inp);
}

fn is_start_of_string(c: Option<&(usize, char)>) -> bool {
    c.unwrap().1 == '"'
}

fn read_until_string_end(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationError> {
    let (from, _) = inp.next().unwrap();
    let mut str = String::new();

    loop {
        // let next = inp.next().expect("unterminated string, aalsdfjasdf");
        let next = inp.next().ok_or(TokenizationError::new_unterminated_str(
            from,
            from + str.len(),
        ))?;
        let (i, c) = next;

        println!("{} {}", i, c);

        match c {
            '"' => return Ok(Some(Token::str(&str, from, i + 1))),
            // If we have an escaped character we push it no matter what (even
            // if it's a '"' for example).
            '\\' => str.push(inp.next().unwrap().1),
            _ => str.push(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let inp = &mut "".char_indices().peekable();
        let r = string_literal_consumer(inp).unwrap_err();
        let e = TokenizationError::new_out_of_bounds();

        assert_eq!(r, e);
    }

    #[test]
    fn non_string() {
        let inp = &mut "1".char_indices().peekable();
        let r = string_literal_consumer(inp).unwrap();
        let e = None;

        assert_eq!(r, e);
        assert_eq!(inp.next().unwrap(), (0, '1'));
    }

    #[test]
    fn unterminated() {
        let inp = "\"Hello, World!";
        let r = string_literal_consumer(&mut inp.char_indices().peekable()).unwrap_err();
        let e = TokenizationError::new_unterminated_str(0, 13);

        assert_eq!(r, e);
    }

    #[test]
    fn empty_string() {
        let val = "\"\"";
        let inp = &mut val.char_indices().peekable();
        let r = string_literal_consumer(inp).unwrap();
        let e = Some(Token::str("", 0, 2));

        assert_eq!(r, e);
    }

    #[test]
    fn normal_string() {
        let val = "\"Hello, World 👋\"";
        let inp = &mut val.char_indices().peekable();
        let r = string_literal_consumer(inp).unwrap();
        let e = Some(Token::str("Hello, World 👋", 0, val.len()));

        assert_eq!(r, e);
    }

    #[test]
    fn containing_quotes() {
        let val = "\"Hello, \\\"World\\\" 👋\"";
        let inp = &mut val.char_indices().peekable();
        let r = string_literal_consumer(inp).unwrap();
        let e = Some(Token::str("Hello, \"World\" 👋", 0, val.len()));

        assert_eq!(r, e);
    }
}
