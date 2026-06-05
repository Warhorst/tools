#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Binary, Display, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shr};

/// An array for bit values (0 or 1) which holds all its data in a single integer.
///
/// The following types are supported as a base:
/// - [`u8`]
/// - [`u16`]
/// - [`u32`]
/// - [`u64`]
/// - [`u128`]
/// - [`usize`] (max capacity depends on target architecture)
///
/// Important: For maximum performance, the BitArray performs no bound checks when accessing
/// bit values by index.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BitArray<B: Base>(B);

impl<B> BitArray<B>
where
    B: Base,
{
    /// Create a new [BitArray] from the given base value.
    pub fn new(value: B) -> Self {
        BitArray(value)
    }

    /// Create a new BitArray from an iterator of bool values.
    ///
    /// If the iterator yields fewer elements than [Base::max_len],
    /// any remaining bit will default to false.
    /// If the iterator yields more elements than [Base::max_len],
    /// any additional element will be ignored.
    pub fn from_bits(iter: impl IntoIterator<Item = bool>) -> Self {
        let mut arr = BitArray::default();

        iter.into_iter()
            .take(B::max_len() as usize)
            .enumerate()
            .for_each(|(i, b)| {
                arr.set(i as u8, b);
            });

        arr
    }

    /// Create a new BitArray from an iterator of indexes. The bits at the given
    /// indexes will be true and everything else will be false.
    pub fn from_indices(iter: impl IntoIterator<Item = u8>) -> Self {
        let mut arr = BitArray::default();

        iter.into_iter().for_each(|index| arr.set(index, true));

        arr
    }

    /// Create a new BitArray where every bit is 0.
    pub fn all_zero() -> Self {
        BitArray(B::zero())
    }

    /// Crate a new BitArray where every bit is 1.
    pub fn all_one() -> Self {
        BitArray(B::max())
    }

    /// Get the bit value of the array at the given index.
    pub fn get(
        &self,
        index: u8,
    ) -> bool {
        // Bitwise AND with data and the desired index, which
        // will leave a number with a single bit set or zero if the bit was not set.
        // If anything greater than zero remained, the bit was set (true), otherwise not (false)
        self.0 & (B::one_at_index(index)) > B::zero()
    }

    /// Set the bit at the given index to the given bit.
    pub fn set(
        &mut self,
        index: u8,
        bit: bool,
    ) {
        if bit {
            // perform bitwise OR with a 1 shifted to the desired index, which will switch it to 1
            self.0 |= B::one_at_index(index);
        } else {
            // perform bitwise AND with a number where every bit is switched to 1 except the desired
            // one, which will switch it to 0
            self.0 &= !B::one_at_index(index);
        }
    }

    /// Create an iterator over all the bits of this array
    pub fn iter(&self) -> BitArrayIter<B> {
        // Just copy the array, as it is not that big
        BitArrayIter::new(*self)
    }

    /// Creating an iterator over all the indexes set to 1.
    pub fn ones(&self) -> Ones<B> {
        Ones::new(*self)
    }

    /// Creating an iterator over all the indexes set to 0.
    pub fn zeroes(&self) -> Zeroes<B> {
        Zeroes::new(*self)
    }
}

impl<B> Display for BitArray<B>
where
    B: Base,
{
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

pub struct BitArrayIter<B: Base> {
    array: BitArray<B>,
    counter: u8,
}

impl<B> BitArrayIter<B>
where
    B: Base,
{
    fn new(array: BitArray<B>) -> Self {
        BitArrayIter { array, counter: 0 }
    }
}

impl<B> Iterator for BitArrayIter<B>
where
    B: Base,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == B::max_len() {
            return None;
        }

        let item = self.array.get(self.counter);
        self.counter += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(B::max_len() as usize))
    }
}

/// An iterator yielding the indexes of all 1 values of a [BitArray];
pub struct Ones<B: Base> {
    counter: u8,
    inner: BitArrayIter<B>,
}

impl<B> Ones<B>
where
    B: Base,
{
    fn new(array: BitArray<B>) -> Self {
        Ones {
            counter: 0,
            inner: BitArrayIter::new(array),
        }
    }
}

impl<B> Iterator for Ones<B>
where
    B: Base,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.counter == B::max_len() {
                break None;
            }

            let elem = self.inner.next();
            let index = self.counter;
            self.counter += 1;

            if let Some(bit) = elem
                && bit
            {
                break Some(index);
            }
        }
    }
}

/// An iterator yielding the indexes of all 0 values of a [BitArray];
pub struct Zeroes<B: Base> {
    counter: u8,
    inner: BitArrayIter<B>,
}

impl<B> Zeroes<B>
where
    B: Base,
{
    fn new(array: BitArray<B>) -> Self {
        Zeroes {
            counter: 0,
            inner: BitArrayIter::new(array),
        }
    }
}

