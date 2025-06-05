// SPDX-FileCopyrightText: 2019 Structure/architecture: adapted from gamozo https://github.com/gamozolabs/guifuzz
// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use crate::rng::Rng;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::process;
use std::sync::{Arc, Mutex};

/// Represents the structure that the fuzzer operates on. Here we use a
/// dedicated newtype instead of a type alias for being able to implement
/// integrated printing routines.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
        write!(f, "{}", String::from_utf8(self.0.clone()).unwrap())
    }
}

/// Statistics relevant during fuzzing.
#[derive(Default)]
pub struct Statistics {
    /// Number of times a random input was tested.
    pub fuzz_cases: usize,

    /// Set of all inputs with unique coverage.
    pub population_set: BTreeSet<Input>,

    /// List of all inputs with unique coverage.
    pub population_list: Vec<Input>,

    /// Coverage database. Associates each unique coverage set with the
    /// corresponding input that caused it. Since we only care about whether or
    /// not we have seen a specific coverage before and not what specific
    /// coverage was obtained, we only record a hash of the coverage information
    /// (to reduce memory usage).
    pub coverage_db: BTreeMap<u64, Input>,

    /// Union of all coverages that were achieved during execution.
    pub coverage_all: Coverage,
}

/// Create and run `n` random fuzz cases and record statistics during execution.
pub fn run(stats: Arc<Mutex<Statistics>>, seed: &[Input]) {
    let mut rng = Rng::new();
    println!("[+] Running with random seed {}", rng.initialseed);

    loop {
        let input = fuzz(&mut rng, Arc::clone(&stats), seed);

        // Ignore input (and don't perform unnecessary expensive re-evaluation)
        // in case we have seen that exact input before.
        if stats.lock().unwrap().population_set.contains(&input) {
            continue;
        }

        let (runcoverage, runoutcome) = run_and_get_coverage(&mut rng, &input);

        let runcoveragehash = {
            let mut hasher = DefaultHasher::new();
            runcoverage.hash(&mut hasher);
            hasher.finish()
        };

        let mut stats = stats.lock().unwrap();

        // Update all fields of the Statistics structure in order.

        // Fuzz cases are always increased, regardless of the coverage information.
        stats.fuzz_cases += 1;

        // Check if the obtained coverage contains new entries / is interesting.
        if runoutcome == RunResult::Pass && !stats.coverage_db.contains_key(&runcoveragehash) {
            stats.population_set.insert(input.clone());
            stats.population_list.push(input.clone());

            stats.coverage_db.insert(runcoveragehash, input.clone());
            stats.coverage_all.extend(runcoverage);

            drop(stats);

            record_input(input);
        }
    }
}

/// Write out/persist interesting inputs that lead to new coverage to the
/// file system. The filename is a hash of the input (content-addressed).
// https://github.com/gamozolabs/guifuzz/blob/471d744e0e46d21cad39e4287ddc6f13c9811b17/mesos/src/main.rs#L18
// https://doc.rust-lang.org/std/hash/trait.Hash.html
fn record_input(input: Input) {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);

    let _ = fs::create_dir("interesting-inputs");
    fs::write(
        format!("interesting-inputs/{:016x}.input", hasher.finish()),
        input.0,
    )
    .unwrap();
}

/// Get next random input to fuzz with by whichever means suitable
/// (e.g. generation of input, choosing as-is from initial corpus,
/// or mutating from current population of inputs).
pub fn fuzz(rng: &mut Rng, stats: Arc<Mutex<Statistics>>, seed: &[Input]) -> Input {
    fuzz_(rng, stats, seed, 2, 10 + 1)
}

pub fn fuzz_(
    rng: &mut Rng,
    stats: Arc<Mutex<Statistics>>,
    seed: &[Input],
    min_mutations: usize,
    max_mutations: usize,
) -> Input {
    let fuzz_cases = stats.lock().unwrap().fuzz_cases;

    if fuzz_cases < seed.len() {
        // Choose input candidate from initial population as seed.
        seed[fuzz_cases].clone()
    } else {
        // Create new a input candidate through mutating existing population.

        // Choose random existing input from population.
        let mut candidate = rng.choice(&stats.lock().unwrap().population_list).clone();

        // Then mutate that input a random number of times.

        // Because mutation can introduce a null-byte -- which is invalid when
        // trying to execute the program (the input is passed on the commandline
        // and null-bytes cannot be passed on the commandline) -- we wrap the
        // generation logic into a loop and retry until we get a valid candiate.
        loop {
            let trials = rng.range(min_mutations as u64, max_mutations as u64);
            for _ in 0..trials {
                candidate = mutate(rng, candidate);
            }

            if !candidate.0.contains(&0x00) {
                break;
            }
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

/// Compile the cgi_decode C program. This is done in a separate function and
/// not in run_and_get_coverage, since it only has to be done once and not on
/// each fuzz case (the source code doesn't change between fuzz cases).
pub fn compile_program() {
    // Compile the C program.
    process::Command::new("gcc")
        .args(["--coverage", "-o", "cgi_decode", "cgi_decode.c"])
        .output()
        .unwrap();
}

/// Run the cgi_decode C program and trace coverage data.
pub fn run_and_get_coverage(rng: &mut Rng, input: &Input) -> (Coverage, RunResult) {
    // Create new temporary directory for multi-threaded running without
    // conflicts.
    let root = format!(
        "testrun-{}-{}",
        unsafe { core::arch::x86_64::_rdtsc() },
        rng.next(),
    );

    // Create a temporary directory that acts as root for this fuzz case execution.
    std::fs::create_dir(&root).unwrap();
    // Copy over the relevant data resulting from the initial program compilation.
    // This is needed for later gathering code coverage with `gcov`.
    std::fs::copy("cgi_decode.c", format!("{}/{}", root, "cgi_decode.c")).unwrap();
    std::fs::copy("cgi_decode", format!("{}/{}", root, "cgi_decode")).unwrap();
    std::fs::copy("cgi_decode.gcno", format!("{}/{}", root, "cgi_decode.gcno")).unwrap();

    // Run the program.
    let cres = process::Command::new(
        // https://doc.rust-lang.org/std/process/struct.Command.html#method.current_dir
        // > If the program path is relative (e.g., "./script.sh"), it’s ambiguous
        // > whether it should be interpreted relative to the parent’s working
        // > directory or relative to current_dir. The behavior in this case is
        // > platform specific and unstable, and it’s recommended to use
        // > canonicalize to get an absolute program path instead.
        fs::canonicalize("./cgi_decode").unwrap(),
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
        .arg("cgi_decode.c")
        .output()
        .unwrap();

    // "Parse" (process) gcov coverage file.
    let mut coverage = BTreeSet::new();
    for line in fs::read_to_string(format!("{}/{}", root, "cgi_decode.c.gcov"))
        .unwrap()
        .lines()
    {
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
        format!("{}/{}", root, "cgi_decode"),
        format!("{}/{}", root, "cgi_decode.c"),
        format!("{}/{}", root, "cgi_decode.c.gcov"),
        format!("{}/{}", root, "cgi_decode.gcda"),
        format!("{}/{}", root, "cgi_decode.gcno"),
    ] {
        let _ = fs::remove_file(file);
    }
    fs::remove_dir(root).unwrap();

    (coverage, res)
}
