use std::slice::SliceIndex;

#[derive(Debug)]
pub struct CharQueue {
    idx: usize,
    str: String,
}

impl CharQueue {
    pub fn new(str: &str) -> CharQueue {
        CharQueue {
            str: str.to_string(),
            idx: 0,
        }
    }
    pub fn new_with_idx(str: &str, idx: usize) -> CharQueue {
        let mut queue = CharQueue::new(str);
        queue.idx = idx;

        queue
    }

    /// Advance the queue by `amount`.
    pub fn advance_by(&mut self, amount: usize) {
        self.idx += amount;
    }
    /// Get the nextX `amount` of characters. This does **NOT** advance the queue.
    pub fn get_next(&self, amount: usize) -> &str {
        &self.str[self.idx..self.idx + amount]
    }
    // Get the byte length of the underlying `str`.
    pub fn len(&self) -> usize {
        self.str.len()
    }
    /// Get the next `char`. This advances the queue.
    pub fn next(&mut self) -> Option<char> {
        let char = self.str.chars().nth(self.idx);

        // Only progress if we actually found a char at the location.
        if char.is_some() {
            self.idx += 1;
        }

        char
    }
    /// Get the next `char` without advancing the queue.
    pub fn peek(&self) -> Option<char> {
        self.str.chars().nth(self.idx)
    }
    /// Get the length of the remaining characters.
    pub fn remaining(&self) -> usize {
        self.len() - self.idx
    }
    /// Get the position (index) of the queue.
    pub fn idx(&self) -> usize {
        self.idx
    }
    /// Find the next character using a closure, starting at the current queue position.
    /// This does **NOT** advance the queue.
    pub fn find_next_by_closure<F: Fn(char) -> bool>(&self, clos: F) -> Option<usize> {
        self.str[self.idx..].find(clos).map(|i| i + self.idx)
    }
    /// Find the next occurrence of a character.
    pub fn find_next_by_char(&self, ch: char) -> Option<usize> {
        self.find_next_by_closure(|c| c == ch)
    }
    /// Exposes get of the underlying `str`.
    pub fn get<I: SliceIndex<str>>(&self, i: I) -> Option<&I::Output> {
        self.str.get(i)
    }
    /// Check if there are remaining characters.
    pub fn has_remaining(&self) -> bool {
        self.idx < self.len()
    }
}
