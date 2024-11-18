use std::{
    hash::{Hash, Hasher},
    io::{self, Write},
};

use rapidhash::RapidHasher;

/// Sends an endless stream of random bytes to stdout, using the recursive
/// hashing scheme employed inside PureRng. This is intended for benchmarking
/// the behaviour when a single PureRng instance is used to take multiple
/// samples, which often happens inside `Rng` trait methods. It can be used much
/// like the stream example, ie. piped into PractRand.
///
/// ```sh
/// cargo run --release --example test_recursive_hashing -- "my seed value" | RNG_test stdin64 -multithreaded
/// ```
fn main() {
    let mut hasher = RapidHasher::default();
    if let Some(arg) = std::env::args().nth(1) {
        arg.hash(&mut hasher);
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut i: u64;

    loop {
        i = hasher.finish();
        hasher.write_u64(i);

        if handle.write_all(&i.to_le_bytes()).is_err() {
            break;
        }
    }
}
