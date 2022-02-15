use crate::char_queue::CharQueue;

use super::{error::TokenizationErr, token::Token};

/// Is used as a fallback if no other consumer "felt responsible" for the input.
/// Will consume everything up to the next space (' ').
pub fn unknown_token_consumer(inp: &mut CharQueue) -> Result<Option<Token>, TokenizationErr> {
    let from = inp.idx();

    while let Some(c) = inp.peek() {
        if c == &' ' {
            break;
        }

        inp.next();
    }

    Err(TokenizationErr::new_unknown_token(from, inp.idx()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumes_everything_to_input_end() {
        for str in ["/---/", "$$$"] {
            let inp = &mut CharQueue::new(str);

            assert_eq!(
                unknown_token_consumer(inp),
                Err(TokenizationErr::new_unknown_token(0, str.len())),
            );
        }
    }

    #[test]
    fn consumes_everything_to_the_next_space() {
        let inp = &mut CharQueue::new("$$$ world");

        assert_eq!(
            unknown_token_consumer(inp),
            Err(TokenizationErr::new_unknown_token(0, 3))
        );
    }

    #[test]
    fn at_offset() {
        let inp = &mut CharQueue::new("   ---");
        inp.advance_by(3);

        assert_eq!(
            unknown_token_consumer(inp),
            Err(TokenizationErr::new_unknown_token(3, 6))
        );
    }
}
