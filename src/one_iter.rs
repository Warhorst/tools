/// An [`Iterator`] which allows to combine 2 different types of iterators
/// to be represented by one wrapper.
pub enum OneIter<IA, IB> {
    IterA(IA),
    IterB(IB),
}

impl<T, IA, IB> OneIter<IA, IB>
where
    IA: Iterator<Item = T>,
    IB: Iterator<Item = T>,
{
    /// Create an [`OneIter::IterA`] from something that implements [`IntoIterator`].
    pub fn iter_a(iter: impl IntoIterator<Item = T, IntoIter = IA>) -> Self {
        OneIter::IterA(iter.into_iter())
    }

    /// Create an [`OneIter::IterB`] from something that implements [`IntoIterator`].
    pub fn iter_b(iter: impl IntoIterator<Item = T, IntoIter = IB>) -> Self {
        OneIter::IterB(iter.into_iter())
    }
}

impl<IA, IB, T> Iterator for OneIter<IA, IB>
where
    IA: Iterator<Item = T>,
    IB: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OneIter::IterA(i) => i.next(),
            OneIter::IterB(i) => i.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::one_iter::OneIter;

    /// Test if I can return either the items of one iterator or nothing, based on a condition.
    #[test]
    pub fn iter_or_empty_works() {
        let items = if true {
            OneIter::iter_a(vec![1, 2, 3])
        } else {
            OneIter::iter_b([])
        }
        .collect::<Vec<_>>();

        assert_eq!(items, vec![1, 2, 3]);

        let items = if false {
            OneIter::iter_a(vec![1, 2, 3])
        } else {
            OneIter::iter_b([])
        }
        .collect::<Vec<_>>();

        assert_eq!(items, vec![])
    }
}
