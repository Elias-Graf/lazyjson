use crate::char_queue::CharQueue;

use super::{error::TokenizationErr, Token};

const START_OF_LINE_COMMENT: usize = 2;
const NEW_LINE: usize = 1;

pub fn line_comment_consumer(queue: &mut CharQueue) -> Result<Option<Token>, TokenizationErr> {
    if !is_start_of_line_comment(queue) {
        return Ok(None);
    }

    let from = queue.idx();
    let to = queue.find_next_char(&'\n').unwrap_or(queue.len());

    let val: String = queue
        .get(from + START_OF_LINE_COMMENT..to)
        .unwrap()
        .into_iter()
        .collect();

    queue.advance_by(to - from + NEW_LINE);

    Ok(Some(Token::new_line_comment(&val, from, to)))
}

fn is_start_of_line_comment(inp: &mut CharQueue) -> bool {
    // Check if we can actually get the next two characters.
    if inp.remaining() < 2 {
        return false;
    }

    let start: String = inp.get_next(2).into_iter().collect();

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
    fn comment_at_start() {
        let queue = &mut CharQueue::new("//todo: good code");

        assert_eq!(
            line_comment_consumer(queue),
            Ok(Some(Token::new_line_comment(
                "todo: good code",
                0,
                queue.len()
            )))
        )
    }

    #[test]
    fn comment_not_at_start() {
        let queue = &mut CharQueue::new("false// should be true");
        queue.advance_by(5);

        assert_eq!(
            line_comment_consumer(queue),
            Ok(Some(Token::new_line_comment(" should be true", 5, 22))),
        )
    }

    #[test]
    fn comment_terminated_with_new_line() {
        let queue = &mut CharQueue::new("// config is on next line\nfalse");

        assert_eq!(
            line_comment_consumer(queue),
            Ok(Some(Token::new_line_comment(
                " config is on next line",
                0,
                25
            ))),
        )
    }

    #[test]
    fn comment_correctly_consumed() {
        let mut queue = CharQueue::new("// comment\n1");

        let _ = line_comment_consumer(&mut queue);

        assert_eq!(queue.next(), Some(&'1'))
    }
}
