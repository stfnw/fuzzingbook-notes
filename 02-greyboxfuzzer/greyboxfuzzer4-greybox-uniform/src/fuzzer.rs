// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;
use std::fs;
use std::process;

use crate::rng::Rng;

/// Represents the structure that the fuzzer operates on. Here we use a
/// dedicated newtype instead of a type alias for being able to implement
/// integrated printing routines.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Input(pub Vec<u8>);

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
        write!(f, "{}", String::from_utf8(self.0.clone()).unwrap())
    }
}

impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Location is a linenumber
type Location = usize;

/// Statement coverage.
pub type Coverage = BTreeSet<Location>;

/// Statistics relevant during fuzzing.
#[derive(Default)]
pub struct Statistics {
    /// Number of times a random input was tested.
    pub fuzz_cases: usize,

    /// List of all inputs with unique coverage.
    pub population: Vec<Input>,

    /// Union of all coverages that were achieved during execution.
    pub coverage_all: Coverage,

    /// Hashes of all coverages seen during fuzzing.
    pub coverage_db: BTreeSet<u64>,

    /// Trace/history of coverage_all sizes.
    pub coverage_cumul: Vec<usize>,
}

/// Get next random input to fuzz with by whichever means suitable
/// (e.g. generation of input, choosing as-is from initial corpus,
/// or mutating from current population of inputs).
pub fn fuzz(rng: &mut Rng, stats: &Statistics, seed: &[Input]) -> Input {
    if stats.fuzz_cases < seed.len() {
        // Choose input candidate from initial population as seed.
        seed[stats.fuzz_cases].clone()
    } else {
        // Create new a input candidate through mutating existing population.

        // Choose random existing input from population.
        let mut candidate = power_schedule_choose(rng, &stats.population).clone();

        // Then mutate that input a random number of times.

        let trials = std::cmp::min(candidate.0.len() as u64, 1 << rng.range(1, 5));
        for _ in 0..trials {
            candidate = mutate(rng, candidate);
        }

        candidate
    }
}

/// Compile the crashme C program. This is done in a separate function and
/// not in run_and_get_coverage, since it only has to be done once and not on
/// each fuzz case (the source code doesn't change between fuzz cases).
pub fn compile_program() {
    process::Command::new("gcc")
        .args(["-Wall", "-g", "--coverage", "-o", "crashme", "crashme.c"])
        .output()
        .unwrap();
}

#[derive(Debug, Eq, PartialEq)]
/// Results of the execution of an external program (essentially an Option).
pub enum RunResult {
    /// Program exits in any other way not due to crash/signal.
    Ok(Coverage),
    /// Program crashes.
    Crash,
}

/// Run the crashme C program and trace coverage data.
pub fn run_and_get_coverage(rng: &mut Rng, input: &Input) -> RunResult {
    // Create new temporary directory for multi-threaded running without
    // conflicts.
    let root = format!(
        "testrun-{}-{}",
        unsafe { core::arch::x86_64::_rdtsc() },
        rng.next(),
    );

    // Create a temporary directory that acts as root for this fuzz case execution.
    fs::create_dir(&root).unwrap();
    // Copy over the relevant data resulting from the initial program compilation.
    // This is needed for later gathering code coverage with `gcov`.
    fs::copy("crashme.c", format!("{}/{}", root, "crashme.c")).unwrap();
    fs::copy("crashme", format!("{}/{}", root, "crashme")).unwrap();
    fs::copy("crashme.gcno", format!("{}/{}", root, "crashme.gcno")).unwrap();

    // Run the program.
    let exitstatus = process::Command::new(
        // https://doc.rust-lang.org/std/process/struct.Command.html#method.current_dir
        // > If the program path is relative (e.g., "./script.sh"), it’s ambiguous
        // > whether it should be interpreted relative to the parent’s working
        // > directory or relative to current_dir. The behavior in this case is
        // > platform specific and unstable, and it’s recommended to use
        // > canonicalize to get an absolute program path instead.
        fs::canonicalize("./crashme").unwrap(),
    )
    .current_dir(&root)
    .arg(format!("{}", input))
    // https://gcc.gnu.org/onlinedocs/gcc/Cross-profiling.html
    // The following two environment variables are needed in order to
    // instruct gcov to write the collected information into the current
    // directory (inside the temporary root of this fuzz case) and not into
    // the directory/absolute path were the program was initially compiled
    // (which is global to all fuzz cases and would thus lead to conflicts).
    .env("GCOV_PREFIX", ".")
    // Strip leading directory names from the initial absolute path. This
    // value should be enough, although I'd prefer an explicit option to
    // strip all leading directory names (idk if there is such an option).
    .env("GCOV_PREFIX_STRIP", "20")
    .stdout(process::Stdio::null())
    .spawn()
    .unwrap()
    .wait()
    .unwrap();

    // Generate coverage data using gcov.
    process::Command::new("gcov")
        .current_dir(&root)
        .arg("crashme.c")
        .output()
        .unwrap();

    // "Parse" (process) gcov coverage file.
    let mut coverage = BTreeSet::new();
    for line in fs::read_to_string(format!("{}/{}", root, "crashme.c.gcov"))
        .unwrap()
        .lines()
    {
        let elems = line.split(':').collect::<Vec<_>>();
        let covered = elems[0].trim();
        let line_number = elems[1].trim().parse::<usize>().unwrap();
        if covered.starts_with("-") || covered.starts_with("#") {
            continue;
        }
        coverage.insert(line_number);
    }

    // Cleanup compiled and generated files.
    for file in [
        format!("{}/{}", root, "crashme"),
        format!("{}/{}", root, "crashme.c"),
        format!("{}/{}", root, "crashme.c.gcov"),
        format!("{}/{}", root, "crashme.gcda"),
        format!("{}/{}", root, "crashme.gcno"),
    ] {
        let _ = fs::remove_file(file);
    }
    fs::remove_dir(root).unwrap();

    if exitstatus.success() {
        RunResult::Ok(coverage)
    } else {
        assert!(coverage.is_empty());
        RunResult::Crash
    }
}

/// Choose a value from a given population of inputs for the SUT.
/// This implementation chooses according to a uniform distribution.
pub fn power_schedule_choose<'a>(rng: &mut Rng, pop: &'a [Input]) -> &'a Input {
    rng.choice(pop)
}

/// Choose a random mutation strategy and apply it to the input.
pub fn mutate(rng: &mut Rng, s: Input) -> Input {
    match rng.int(3) {
        0 => insert_random_character(rng, s),
        1 => delete_random_character(rng, s),
        2 => flip_random_bit(rng, s),
        _ => panic!("Can't happen"),
    }
}

fn insert_random_character(rng: &mut Rng, mut s: Input) -> Input {
    let pos = rng.int((s.0.len() + 1) as u64) as usize;
    let chr = rng.range(32, 127 + 1) as u8;
    s.0.insert(pos, chr);
    s
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

fn flip_random_bit(rng: &mut Rng, mut s: Input) -> Input {
    // Here we loop to make sure that the result does not contain a null-byte,
    // which would be impossible to pass on the commandline to the external
    // program.
    loop {
        let pos = rng.int(s.0.len() as u64) as usize;
        let bit = 1 << rng.int(7);
        s.0[pos] ^= bit; // Flip bit.
        if s.0[pos] != 0 {
            return s;
        }
        s.0[pos] ^= bit; // Flip bit back and try next random mutation.
    }
}
