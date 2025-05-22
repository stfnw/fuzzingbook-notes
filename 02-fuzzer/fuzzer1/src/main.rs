// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/Fuzzer.html

mod rng;

fn main() {
    let random_fuzzer = RandomFuzzer::default();
    for _ in 0..10 {
        println!("{}", String::from_utf8(random_fuzzer.fuzz()).unwrap());
    }

    let random_fuzzer = RandomFuzzer::new(10, 20, 65, 26);
    for _ in 0..10 {
        println!("{}", String::from_utf8(random_fuzzer.fuzz()).unwrap());
    }
}

trait Fuzzer {
    fn fuzz(&self) -> Vec<u8>;
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
    fn fuzz(&self) -> Vec<u8> {
        let mut rng = rng::Rng::new();
        let len = rng.range(self.min_length, self.max_length);
        let mut res = Vec::new();
        for _ in 0..len {
            res.push(rng.range(self.char_start, self.char_start + self.char_range) as u8);
        }
        res
    }
}
