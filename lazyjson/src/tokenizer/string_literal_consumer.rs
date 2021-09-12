use std::{iter::Peekable, str::CharIndices};

use super::{
    error::{NoInput, TokenizationError},
    ConsumerResponse, Token,
};

#[deprecated(note = "use `string_literal_consumer`")]
pub fn old_string_literal_consumer(
    inp: &String,
    offset: usize,
) -> Result<ConsumerResponse, TokenizationError> {
    // TODO: declare chars as variable
    if inp.chars().nth(offset).unwrap() != '"' {
        return Ok(ConsumerResponse { cons: 0, tok: None });
    }

    let mut cons: usize = 1;
    let mut val = String::new();

    loop {
        if offset + cons >= inp.chars().count() {
            return Err(TokenizationError::new_unterminated_string(
                inp,
                offset + cons,
            ));
        }

        let c = inp.chars().nth(offset + cons).unwrap();

        if c == '\\' {
            let next = inp.chars().nth(offset + cons + 1).unwrap();

            if next == '"' {
                val.push('"');

                cons += 2;
            }
        } else if c == '"' {
            cons += 1;
            break;
        } else {
            val.push(c);
            cons += 1;
        }
    }

    Ok(ConsumerResponse {
        cons,
        tok: Some(Token::str(&val, offset, offset + cons)),
    })
}

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
    let to = str.last().unwrap().0 + CLOSING_QUOTE_AND_INCLUSIVE;

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
    fn valid_at_start() {
        let val = "\"Hello, World 👋\"";
        let inp = &mut val.char_indices().peekable();
        let r = string_literal_consumer(inp).unwrap();
        let e = Some(Token::str("Hello, World 👋", 0, val.chars().count()));

        assert_eq!(r, e);
    }

    #[test]
    fn valid_at_start_with_escaped_quote() {
        let val = "\"Hello, \\\"World\\\" 👋\"";
        let inp = &mut val.char_indices().peekable();
        let r = string_literal_consumer(inp).unwrap();
        let e = Some(Token::str("Hello, \"World\" 👋", 0, val.chars().count()));

        assert_eq!(r, e);
    }
}

#[cfg(test)]
mod old_tests {
    use super::*;

    #[test]
    fn consume_non_string() {
        let r = old_string_literal_consumer(&"0".to_string(), 0).unwrap();
        let e = ConsumerResponse { cons: 0, tok: None };

        assert_eq!(r, e);
    }
    #[test]
    fn consume_empty_string() {
        consume_and_assert_string("\"\"");
    }
    #[test]
    fn consume_string() {
        consume_and_assert_string("\"hello world\"");
    }
    #[test]
    fn consume_string_with_escaped_quote() {
        let r = old_string_literal_consumer(&"\"hello \\\" world\"".to_string(), 0).unwrap();
        let e = ConsumerResponse {
            cons: 16,
            tok: Some(Token::str("hello \" world", 0, 16)),
        };

        assert_eq!(r, e);
    }
    #[test]
    fn consume_unterminated_string() {
        let inp = "\"hello world";
        let r = old_string_literal_consumer(&inp.to_string(), 0).unwrap_err();
        let e = TokenizationError::new_unterminated_string(inp, 12);

        assert_eq!(r, e);
    }

    fn consume_and_assert_string(val: &str) {
        let val = &val.to_string();
        let r = old_string_literal_consumer(val, 0).unwrap();
        let len = val.chars().count();
        let e = ConsumerResponse {
            cons: len,
            tok: Some(Token::str(&val[1..len - 1], 0, len)),
        };
        assert_eq!(r, e);
    }
}
