// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/Fuzzer.html

mod rng;

use std::io::{Read, Write};
use std::process;

fn main() {
    let random_fuzzer = RandomFuzzer::new(10, 20, 65, 26);
    let program_runner = ProgramRunner::new("cat".to_string());

    for _ in 0..10 {
        let res = random_fuzzer.run(&program_runner);
        println!("{:?} {}", res.1, res.0);
    }
}

#[derive(Debug)]
struct Bytes(Vec<u8>);

// This assumes that the string is valid utf8.
impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

#[derive(Debug)]
enum RunResult {
    Pass,
    Fail,
    Unresolved,
}

trait Runner {
    fn run(&self, inp: Bytes) -> (Bytes, RunResult);
}

struct PrintRunner {}

impl Runner for PrintRunner {
    fn run(&self, inp: Bytes) -> (Bytes, RunResult) {
        println!("{}", inp);
        (inp, RunResult::Unresolved)
    }
}

struct ProgramRunner {
    program: String,
}

impl ProgramRunner {
    fn new(program: String) -> Self {
        Self { program }
    }
}

impl Runner for ProgramRunner {
    fn run(&self, inp: Bytes) -> (Bytes, RunResult) {
        let mut pgm = process::Command::new(self.program.clone())
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = pgm.stdin.take().unwrap();
        let mut stdout = pgm.stdout.take().unwrap();
        let mut stderr = pgm.stderr.take().unwrap();

        stdin.write_all(&inp.0).unwrap();
        drop(stdin);

        let mut o: Vec<u8> = Vec::new();
        stdout.read_to_end(&mut o).unwrap();

        let mut e: Vec<u8> = Vec::new();
        stderr.read_to_end(&mut e).unwrap();

        let exitcode = pgm.wait().unwrap().code().unwrap();

        let outcome = if exitcode == 0 {
            RunResult::Pass
        } else if exitcode < 0 {
            RunResult::Fail
        } else {
            RunResult::Unresolved
        };

        (Bytes(o), outcome)
    }
}

trait Fuzzer {
    fn fuzz(&self) -> Bytes;
    fn run<T: Runner>(&self, runner: &T) -> (Bytes, RunResult);
}

struct RandomFuzzer {
    min_length: u64,
    max_length: u64,
    char_start: u64,
    char_range: u64,
}

impl RandomFuzzer {
    fn new(min_length: u64, max_length: u64, char_start: u64, char_range: u64) -> Self {
        assert!(char_start <= 0x100);
        assert!(char_start + char_range <= 0x100);
        Self {
            min_length,
            max_length,
            char_start,
            char_range,
        }
    }
}

impl Default for RandomFuzzer {
    fn default() -> Self {
        Self::new(10, 100, 32, 32)
    }
}

impl Fuzzer for RandomFuzzer {
    fn fuzz(&self) -> Bytes {
        let mut rng = rng::Rng::new();
        let len = rng.range(self.min_length, self.max_length);
        let mut res = Vec::new();
        for _ in 0..len {
            res.push(rng.range(self.char_start, self.char_start + self.char_range) as u8);
        }
        Bytes(res)
    }

    fn run<T: Runner>(&self, runner: &T) -> (Bytes, RunResult) {
        runner.run(self.fuzz())
    }
}
