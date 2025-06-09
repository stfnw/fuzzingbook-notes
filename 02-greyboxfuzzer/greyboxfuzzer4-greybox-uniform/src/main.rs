// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

mod fuzzer;
mod rng;

use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;
use std::time::Instant;

use fuzzer::Input;

fn main() {
    let mut rng = rng::Rng::new();
    println!("[+] Running with random seed {}", rng.initialseed);

    let n = 3000;

    fuzzer::compile_program();

    let start = Instant::now();

    let mut stats = fuzzer::Statistics::default();

    for i in 0..n {
        if i % 200 == 0 {
            println!("Fuzz case {}", i);
        }

        let initial_population = vec![Input::from_str("good")];

        let input = fuzzer::fuzz(&mut rng, &stats, &initial_population);

        match fuzzer::run_and_get_coverage(&mut rng, &input) {
            fuzzer::RunResult::Crash => println!("Found crash!"),
            fuzzer::RunResult::Ok(coverage) => {
                let coveragehash = {
                    let mut hasher = DefaultHasher::new();
                    coverage.hash(&mut hasher);
                    hasher.finish()
                };

                if !stats.coverage_db.contains(&coveragehash) {
                    // We have some new coverage.
                    stats.coverage_db.insert(coveragehash);
                    stats.population.push(input);
                }
                stats.coverage_all.extend(coverage);
                stats.coverage_cumul.push(stats.coverage_all.len());
            }
        }

        stats.fuzz_cases += 1;
    }

    let end = Instant::now();

    println!();
    println!("[+] Greybox mutation-based fuzzer:");
    println!(
        "    - Runtime:                        {:0.4}s",
        (end - start).as_secs_f64()
    );
    println!(
        "    - Inputs leading to new coverage: {:?}",
        stats.population
    );
    println!(
        "    - All coverage:                   {:0.4} {:?}",
        stats.coverage_all.len(),
        stats.coverage_all
    );

    let mut logfile = std::fs::File::create("plot.data").unwrap();
    for (i, el) in stats.coverage_cumul.iter().enumerate() {
        writeln!(logfile, "{} {}", i, el).unwrap();
    }
    logfile.flush().unwrap();
}
