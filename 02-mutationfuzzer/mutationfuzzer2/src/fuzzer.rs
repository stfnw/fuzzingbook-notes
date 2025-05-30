// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use crate::rng::Rng;

/// Represents the structure that the fuzzer operates on. Here we use a
/// dedicated newtype instead of a type alias for being able to implement
/// integrated printing routines.
#[derive(Clone, Debug)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Convert a `&str` to `Bytes`. I choose to do it this way and not use
    /// `FromStr` trait since that returns a Result which has to be unwrapped.
    /// This is unnecessary since in this case the conversion can never fail
    /// (Vec<u8> is a super-set of &str).
    pub fn from_str(s: &str) -> Self {
        Self(s.as_bytes().to_vec())
    }
}

// This assumes that the string is valid utf8.
impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

pub trait Fuzzer {
    fn fuzz(&self, rng: &mut Rng) -> Bytes;
}

pub struct MutationFuzzer {
    /// Initial population passed upon fuzzer construction. First fuzz
    /// candidates are drawn from here, before we start mutating.
    initial_population: Vec<Bytes>,

    /// Number of times this fuzzer has produced output. We use interior
    /// mutability here in order to not violate the trait definition and keep
    /// the `self` passed to `fuzz` immutable.
    nfuzzed: std::cell::RefCell<usize>,
}

impl MutationFuzzer {
    pub fn new(seed: Vec<Bytes>) -> Self {
        Self {
            initial_population: seed,
            nfuzzed: std::cell::RefCell::new(0),
        }
    }

    pub fn fuzz_(&self, rng: &mut Rng, min_mutations: usize, max_mutations: usize) -> Bytes {
        let nfuzzed = *self.nfuzzed.borrow();
        *self.nfuzzed.borrow_mut() = nfuzzed + 1;
        if nfuzzed < self.initial_population.len() {
            self.initial_population[nfuzzed].clone()
        } else {
            self.create_candidate(rng, min_mutations, max_mutations)
        }
    }

    fn create_candidate(&self, rng: &mut Rng, min_mutations: usize, max_mutations: usize) -> Bytes {
        let mut candidate = rng.choice(&self.initial_population).clone();
        let trials = rng.range(min_mutations as u64, max_mutations as u64);
        for _ in 0..trials {
            candidate = mutate(rng, candidate);
        }
        candidate
    }
}

impl Fuzzer for MutationFuzzer {
    fn fuzz(&self, rng: &mut Rng) -> Bytes {
        self.fuzz_(rng, 2, 10 + 1)
    }
}

/// Choose a random mutation strategy and apply it to the input.
pub fn mutate(rng: &mut Rng, s: Bytes) -> Bytes {
    match rng.int(3) {
        0 => delete_random_character(rng, s),
        1 => insert_random_character(rng, s),
        2 => flip_random_bit(rng, s),
        _ => panic!("Can't happen"),
    }
}

fn delete_random_character(rng: &mut Rng, mut s: Bytes) -> Bytes {
    if s.0.is_empty() {
        s
    } else {
        let pos = rng.int(s.0.len() as u64) as usize;
        s.0.remove(pos);
        s
    }
}

fn insert_random_character(rng: &mut Rng, mut s: Bytes) -> Bytes {
    let pos = rng.int((s.0.len() + 1) as u64) as usize;
    let chr = rng.range(32, 127 + 1) as u8;
    s.0.insert(pos, chr);
    s
}

fn flip_random_bit(rng: &mut Rng, mut s: Bytes) -> Bytes {
    let pos = rng.int(s.0.len() as u64) as usize;
    let bit = 1 << rng.int(7);
    s.0[pos] ^= bit;
    s
}
