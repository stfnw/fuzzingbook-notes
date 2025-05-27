// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/Coverage.html

mod rng;

use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::process;

/// Fuzz an external C program (cgi_decode) and gather/plot coverage.

fn main() {
    let mut rng = rng::Rng::new();

    let mut population = Vec::new();
    for _ in 0..100 {
        let len = rng.range(5, 10);
        let input = rng.ascii_printable(len);
        println!("{}", input);
        population.push(input);
    }

    let (_, cumulative_coverage) = population_coverage(population);

    let mut file = fs::File::create("plot.data").unwrap();
    for i in 0..cumulative_coverage.len() {
        file.write_all(format!("{} {}\n", i, cumulative_coverage[i]).as_bytes())
            .unwrap();
    }
}

type Input = String;
type Population = Vec<Input>;

fn population_coverage(population: Population) -> (StatementCoverage, Vec<usize>) {
    let mut all_coverage = BTreeSet::new();
    let mut cumulative_coverage = Vec::new();

    for s in population {
        let cov = run_and_get_coverage(s);
        all_coverage.extend(cov);
        cumulative_coverage.push(all_coverage.len());
    }

    (all_coverage, cumulative_coverage)
}

type Location = (String, usize);
type StatementCoverage = BTreeSet<Location>;

/// Run the cgi_decode C program and trace coverage data.
fn run_and_get_coverage(input: Input) -> StatementCoverage {
    // Compile the C program.
    process::Command::new("gcc")
        .args(["--coverage", "-o", "../cgi_decode", "../cgi_decode.c"])
        .output()
        .unwrap();

    // Run the program.
    process::Command::new("../cgi_decode")
        .arg(input)
        .output()
        .unwrap();

    // Generate coverage data using gcov.
    process::Command::new("gcov")
        .arg("../cgi_decode.c")
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

    // Cleanup compiled and generated files.
    for file in [
        "cgi_decode.c.gcov",
        "../cgi_decode",
        "../cgi_decode.gcda",
        "../cgi_decode.gcno",
    ] {
        let _ = fs::remove_file(file);
    }

    coverage
}
