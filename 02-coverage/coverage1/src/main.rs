// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/Coverage.html

use std::collections::BTreeSet;
use std::fs;
use std::process;

/// Compile an external C program (cgi_decode) and gather statement coverage
/// from it through processing the gcov coverage data file.

// cov_standard = {("cgi_decode", 9), ("cgi_decode", 10), ("cgi_decode", 11), ("cgi_decode", 13), ("cgi_decode", 14), ("cgi_decode", 15), ("cgi_decode", 16), ("cgi_decode", 17), ("cgi_decode", 18), ("cgi_decode", 19), ("cgi_decode", 20), ("cgi_decode", 21), ("cgi_decode", 22), ("cgi_decode", 24), ("cgi_decode", 25), ("cgi_decode", 26), ("cgi_decode", 27), ("cgi_decode", 28), ("cgi_decode", 29), ("cgi_decode", 31), ("cgi_decode", 32), ("cgi_decode", 33), ("cgi_decode", 34), ("cgi_decode", 35), ("cgi_decode", 36), ("cgi_decode", 37), ("cgi_decode", 39), ("cgi_decode", 40), ("cgi_decode", 41), ("cgi_decode", 43), ("cgi_decode", 51), ("cgi_decode", 52), ("cgi_decode", 54), ("cgi_decode", 55), ("cgi_decode", 58), ("cgi_decode", 59), ("cgi_decode", 61), ("cgi_decode", 62), ("cgi_decode", 64), ("cgi_decode", 65), ("cgi_decode", 66), ("cgi_decode", 67)}
//
// cov_plus     = {("cgi_decode", 9), ("cgi_decode", 10), ("cgi_decode", 11), ("cgi_decode", 13), ("cgi_decode", 14), ("cgi_decode", 15), ("cgi_decode", 16), ("cgi_decode", 17), ("cgi_decode", 18), ("cgi_decode", 19), ("cgi_decode", 20), ("cgi_decode", 21), ("cgi_decode", 22), ("cgi_decode", 24), ("cgi_decode", 25), ("cgi_decode", 26), ("cgi_decode", 27), ("cgi_decode", 28), ("cgi_decode", 29), ("cgi_decode", 31), ("cgi_decode", 32), ("cgi_decode", 33), ("cgi_decode", 34), ("cgi_decode", 35), ("cgi_decode", 36), ("cgi_decode", 37), ("cgi_decode", 39), ("cgi_decode", 40), ("cgi_decode", 41), ("cgi_decode", 42), ("cgi_decode", 43), ("cgi_decode", 51), ("cgi_decode", 52), ("cgi_decode", 54), ("cgi_decode", 55), ("cgi_decode", 58), ("cgi_decode", 59), ("cgi_decode", 61), ("cgi_decode", 62), ("cgi_decode", 64), ("cgi_decode", 65), ("cgi_decode", 66), ("cgi_decode", 67)}
//
// difference   = [("cgi_decode", 42)]

fn main() {
    // let cov = run_and_get_coverage("Send+mail+to+me%40fuzzingbook.org");
    let cov_standard = run_and_get_coverage("abc");
    let cov_plus = run_and_get_coverage("a+b");

    println!("cov_standard = {:?}\n", cov_standard);

    println!("cov_plus     = {:?}\n", cov_plus);

    println!(
        "difference   = {:?}\n",
        cov_plus.difference(&cov_standard).collect::<Vec<_>>()
    );
}

type Location = (String, usize);

type StatementCoverage = BTreeSet<Location>;

/// Run the cgi_decode C program and trace coverage data.
fn run_and_get_coverage(input: &str) -> StatementCoverage {
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
