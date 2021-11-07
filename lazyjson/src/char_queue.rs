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
        &self
            .get(self.idx..self.idx + amount)
            .expect(&format!("index {} + {} out of bounds", self.idx, amount))
            .iter()
            .collect::<String>()
    }
    // Get the byte length of the underlying `str`.
    pub fn len(&self) -> usize {
        self.str.chars().count()
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
    pub fn find_next_by_closure<P: Fn(&char) -> bool>(&self, predicate: P) -> Option<usize> {
        self.get(self.idx..)?.iter().position(predicate)
    }
    /// Find the next occurrence of a character.
    pub fn find_next_by_char(&self, ch: &char) -> Option<usize> {
        self.find_next_by_closure(|c| c == ch)
    }
    /// Get a part of the underlying `[char]`.
    pub fn get<I: SliceIndex<[char]>>(&self, i: I) -> Option<&I::Output> {
        self.str.chars().collect::<Vec<char>>().get(i)
    }
    /// Check if there are remaining characters.
    pub fn has_remaining(&self) -> bool {
        self.idx < self.len()
    }
}
