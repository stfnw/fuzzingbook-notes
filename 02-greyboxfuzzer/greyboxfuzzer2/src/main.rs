// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

mod fuzzer;
mod rng;

use fuzzer::Input;

fn main() {
    let mut rng = rng::Rng::new();

    // let initial_input = Input::from_str("good");
    // let n = 30000;

    fuzzer::compile_program();

    for input in ["good", "bad", "bad!"] {
        let input = Input::from_str(input);
        let coverage = fuzzer::run_and_get_coverage(&mut rng, &input);
        println!("{:?}", coverage);
    }

    // Ok({("crashme", 8), ("crashme", 9), ("crashme", 13), ("crashme", 14), ("crashme", 16), ("crashme", 17)})
    // Ok({("crashme", 8), ("crashme", 9), ("crashme", 13), ("crashme", 14), ("crashme", 16), ("crashme", 19), ("crashme", 22), ("crashme", 25), ("crashme", 26)})
    // crashme: crashme.c:29: main: Assertion `0' failed.
    // Crash
}