impl<B> Iterator for Zeroes<B>
where
    B: Base,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.counter == B::max_len() {
                break None;
            }

            let elem = self.inner.next();
            let index = self.counter;
            self.counter += 1;

            if let Some(bit) = elem
                && !bit
            {
                break Some(index);
            }
        }
    }
}

/// Defines a base for a [BitArray]. This base holds the data for the array.
pub trait Base:
    Copy
    + Default
    + Display
    + Binary
    + Not<Output = Self>
    + BitOrAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + Shr<Output = Self>
    + PartialOrd
    + sealed::BaseSealed
{
    /// Return the maximum amount of bits this base can hold
    fn max_len() -> u8;

    /// Returns the max value of this base.
    fn max() -> Self;

    /// Return the representation of zero for this base
    fn zero() -> Self;

    /// Return the representation of one for this base
    fn one() -> Self;

    /// Return the representation of a one at the given index for this base
    fn one_at_index(index: u8) -> Self;
}

impl sealed::BaseSealed for u8 {}
impl Base for u8 {
    fn max_len() -> u8 {
        8
    }

    fn max() -> Self {
        u8::MAX
    }

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn one_at_index(index: u8) -> Self {
        1 << index
    }
}

impl sealed::BaseSealed for u16 {}
impl Base for u16 {
    fn max_len() -> u8 {
        16
    }

    fn max() -> Self {
        u16::MAX
    }

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn one_at_index(index: u8) -> Self {
        1 << index
    }
}

impl sealed::BaseSealed for u32 {}
impl Base for u32 {
    fn max_len() -> u8 {
        32
    }

    fn max() -> Self {
        u32::MAX
    }

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn one_at_index(index: u8) -> Self {
        1 << index
    }
}

impl sealed::BaseSealed for u64 {}
impl Base for u64 {
    fn max_len() -> u8 {
        64
    }

    fn max() -> Self {
        u64::MAX
    }

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn one_at_index(index: u8) -> Self {
        1 << index
    }
}

impl sealed::BaseSealed for u128 {}
impl Base for u128 {
    fn max_len() -> u8 {
        128
    }

    fn max() -> Self {
        u128::MAX
    }

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn one_at_index(index: u8) -> Self {
        1 << index
    }
}

impl sealed::BaseSealed for usize {}
impl Base for usize {
    #[cfg(target_pointer_width = "64")]
    fn max_len() -> u8 {
        64
    }

    #[cfg(target_pointer_width = "32")]
    fn max_len() -> u8 {
        32
    }

    #[cfg(target_pointer_width = "16")]
    fn max_len() -> u8 {
        16
    }

    fn max() -> Self {
        usize::MAX
    }

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn one_at_index(index: u8) -> Self {
        1 << index
    }
}

mod sealed {
    /// Used to seal the [crate::Base] trait.
    pub trait BaseSealed {}
}

#[cfg(test)]
mod tests {
    use super::BitArray;

    #[test]
    fn from_bits_works() {
        let arr = BitArray::<u8>::from_bits([true, false, false, false, false, true, true, true]);
        assert_eq!(arr.to_string(), "11100001");
    }

    #[test]
    fn from_indexes_works() {
        let arr = BitArray::<u8>::from_indices([0, 5, 6, 7]);
        assert_eq!(arr.to_string(), "11100001")
    }

    #[test]
    fn new_with_too_large_iter_works() {
        let arr = BitArray::<u128>::from_bits([true; 129]);
        assert_eq!(arr, BitArray::all_one())
    }

    #[test]
    fn get_works() {
        let arr = BitArray::<u64>::from_bits([true, false, false, false, false, true, true, true]);

        assert!(arr.get(0));
        assert!(!arr.get(2));
    }

    #[test]
    fn iter_works() {
        let iter = [true, false, false, false, false, true, true, true];
        let arr = BitArray::<u64>::from_bits(iter);

        arr.iter().enumerate().for_each(|(i, b)| {
            if i < iter.len() {
                assert_eq!(
                    iter[i], b,
                    "the first elements must be the same as in the given iter"
                );
            } else {
                assert!(
                    !b,
                    "the remaining elements must be false, as they were not set"
                )
            }
        });
    }

    #[test]
    fn ones_work() {
        let expected = vec![0, 2, 4, 6];
        let arr = BitArray::<u8>::from_bits([true, false, true, false, true, false, true, false]);

        assert_eq!(arr.ones().collect::<Vec<_>>(), expected)
    }

    #[test]
    fn zeroes_work() {
        let expected = vec![1, 3, 5, 7];
        let arr = BitArray::<u8>::from_bits([true, false, true, false, true, false, true, false]);

        assert_eq!(arr.zeroes().collect::<Vec<_>>(), expected)
    }
}
