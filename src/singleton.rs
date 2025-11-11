use core::iter::FusedIterator;

/// An iterator that produces an element once.
pub struct Singleton<A> {
    pub(crate) elt: Option<A>,
}


impl<A> Iterator for Singleton<A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        self.elt.take()
    }
}

/// Create an iterator that produces `element` once. Allows for promoting a value
/// to an iterator.
pub fn singleton<A>(element: A) -> Singleton<A> {
    Singleton { elt: Some(element) }
}

impl<A> DoubleEndedIterator for Singleton<A>
where
    A: Clone,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }

    #[inline]
    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.fold(init, f)
    }
}

impl<A> ExactSizeIterator for Singleton<A> where A: Clone {}

impl<A> FusedIterator for Singleton<A> where A: Clone {}

pub trait IntoSingleton<A>: Sized {
    fn singleton(self) -> Singleton<A>;
}

impl<A> IntoSingleton<A> for A {
    fn singleton(self) -> Singleton<A> {
        singleton(self)
    }
}