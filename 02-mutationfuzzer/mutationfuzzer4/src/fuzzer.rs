// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use crate::coverage::{run_and_get_coverage, Coverage, CumulativeCoverage, RunResult};
use crate::rng::Rng;

use std::collections::BTreeSet;

/// Represents the structure that the fuzzer operates on. Here we use a
/// dedicated newtype instead of a type alias for being able to implement
/// integrated printing routines.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Input(Vec<u8>);

impl Input {
    /// Convert a `&str` to `Input`. I choose to do it this way and not use
    /// `FromStr` trait since that returns a Result which has to be unwrapped.
    /// This is unnecessary since in this case the conversion can never fail
    /// (Vec<u8> is a super-set of &str).
    pub fn from_str(s: &str) -> Self {
        Self(s.as_bytes().to_vec())
    }
}

// This assumes that the string is valid utf8.
impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}

pub struct MutationCoverageFuzzer {
    /// The size of the initial population. This is need for distinguishing
    /// when `fuzz` should draw from the initial population vs start mutating.
    initial_population_size: usize,

    /// Currently evolved population.
    population: Vec<Input>,
    population_set: BTreeSet<Input>,

    /// Number of times a random input was tested.
    fuzz_cases: usize,
}

impl MutationCoverageFuzzer {
    pub fn new(seed: Vec<Input>) -> Self {
        Self {
            initial_population_size: seed.len(),
            population: seed.clone(),
            population_set: seed.into_iter().collect(),
            fuzz_cases: 0,
        }
    }

    pub fn fuzz(&mut self, rng: &mut Rng) -> Input {
        self.fuzz_(rng, 2, 10 + 1)
    }

    pub fn fuzz_(&mut self, rng: &mut Rng, min_mutations: usize, max_mutations: usize) -> Input {
        self.fuzz_cases += 1;
        if self.fuzz_cases - 1 < self.initial_population_size {
            self.population[self.fuzz_cases - 1].clone()
        } else {
            self.create_candidate(rng, min_mutations, max_mutations)
        }
    }

    fn create_candidate(&self, rng: &mut Rng, min_mutations: usize, max_mutations: usize) -> Input {
        let mut candidate = rng.choice(&self.population).clone();
        let trials = rng.range(min_mutations as u64, max_mutations as u64);
        for _ in 0..trials {
            candidate = mutate(rng, candidate);
        }
        candidate
    }

    pub fn population(&self) -> Vec<Input> {
        self.population.clone()
    }

    pub fn runs(&mut self, rng: &mut Rng, n: usize) -> (Coverage, CumulativeCoverage) {
        // Current coverage (union of all coverages during execution; set of
        // unique locations).
        let mut coverage: Coverage = BTreeSet::new();

        // Trace of coverage numbers (size of coverage) accross all fuzzer runs.
        let mut cumulative_coverage: CumulativeCoverage = Vec::new();

        for _ in 0..n {
            let input = self.fuzz(rng);

            let (runcoverage, runoutcome) = run_and_get_coverage(&input);

            if runoutcome == RunResult::Pass
                && !runcoverage
                    .difference(&coverage)
                    .collect::<Vec<_>>()
                    .is_empty()
            {
                if !self.population_set.contains(&input) {
                    self.population_set.insert(input.clone());
                    self.population.push(input.clone());
                }

                coverage.extend(runcoverage);
            }

            cumulative_coverage.push(coverage.len());
            assert!(
                self.fuzz_cases == cumulative_coverage.len(),
                "{} != {}:\n     {:?}",
                self.fuzz_cases,
                cumulative_coverage.len(),
                cumulative_coverage
            );
        }

        (coverage, cumulative_coverage)
    }
}

/// Choose a random mutation strategy and apply it to the input.
pub fn mutate(rng: &mut Rng, s: Input) -> Input {
    match rng.int(3) {
        0 => delete_random_character(rng, s),
        1 => insert_random_character(rng, s),
        2 => flip_random_bit(rng, s),
        _ => panic!("Can't happen"),
    }
}

fn delete_random_character(rng: &mut Rng, mut s: Input) -> Input {
    if s.0.is_empty() {
        s
    } else {
        let pos = rng.int(s.0.len() as u64) as usize;
        s.0.remove(pos);
        s
    }
}

fn insert_random_character(rng: &mut Rng, mut s: Input) -> Input {
    let pos = rng.int((s.0.len() + 1) as u64) as usize;
    let chr = rng.range(32, 127 + 1) as u8;
    s.0.insert(pos, chr);
    s
}

fn flip_random_bit(rng: &mut Rng, mut s: Input) -> Input {
    let pos = rng.int(s.0.len() as u64) as usize;
    let bit = 1 << rng.int(7);
    s.0[pos] ^= bit;
    s
}
