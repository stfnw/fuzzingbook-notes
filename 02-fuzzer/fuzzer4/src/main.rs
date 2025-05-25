// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/Fuzzer.html

mod rng;

use std::fs;
use std::process;

fn main() {
    let tmpdir = format!("/tmp/tmp-{}", unsafe { core::arch::x86_64::_rdtsc() });
    let tmpfile = format!("{}/{}", tmpdir, "input.txt");
    fs::create_dir(tmpdir).unwrap();

    let random_fuzzer = RandomFuzzer::new(20, 100, 32, 32);

    let mut runs = Vec::new();
    for _ in 0..100 {
        let data = random_fuzzer.fuzz();
        fs::write(&tmpfile, data.0).unwrap();

        let out = process::Command::new("bc")
            .arg(&tmpfile)
            .stdin(process::Stdio::null())
            .output()
            .unwrap();

        println!(
            "{} {} {}",
            out.status.code().unwrap(),
            Bytes(out.stdout.clone()),
            Bytes(out.stderr.clone())
        );
        runs.push(out);
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

trait Fuzzer {
    fn fuzz(&self) -> Bytes;
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
}
