pub mod seq;

use std::hash::{Hash, Hasher};

use rand::{
    distributions::{
        self,
        uniform::{SampleRange, SampleUniform},
        Distribution, Standard,
    },
    Fill, Rng, RngCore,
};

#[cfg(feature = "rapidhash")]
pub type PureRng = PureRandomGenerator<rapidhash::RapidHasher>;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PureRandomGenerator<H>
where
    H: Hasher + Default + Clone,
{
    hasher: H,
}

impl<H> PureRandomGenerator<H>
where
    H: Hasher + Default + Clone,
{
    /// Creates a new generator with the given hashable value as the seed. In a
    /// game this might be the world seed, or just the system time etc.
    ///
    /// # Examples
    ///
    /// ```
    /// use pure_rng::PureRng;
    ///
    /// let rng = PureRng::new("initial seed");
    /// ```
    pub fn new(hashable: impl Hash) -> Self {
        Self::default().seed(hashable)
    }

    /// Forks the generator, and advances the fork's state by hashing the given
    /// value.
    ///
    /// This is the core of the API - sometimes called "splitting" or "forking"
    /// an RNG. The difference is that with PureRng you split every time you
    /// generate a new value.
    ///  
    /// # Examples
    ///
    /// ```
    /// use pure_rng::PureRng;
    ///
    /// let rng = PureRng::new("initial seed");
    /// let sub_rng = rng.seed("a convenient label to differentiate");
    /// let ten_values: Vec<u64> = (0..10).map(|i| sub_rng.seed(i).gen()).collect();
    ///
    /// #[derive(Hash)]
    /// struct Point { x: i32, y: i32 }
    ///
    /// let value_from_point: u64 = rng
    ///     .seed(Point { x: 10, y: 12 })
    ///     .gen();
    /// ```
    pub fn seed(&self, hashable: impl Hash) -> Self {
        let mut fork = self.clone();
        hashable.hash(&mut fork.hasher);

        fork
    }
}

impl<H> RngCore for PureRandomGenerator<H>
where
    H: Hasher + Default + Clone,
{
    fn next_u32(&mut self) -> u32 {
        // Good enough for government work.
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        // Get the random value by finishing the hasher.
        let val = self.hasher.finish();

        // Write the value back into the hasher to advance the state. Continuing
        // to use the hasher after you've finished it is explicitly supported.
        self.hasher.write_u64(val);

        val
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

/// Wrappers for the [`Rng`] trait functions.
impl<H> PureRandomGenerator<H>
where
    H: Hasher + Default + Clone,
{
    /// Return a random value supporting the [`Standard`] distribution.
    ///
    /// See [`Rng::gen`].
    #[inline]
    pub fn gen<T>(mut self) -> T
    where
        Standard: Distribution<T>,
    {
        Rng::gen(&mut self)
    }

    /// Generate a random value in the given range.
    ///
    /// See [`Rng::gen_range`].
    pub fn gen_range<T, Q>(mut self, range: Q) -> T
    where
        T: SampleUniform,
        Q: SampleRange<T>,
    {
        Rng::gen_range(&mut self, range)
    }

    /// Sample a new value, using the given distribution.
    ///
    /// See [`Rng::sample`].
    pub fn sample<T, D: Distribution<T>>(mut self, distr: D) -> T {
        Rng::sample(&mut self, distr)
    }

    /// Create an iterator that generates values using the given distribution.
    ///
    /// See [`Rng::sample_iter`].
    pub fn sample_iter<T, D>(self, distr: D) -> distributions::DistIter<D, Self, T>
    where
        D: Distribution<T>,
        Self: Sized,
    {
        Rng::sample_iter(self, distr)
    }

    /// Fill any type implementing [`Fill`] with random data.
    ///
    /// See [`Rng::fill`].
    pub fn fill<T: Fill + ?Sized>(mut self, dest: &mut T) {
        Rng::fill(&mut self, dest)
    }

    /// Fill any type implementing [`Fill`] with random data
    ///
    /// See [`Rng::try_fill`].
    pub fn try_fill<T: Fill + ?Sized>(mut self, dest: &mut T) -> Result<(), rand::Error> {
        Rng::try_fill(&mut self, dest)
    }

    /// Return a bool with a probability `p` of being true.
    ///
    /// See [`Rng::gen_bool`].
    #[inline]
    pub fn gen_bool(mut self, p: f64) -> bool {
        Rng::gen_bool(&mut self, p)
    }

    /// Return a bool with a probability of `numerator/denominator` of being true.
    ///
    /// See [`Rng::gen_ratio`].
    #[inline]
    pub fn gen_ratio(mut self, numerator: u32, denominator: u32) -> bool {
        Rng::gen_ratio(&mut self, numerator, denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeatable() {
        let rng = PureRng::default();

        let val_1: u64 = rng.seed("lol").gen();
        let val_2: u64 = rng.seed("lol").gen();
        assert_eq!(val_1, val_2);

        let rng_2 = rng.seed("foo");

        let val_3: u64 = rng_2.seed("lol").gen();
        let val_4: u64 = rng_2.seed("lol").gen();
        assert_eq!(val_3, val_4);

        assert_ne!(val_1, val_3);

        let rng_3 = rng_2.seed("bar");

        let val_5: u64 = rng_3.seed("lol").gen();
        let val_6: u64 = rng_3.seed("lol").gen();
        assert_eq!(val_5, val_6);

        assert_ne!(val_3, val_5);
        assert_ne!(val_2, val_5);
    }
}
