PureRng is a [`rand`](https://crates.io/crates/rand)-compatible RNG library for generating repeatable, controlled random values, designed primarily for use in games. It uses a hash function to generate exactly one random value per seed. A convenient API is provided to distribute seed values throughout your program in a hierarchical fashion.

# Usage

```rust
use pure_rng::{PureRng, seq::IteratorPureRandom};

// Create the root rng using an initial seed, typically from an external source.
let rng = PureRng::new(1234);

// Seed two more RNGs with arbitrary labels.
// These effectively have their seed appended to that of the parent.
let rng_a = rng.seed("a convenient label");
let rng_b = rng.seed("a different label");

// Generate a value from the first RNG, consuming it.
let value_a: u32 = rng_a.gen();

// Create two more forks of the second RNG, and generate values from them inline.
let value_b1: i64 = rng_b.seed(1).gen();
let value_b2: f64 = rng_b.seed(2).gen();

// Use your custom types
#[derive(Hash)]
struct Point { x: i32, y: i32 }

let value_from_point: u64 = rng
    .seed(Point { x: 10, y: 12 })
    .gen();

// Use the all the usual `rand` API features
let character = rng
    .seed("character")
    .sample(rand::distributions::Alphanumeric) as char;
```

# Motivation

In games driven by procedural generation it's typically required that all generated content is uniquely determined by the initial "world seed" value. If two players input the same seed and subsequently experience different content, this is called "divergence" and is considered a bug.

Divergence typically arises when a traditional RNG is called a varying number of times. For example:

```rust
use rand::{Rng, SeedableRng, rngs::StdRng};
let rng = StdRng::seed_from_u64(1234);

let mut value: u32 = rng.gen_range(0..10);

for _ in 0..get_int_from_player() {
    value += rng.gen_range(0..10);
}

let subsequent_value: bool = rng.gen();
```

Here, the RNG will be called a different number of times depending on user input. This will cause the subsequent value to differ depending on how the user behaves, causing divergence. This also happens when you as the developer modify your code, adding or removing RNG calls and again causing divergence, breaking your test cases in a frustrating manner.

PureRng lets us do this instead:

```rust
use pure_rng::PureRng;
let rng = PureRng::new(1234);

let mut value: u32 = rng.seed("initial value").gen();

for i in 0..get_int_from_player() {
    value += rng.seed(("increment", i)).gen_range(0..0);
}

let subsequent_value: bool = rng.seed("subsequent value").gen();
```

With PureRng, every value generated is a pure function of the chain of seeds used to generate it. There is no state shared between calls like in a typical RNG, and so they cannot interfere with each other.

# Implementation

Each `PureRng` contains a [`std::hash::Hasher`](https://doc.rust-lang.org/std/hash/trait.Hasher.html). Calling `fork()` clones the generator, hashes the value passed in, and returns the clone. When generating a value, we `finish()` the hasher and take the resulting `u64` as the random data.

Many `rand` functions that return single values still take multiple samples in order to guarantee statistical quality. Therefore PureRng still needs the ability to generate many values from a single seed, even if this is never exposed to the user. It does this by writing generated values back to the hasher, advancing the state.

## Is that cryptographically sound?

No.

## Are the numbers at least random?

Yes. PureRng passes PractRand out to 32 terabytes. The crate includes examples for use with PractRand that test both the iterated hashing that occurs when `rand` takes multiple samples, as well as the output from hashing consecutive integers, a typical use case.

# rand-compatible API

PureRng wraps all the functions you know and love from the [`Rng`](https://docs.rs/rand/0.8.5/rand/trait.Rng.html), [`SliceRandom`](https://docs.rs/rand/0.8.5/rand/seq/trait.SliceRandom.html) and [`IteratorRandom`](https://docs.rs/rand/0.8.5/rand/seq/trait.IteratorRandom.html) traits, the difference being that they consume `self`.  This is what makes PureRNG pure, stopping you from reusing a given instance and so helping to prevent divergence bugs. The method bodies are delegated directly to the original traits.

Note that while the [`Distribution`](https://docs.rs/rand/0.8.5/rand/distributions/trait.Distribution.html) trait is supported as shown in the examples, being as it depends merely on `Rng`, there is currently no `PureDistribution` wrapper that would allow implementators to call `seed()` on the passed rng.

# Versioning

The major and minor components of PureRng version numbers track the rand versions they are compatible with. Patch versions are reserved for local fixes and improvements.

Support is planned for the upcoming 0.9 version of `rand`.

## Using a different Hasher

Everyone has their own favourite hash function. To use yours, disable the default feature and define `PureRng`:

```rust
type PureRng = pure_rng::PureRandomGenerator<MyHasher>;
```

Note that PureRng does nothing to ensure consitent byte order (endianness) in the hashing algorithm across platforms. The default RapidHasher implementation handles this for you, but if your preferred one doesn't then you can wrap it using the [deterministic-hash](https://crates.io/crates/deterministic-hash) crate.


# Serde support

Simply enable the `serde` feature.

# Nix

The repo includes a `flake.nix` file which provides a rust dev environment, as well as a separate `quality_tests` shell which builds a number of randomness-testing tools, including the popular [PractRand](https://pracrand.sourceforge.net/).

```sh
nix develop .#quality_tests
```

# License

MIT.

# Contributing

Go nuts.

# Prior art

* ### [Andrew Clifton - Don't generate, hash! (Or, how I learned to stop worrying and love SplitMix64)](https://www.youtube.com/watch?v=e4b--cyXEsM)
  
  A talk from the Roguelike Celebration conference that presents the concept of deterministic randomness via hashing in the context of games. As described in the title he brings in a traditional RNG alongside the hash function.

* ### [froggy_rand](https://crates.io/crates/froggy-rand)
  
  A rust crate implementing the scheme given in the above presentation. Provides it's own algorithms for things like random values in a range instead of integrating with the more robust `rand` implementations. As above, the SplitMix64 generator is employed.

* ### [rand_seeder](https://github.com/rust-random/seeder)
  
  This crate implements the `rand` API, coming as it does straight from the `rand` project itself. It's built on the SipHash-2.4 hash function and a custom SipRNG algorithm based on the SipHash mixer. The hasher is converted into a SipRNG and that is used to actually generate the random output. It boasts the ability to conserve the entire 256 bits of state from the hasher and pass that into the RNG.  
  
  PureRNG cannot do this - when we finish the hasher we get out only a 64 bit integer, and that is used to advance the state. Information is being thrown away. rand_seeder is able to do this because it has its own custom cryptographic primitives under the hood. PureRNG leans on the `Hasher` trait and that doesn't give us access to the hasher's internal state. As mentioned, this entropy loss doesn't appear to affect PureRNG's statistical quality - plenty of non-cryptographic RNGs get by OK with only 64 bits of state after all.
