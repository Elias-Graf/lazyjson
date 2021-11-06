use crate::char_queue::CharQueue;

use super::{error::TokenizationErr, Token};

const START_OF_LINE_COMMENT: usize = 2;

pub fn line_comment_consumer(inp: &mut CharQueue) -> Result<Option<Token>, TokenizationErr> {
    if !is_start_of_line_comment(inp) {
        return Ok(None);
    }

    let from = inp.idx();

    inp.advance_by(START_OF_LINE_COMMENT);

    let to = inp.find_next_by_char('\n').unwrap_or(inp.len());
    let val = inp.get(from + START_OF_LINE_COMMENT..to).unwrap();

    Ok(Some(Token::new_line_comment(val, from, to)))
}

fn is_start_of_line_comment(inp: &mut CharQueue) -> bool {
    // Check if we can actually get the next two characters.
    if inp.remaining() < 2 {
        return false;
    }

    let start = inp.get_next(2);

    start == "//"
}

#[cfg(test)]
mod tests {
    use crate::{char_queue::CharQueue, tokenizer::Token};

    use super::line_comment_consumer;

    #[test]
    fn non_comment() {
        let inp = &mut CharQueue::new("1");
        let t = line_comment_consumer(inp).unwrap();

        assert_eq!(t, None);
    }

    #[test]
    fn line_comment_at_start() {
        let inp = &mut CharQueue::new("//todo: good code");
        let t = line_comment_consumer(inp).unwrap();

        assert_eq!(
            t,
            Some(Token::new_line_comment("todo: good code", 0, inp.len()))
        )
    }

    #[test]
    fn line_comment_not_at_start() {
        let inp = &mut CharQueue::new_with_idx("false// should be true", 5);
        let t = line_comment_consumer(inp).unwrap();

        assert_eq!(t, Some(Token::new_line_comment(" should be true", 5, 22)))
    }

    #[test]
    fn line_comment_terminated_with_new_line() {
        let inp = &mut CharQueue::new("// config is on next line\nfalse");
        let t = line_comment_consumer(inp).unwrap();

        assert_eq!(
            t,
            Some(Token::new_line_comment(" config is on next line", 0, 25))
        )
    }

    #[test]
    fn queue_is_correctly_advanced() {
        unimplemented!()
    }
}
