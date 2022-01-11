use std::slice::SliceIndex;

#[derive(Debug)]
pub struct CharQueue {
    idx: usize,
    chars: Vec<char>,
}

impl CharQueue {
    pub fn new(str: &str) -> CharQueue {
        CharQueue {
            idx: 0,
            chars: str.chars().collect(),
        }
    }

    /// Advance the queue by `amount`.
    pub fn advance_by(&mut self, amount: usize) {
        self.idx += amount;
    }
    /// Get the nextX `amount` of characters. This does **NOT** advance the queue.
    pub fn get_next(&self, amount: usize) -> &[char] {
        &self.chars[self.idx..self.idx + amount]
    }
    pub fn len(&self) -> usize {
        self.chars.len()
    }
    /// Get the next `char` and advance the queue.
    pub fn next(&mut self) -> Option<&char> {
        let char = self.chars.get(self.idx);

        // Only progress if we actually found a char at the location.
        if char.is_some() {
            self.idx += 1;
        }

        char
    }
    /// Get the next `char` **without** advancing the queue.
    pub fn peek(&self) -> Option<&char> {
        self.chars.get(self.idx)
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
    pub fn find_next<P: Fn(&char) -> bool>(&self, predicate: P) -> Option<usize> {
        self.get(self.idx..)?
            .iter()
            .position(predicate)
            .map(|idx| idx + self.idx)
    }
    /// Find the next occurrence of a character.
    pub fn find_next_char(&self, ch: &char) -> Option<usize> {
        self.find_next(|c| c == ch)
    }
    /// Get a part of the underlying `[char]`.
    pub fn get<Idx: SliceIndex<[char]>>(&self, i: Idx) -> Option<&Idx::Output> {
        self.chars.get(i)
    }
    /// Check if there are remaining characters.
    pub fn has_remaining(&self) -> bool {
        self.idx < self.len()
    }
}
