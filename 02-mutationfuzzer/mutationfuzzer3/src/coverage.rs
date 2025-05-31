// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use crate::fuzzer::Input;

use std::collections::BTreeSet;
use std::fs;
use std::process;

/// Location is a tuple (filename, linenumber).
type Location = (String, usize);

/// Statement coverage.
type Coverage = BTreeSet<Location>;

/// Run the cgi_decode C program and trace coverage data.
pub fn run_and_get_coverage(input: Input) -> Coverage {
    // Compile the C program.
    process::Command::new("gcc")
        .args(["--coverage", "-o", "cgi_decode", "cgi_decode.c"])
        .output()
        .unwrap();

    // Run the program.
    process::Command::new("./cgi_decode")
        .arg(format!("{}", input))
        .output()
        .unwrap();

    // Generate coverage data using gcov.
    process::Command::new("gcov")
        .arg("cgi_decode.c")
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
        "cgi_decode",
        "cgi_decode.gcda",
        "cgi_decode.gcno",
    ] {
        let _ = fs::remove_file(file);
    }

    coverage
}
