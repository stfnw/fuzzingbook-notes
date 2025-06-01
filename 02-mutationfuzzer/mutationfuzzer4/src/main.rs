// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/MutationFuzzer.html Guiding by Coverage

/// Wrappers around easily gathering code coverage.
mod coverage;
mod fuzzer;
mod rng;

use coverage::plot_cumulative_coverage;
use fuzzer::MutationCoverageFuzzer;
use rng::Rng;

// [+] Running with random seed 15755402468159623144
//
// [+] Final population
// http://www.google.com/search?q=fuzzing
// http://www.google.com+search?q=fuzijg
//
// [+] Final coverage: 43

fn main() {
    let mut rng = Rng::new();
    println!("[+] Running with random seed {}", rng.initialseed);
    println!();

    let input = fuzzer::Input::from_str("http://www.google.com/search?q=fuzzing");

    let mut mutation_fuzzer = MutationCoverageFuzzer::new(vec![input]);
    let (cov_all, cov_cumul) = mutation_fuzzer.runs(&mut rng, 30);

    let pop = mutation_fuzzer.population();

    println!("[+] Final population");
    for el in pop {
        println!("{}", el);
    }
    println!();

    println!("[+] Final coverage: {}", cov_all.len());

    // Output gnuplot file: ./plot.plt
    // Note: since running external programs is so slow, we can't execute many
    // fuzz cases => the code coverage doesn't improve significantly
    // => the diagram is relatively useless.
    // => write multithreaded implementation.
    plot_cumulative_coverage(&cov_cumul);
}
