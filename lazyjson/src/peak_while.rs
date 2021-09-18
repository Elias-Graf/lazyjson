use std::iter::Peekable;

pub struct PeekWhile<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> Iterator for PeekWhile<&mut Peekable<I>, P>
where
    I: Iterator,
    P: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.iter.next_if(&self.predicate)
    }
}

pub trait PeekWhileExt: Sized + Iterator {
    fn peek_while<P>(self, predicate: P) -> PeekWhile<Self, P>
    where
        P: Fn(&Self::Item) -> bool;
}

impl<I: Iterator> PeekWhileExt for &mut Peekable<I> {
    fn peek_while<P>(self, predicate: P) -> PeekWhile<Self, P>
    where
        P: Fn(&I::Item) -> bool,
    {
        PeekWhile {
            iter: self,
            predicate,
        }
    }
}
