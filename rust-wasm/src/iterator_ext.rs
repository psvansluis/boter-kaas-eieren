use std::iter::Peekable;

pub trait IteratorExt: Iterator + Sized {
    fn niet_leeg(self) -> Option<Peekable<Self>> {
        let mut peekable = self.peekable();
        peekable.peek()?;
        Some(peekable)
    }
}

impl<I: Iterator> IteratorExt for I {}
