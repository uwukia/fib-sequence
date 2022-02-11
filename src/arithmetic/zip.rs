use std::ops::Deref;

pub struct DefaultZip<A, B, C> where
    A: Iterator<Item = C>,
    B: Iterator<Item = C>,
    C: Deref<Target = u32>,
{
    a: A,
    b: B,
}

impl<A, B, C> DefaultZip<A, B, C> where
    A: Iterator<Item = C>,
    B: Iterator<Item = C>,
    C: Deref<Target = u32>,
{
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B, C> Iterator for DefaultZip<A, B, C> where
    A: Iterator<Item = C>,
    B: Iterator<Item = C>,
    C: Deref<Target = u32>,
{
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next()) {
            (Some(x), Some(y)) => Some((*x, *y)),
            (Some(x), None)    => Some((*x, 0u32)),
            (None,    Some(y)) => Some((0u32, *y)),
            (None,    None)    => None,
        }
    }
}