pub mod index;

use std::hash::Hasher;

use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform},
        WeightedError,
    },
    seq::{IteratorRandom, SliceChooseIter, SliceRandom},
};

use crate::PureRandomGenerator;

/// Wrappers for the [IteratorRandom] trait functions. Implemented on all iterators.
pub trait IteratorPureRandom<H>: Iterator + Sized
where
    H: Hasher + Default + Clone,
{
    /// Choose one element at random from the iterator.
    ///
    /// See [IteratorRandom::choose].
    fn choose(self, mut rng: PureRandomGenerator<H>) -> Option<Self::Item> {
        IteratorRandom::choose(self, &mut rng)
    }

    /// Choose one element at random from the iterator.
    ///
    /// See [IteratorRandom::choose_stable].
    fn choose_stable(self, mut rng: PureRandomGenerator<H>) -> Option<Self::Item> {
        IteratorRandom::choose_stable(self, &mut rng)
    }

    /// Collects values at random from the iterator into a supplied buffer
    /// until that buffer is filled.
    ///
    /// See [IteratorRandom::choose_multiple_fill].
    fn choose_multiple_fill(
        self,
        mut rng: PureRandomGenerator<H>,
        buf: &mut [Self::Item],
    ) -> usize {
        IteratorRandom::choose_multiple_fill(self, &mut rng, buf)
    }

    /// Collects `amount` values at random from the iterator into a vector.
    ///
    /// See [IteratorRandom::choose_multiple].
    fn choose_multiple(
        mut self,
        mut rng: PureRandomGenerator<H>,
        amount: usize,
    ) -> Vec<Self::Item> {
        IteratorRandom::choose_multiple(&mut self, &mut rng, amount)
    }
}

impl<I, H> IteratorPureRandom<H> for I
where
    I: Iterator + Sized,
    H: Hasher + Default + Clone,
{
}

/// Wrappers for the [SliceRandom] trait functions. Implemented on all `[T]` slice types.
pub trait SlicePureRandom<H>
where
    H: Hasher + Default + Clone,
{
    type Item;

    /// Returns a reference to one random element of the slice, or `None` if the
    /// slice is empty.
    ///
    /// See [SliceRandom::choose].
    fn choose(&self, rng: PureRandomGenerator<H>) -> Option<&Self::Item>;

    /// Returns a mutable reference to one random element of the slice, or
    /// `None` if the slice is empty.
    ///
    /// See [SliceRandom::choose_mut].
    fn choose_mut(&mut self, rng: PureRandomGenerator<H>) -> Option<&mut Self::Item>;

    /// Chooses `amount` elements from the slice at random, without repetition,
    /// and in random order.
    ///
    /// See [SliceRandom::choose_multiple].
    fn choose_multiple(
        &self,
        rng: PureRandomGenerator<H>,
        amount: usize,
    ) -> SliceChooseIter<Self, Self::Item>;

    /// Similar to [`SlicePureRandom::choose`], but where the likelihood of each outcome may be
    /// specified.
    ///
    /// See [SliceRandom::choose_weighted].
    fn choose_weighted<F, B, X>(
        &self,
        rng: PureRandomGenerator<H>,
        weight: F,
    ) -> Result<&Self::Item, WeightedError>
    where
        F: Fn(&Self::Item) -> B,
        B: SampleBorrow<X>,
        X: SampleUniform
            + for<'a> ::core::ops::AddAssign<&'a X>
            + ::core::cmp::PartialOrd<X>
            + Clone
            + Default;

    /// Similar to [`SlicePureRandom::choose_mut`], but where the likelihood of each outcome may
    /// be specified.
    ///
    /// See [SliceRandom::choose_weighted_mut].
    fn choose_weighted_mut<F, B, X>(
        &mut self,
        rng: PureRandomGenerator<H>,
        weight: F,
    ) -> Result<&mut Self::Item, WeightedError>
    where
        F: Fn(&Self::Item) -> B,
        B: SampleBorrow<X>,
        X: SampleUniform
            + for<'a> ::core::ops::AddAssign<&'a X>
            + ::core::cmp::PartialOrd<X>
            + Clone
            + Default;

    /// Similar to [`SlicePureRandom::choose_multiple`], but where the likelihood of each element's
    /// inclusion in the output may be specified.
    ///
    /// See [SliceRandom::choose_multiple_weighted].
    fn choose_multiple_weighted<F, X>(
        &self,
        rng: PureRandomGenerator<H>,
        amount: usize,
        weight: F,
    ) -> Result<SliceChooseIter<Self, Self::Item>, WeightedError>
    where
        F: Fn(&Self::Item) -> X,
        X: Into<f64>;

    /// Shuffle a mutable slice in place.
    ///
    /// See [SliceRandom::shuffle].
    fn shuffle(&mut self, rng: PureRandomGenerator<H>);

    /// Shuffle a slice in place, but exit early.
    ///
    /// See [SliceRandom::partial_shuffle].
    fn partial_shuffle(
        &mut self,
        rng: PureRandomGenerator<H>,
        amount: usize,
    ) -> (&mut [Self::Item], &mut [Self::Item]);
}

