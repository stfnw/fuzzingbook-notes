// SPDX-FileCopyrightText: 2019 Rough structure: adapted from gamozo https://github.com/gamozolabs/guifuzz
// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use crate::rng::Rng;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::process;

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

/// Statistics relevant during fuzzing.
#[derive(Default)]
pub struct Statistics {
    /// Number of times a random input was tested.
    pub fuzz_cases: usize,

    /// Coverage database. Associates each unique coverage set with the
    /// corresponding input that caused it.
    pub coverage_db: BTreeMap<Coverage, Input>,

    /// Union of all coverages that were achieved during execution.
    pub coverage_all: Coverage,

    /// Set of all inputs with unique coverage.
    pub population_set: BTreeSet<Input>,

    /// List of all inputs with unique coverage.
    pub population_list: Vec<Input>,

    /// The size of the initial population. This is need for distinguishing
    /// when `fuzz` should draw from the initial population vs start mutating.
    pub initial_population_size: usize,

    /// Trace of coverage numbers (size of coverage_db) accross all fuzzer runs.
    pub cumulative_coverage: Vec<usize>,
}

impl Statistics {
    pub fn new(seed: Vec<Input>) -> Self {
        let mut res = Self::default();
        res.population_set = seed.clone().into_iter().collect();
        res.population_list = seed;
        res
    }
}

/// Create and run `n` random fuzz cases and record statistics during execution.
pub fn run(rng: &mut Rng, stats: &mut Statistics, n: usize) {
    for _ in 0..n {
        let input = fuzz(rng, stats);

        // Ignore input (and don't perform unnecessary expensive re-evaluation)
        // in case we have seen that exact input before.
        if stats.population_set.contains(&input) {
            continue;
        }

        let (runcoverage, runoutcome) = run_and_get_coverage(&input);

        // Check if the obtained coverage contains new entries / is interesting.
        if runoutcome == RunResult::Pass && !stats.coverage_db.contains_key(&runcoverage) {
            stats.population_set.insert(input.clone());
            stats.population_list.push(input.clone());

            stats.coverage_all.extend(runcoverage);
        }

        stats.cumulative_coverage.push(stats.coverage_all.len());
        assert!(
            stats.fuzz_cases == stats.cumulative_coverage.len(),
            "{} != {}:\n     {:?}",
            stats.fuzz_cases,
            stats.cumulative_coverage.len(),
            stats.cumulative_coverage
        );
    }
}

/// Get next random input to fuzz with by whichever means suitable
/// (e.g. generation of input, choosing as-is from initial corpus,
/// or mutating from current population of inputs).
pub fn fuzz(rng: &mut Rng, stats: &mut Statistics) -> Input {
    fuzz_(rng, stats, 2, 10 + 1)
}

pub fn fuzz_(
    rng: &mut Rng,
    stats: &mut Statistics,
    min_mutations: usize,
    max_mutations: usize,
) -> Input {
    stats.fuzz_cases += 1;
    if stats.fuzz_cases - 1 < stats.initial_population_size {
        // Choose input candidate from initial population as seed.
        stats.population_list[stats.fuzz_cases - 1].clone()
    } else {
        // Create new a input candidate through mutating existing population.

        // Choose random existing input from population.
        let mut candidate = rng.choice(&stats.population_list).clone();

        // Mutate that input a random number of times.
        let trials = rng.range(min_mutations as u64, max_mutations as u64);
        for _ in 0..trials {
            candidate = mutate(rng, candidate);
        }

        candidate
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

/// Location is a tuple (filename, linenumber).
type Location = (String, usize);

/// Statement coverage.
pub type Coverage = BTreeSet<Location>;

#[derive(Debug, Eq, PartialEq)]
pub enum RunResult {
    Pass,
    Fail,
    Unresolved,
}

/// Run the cgi_decode C program and trace coverage data.
pub fn run_and_get_coverage(input: &Input) -> (Coverage, RunResult) {
    // Compile the C program.
    process::Command::new("gcc")
        .args(["--coverage", "-o", "cgi_decode", "cgi_decode.c"])
        .output()
        .unwrap();

    // Run the program.
    let cres = process::Command::new("./cgi_decode")
        .arg(format!("{}", input))
        .stdout(process::Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Generate coverage data using gcov.
    process::Command::new("gcov")
        .arg("cgi_decode.c")
        .output()
        .unwrap();

    // "Parse" (process) gcov coverage file.
    let mut coverage = BTreeSet::new();
    for line in fs::read_to_string("cgi_decode.c.gcov").unwrap().lines() {
        let elems = line.split(':').collect::<Vec<_>>();
        let covered = elems[0].trim();
        let line_number = elems[1].trim().parse::<usize>().unwrap();
        if covered.starts_with("-") || covered.starts_with("#") {
            continue;
        }
        coverage.insert(("cgi_decode".to_string(), line_number));
    }

    let res = match cres.code() {
        Some(0) => RunResult::Pass,
        Some(n) if n < 0 => RunResult::Fail,
        _ => RunResult::Unresolved,
    };

    // Cleanup compiled and generated files.
    for file in [
        "cgi_decode.c.gcov",
        "cgi_decode",
        "cgi_decode.gcda",
        "cgi_decode.gcno",
    ] {
        let _ = fs::remove_file(file);
    }

    (coverage, res)
}

/// Output the cumulative coverage over time into a file that can then be
/// plotted in a diagram with gnuplot.
pub fn plot_cumulative_coverage(cov_cumul: &Vec<usize>) {
    let mut file = fs::File::create("plot.data").unwrap();
    for i in 0..cov_cumul.len() {
        file.write_all(format!("{} {}\n", i, cov_cumul[i]).as_bytes())
            .unwrap();
    }
}
