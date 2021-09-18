use std::{iter::Peekable, str::CharIndices};

use super::{error::TokenizationError, Token};

static CLOSING_QUOTE_AND_INCLUSIVE: usize = 2;

pub fn string_literal_consumer(
    inp: &mut Peekable<CharIndices>,
) -> Result<Option<Token>, TokenizationError> {
    if inp.peek().is_none() {
        return Err(TokenizationError::new_out_of_bounds());
    }

    if !is_start_of_string(inp.peek()) {
        return Ok(None);
    }

    let start_of_string = inp.next().unwrap();
    let str = read_until_string_end(inp);
    let val = convert_to_string(&str);
    let from = start_of_string.0;
    let to = str.last().unwrap_or(&(0, ' ')).0 + CLOSING_QUOTE_AND_INCLUSIVE;

    Ok(Some(Token::str(&val, from, to)))
}

fn is_start_of_string(c: Option<&(usize, char)>) -> bool {
    c.unwrap().1 == '"'
}

fn read_until_string_end(inp: &mut Peekable<CharIndices>) -> Vec<(usize, char)> {
    let mut str: Vec<(usize, char)> = Vec::new();

    loop {
        let (i, c) = inp.next().unwrap();

        match c {
            '"' => break,
            // If we have an escaped character we push it no matter what (even
            // if it's a '"' for example).
            '\\' => str.push(inp.next().unwrap()),
            _ => str.push((i, c)),
        }
    }

    str
}

fn convert_to_string(str: &Vec<(usize, char)>) -> String {
    str.iter()
        .fold(String::new(), |s, (_, c)| s + &c.to_string())
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
        // The iterator should not be advanced
        assert_eq!(inp.next().unwrap(), (0, '1'));
    }

    #[test]
    fn checking_does_not_consume() {
        let inp = &mut "1".char_indices().peekable();

        string_literal_consumer(inp).unwrap();

        assert_eq!(inp.next().unwrap(), (0, '1'));
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
        let e = Some(Token::str("Hello, World 👋", 0, val.chars().count()));

        assert_eq!(r, e);
    }

    #[test]
    fn containing_quotes() {
        let val = "\"Hello, \\\"World\\\" 👋\"";
        let inp = &mut val.char_indices().peekable();
        let r = string_literal_consumer(inp).unwrap();
        let e = Some(Token::str("Hello, \"World\" 👋", 0, val.chars().count()));

        assert_eq!(r, e);
    }
}