impl<T, H> SlicePureRandom<H> for [T]
where
    H: Hasher + Default + Clone,
{
    type Item = T;

    fn choose(&self, mut rng: PureRandomGenerator<H>) -> Option<&Self::Item> {
        SliceRandom::choose(self, &mut rng)
    }

    fn choose_mut(&mut self, mut rng: PureRandomGenerator<H>) -> Option<&mut Self::Item> {
        SliceRandom::choose_mut(self, &mut rng)
    }

    fn choose_multiple(
        &self,
        mut rng: PureRandomGenerator<H>,
        amount: usize,
    ) -> SliceChooseIter<Self, Self::Item> {
        SliceRandom::choose_multiple(self, &mut rng, amount)
    }

    fn choose_weighted<F, B, X>(
        &self,
        mut rng: PureRandomGenerator<H>,
        weight: F,
    ) -> Result<&Self::Item, WeightedError>
    where
        F: Fn(&Self::Item) -> B,
        B: SampleBorrow<X>,
        X: SampleUniform
            + for<'a> ::core::ops::AddAssign<&'a X>
            + ::core::cmp::PartialOrd<X>
            + Clone
            + Default,
    {
        SliceRandom::choose_weighted(self, &mut rng, weight)
    }

    fn choose_weighted_mut<F, B, X>(
        &mut self,
        mut rng: PureRandomGenerator<H>,
        weight: F,
    ) -> Result<&mut Self::Item, WeightedError>
    where
        F: Fn(&Self::Item) -> B,
        B: SampleBorrow<X>,
        X: SampleUniform
            + for<'a> ::core::ops::AddAssign<&'a X>
            + ::core::cmp::PartialOrd<X>
            + Clone
            + Default,
    {
        SliceRandom::choose_weighted_mut(self, &mut rng, weight)
    }

    fn choose_multiple_weighted<F, X>(
        &self,
        mut rng: PureRandomGenerator<H>,
        amount: usize,
        weight: F,
    ) -> Result<SliceChooseIter<Self, Self::Item>, WeightedError>
    where
        F: Fn(&Self::Item) -> X,
        X: Into<f64>,
    {
        SliceRandom::choose_multiple_weighted(self, &mut rng, amount, weight)
    }

    fn shuffle(&mut self, mut rng: PureRandomGenerator<H>) {
        SliceRandom::shuffle(self, &mut rng)
    }

    fn partial_shuffle(
        &mut self,
        mut rng: PureRandomGenerator<H>,
        amount: usize,
    ) -> (&mut [Self::Item], &mut [Self::Item]) {
        SliceRandom::partial_shuffle(self, &mut rng, amount)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::IteratorPureRandom;
    use super::SlicePureRandom;

    #[test]
    fn test_iterator() {
        let v = [1, 2, 3, 4, 5];
        let chosen = v.iter().choose(PureRng::default()).unwrap();

        assert!(v.contains(chosen));
    }

    #[test]
    fn test_slice() {
        let v = [1, 2, 3, 4, 5];
        let chosen = &v.choose(PureRng::default()).unwrap();

        assert!(v.contains(chosen));
    }
}
