use std::slice::SliceIndex;

pub struct Queue<T> {
    idx: usize,
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(items: Vec<T>) -> Queue<T> {
        Queue { idx: 0, items }
    }

    /// Freely retrieve any part of the underlying slice.
    pub fn get<I: SliceIndex<[T]>>(&self, i: I) -> Option<&I::Output> {
        self.items.get(i)
    }

    /// Get the next item and advance the queue.
    pub fn next(&mut self) -> Option<&T> {
        let item = self.items.get(self.idx);

        // Only increase the index if an item was found. Otherwise it would be
        // increased up to infinity once the end was reached.
        if item.is_some() {
            self.idx += 1;
        }

        item
    }

    /// Get the next item but not advance the queue.
    pub fn peek(&self) -> Option<&T> {
        self.items.get(self.idx)
    }

    /// The the current queue position.
    pub fn idx(&self) -> usize {
        self.idx
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_returns_the_current_item_and_advances_the_queue() {
        let mut queue = Queue::new("abc".chars().collect());

        assert_eq!(queue.next(), Some(&'a'));
        assert_eq!(queue.next(), Some(&'b'));
        assert_eq!(queue.next(), Some(&'c'));
        assert_eq!(queue.next(), None);
    }

    #[test]
    fn next_does_not_increase_the_index_after_end_was_reached() {
        let mut queue = Queue::new("".chars().collect());

        assert_eq!(queue.idx(), 0);

        queue.next();

        assert_eq!(queue.idx(), 0);
    }

    #[test]
    fn peek_returns_the_current_item_without_advancing_the_queue() {
        let queue = Queue::new("abc".chars().collect());

        assert_eq!(queue.peek(), Some(&'a'));
        assert_eq!(queue.peek(), Some(&'a'));
        assert_eq!(queue.idx(), 0);
    }

    #[test]
    fn get_returns_the_requested_slice_index() {
        let queue = Queue::new("abc".chars().collect());

        assert_eq!(queue.get(1..3).unwrap(), &['b', 'c']);
        assert_eq!(queue.get(3..4), None);
    }
}
