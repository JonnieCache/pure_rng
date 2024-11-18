use std::io::{self, Write};

use pure_rng::PureRng;

/// Sends an endless stream of random bytes to stdout, by hashing consecutive
/// gray-coded integers, ie. a sequence where each term differs by only one bit.
/// This sequence has very low entropy and so it's a good worst-case input to
/// test PureRng's design.
///
/// Intended for benchmarking purposes, eg. pipe into PractRand. The nix flake
/// in this repo includes a shell with PractRand for this purpose.
///
/// Whatever you pass on the command line is used as the seed.
///
/// ```sh
/// cargo run --release --example test_stream -- "my seed value" | RNG_test stdin64 -multithreaded
/// ```
fn main() {
    let rng = match std::env::args().nth(1) {
        Some(arg) => PureRng::new(arg),
        None => PureRng::default(),
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut i: u64 = 0;

    loop {
        // Compute the Gray code
        // see: https://en.wikipedia.org/wiki/Gray_code#Converting_to_and_from_Gray_code
        let gray_i = i ^ (i >> 1);
        let val: u64 = rng.seed(gray_i).gen();

        if handle.write_all(&val.to_ne_bytes()).is_err() {
            break;
        }

        i += 1;
    }
}
