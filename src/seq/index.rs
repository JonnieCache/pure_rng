//! Wrappers for the [`rand::seq::index`] functions.

use rand::distributions::WeightedError;
use rand::seq::index::IndexVec;

use crate::PureRng;

/// Randomly sample exactly `amount` distinct indices from `0..length`, and
/// return them in random order (fully shuffled).
///
/// See [`rand::seq::index::sample`]
pub fn sample(mut rng: PureRng, length: usize, amount: usize) -> IndexVec {
    rand::seq::index::sample(&mut rng, length, amount)
}

/// Randomly sample exactly `amount` distinct indices from `0..length`, and
/// return them in an arbitrary order (there is no guarantee of shuffling or
/// ordering).
///
/// See [`rand::seq::index::sample_weighted`]
pub fn sample_weighted<F, X>(
    mut rng: PureRng,
    length: usize,
    weight: F,
    amount: usize,
) -> Result<IndexVec, WeightedError>
where
    F: Fn(usize) -> X,
    X: Into<f64>,
{
    rand::seq::index::sample_weighted(&mut rng, length, weight, amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let rng = PureRng::default();
        let length = 10;
        let amount = 5;
        let indices = sample(rng, length, amount);

        assert_eq!(indices.len(), amount);

        for i in indices.iter() {
            assert!((0..length).contains(&i));
        }
    }

    #[test]
    fn test_sample_weighted() {
        let rng = PureRng::default();
        let length = 10;
        let amount = 5;
        let weight = |i| i as f64;
        let indices = sample_weighted(rng, length, weight, amount).unwrap();

        assert_eq!(indices.len(), amount);

        for i in indices.iter() {
            assert!((0..length).contains(&i));
        }
    }
}
