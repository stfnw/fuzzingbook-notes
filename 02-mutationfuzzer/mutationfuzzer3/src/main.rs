// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/MutationFuzzer.html Guiding by Coverage

/// Wrappers around easily gathering code coverage.
mod coverage;
mod fuzzer;

use coverage::run_and_get_coverage;

fn main() {
    let input = fuzzer::Input::from_str("http://www.google.com/search?q=fuzzing");

    let coverage = run_and_get_coverage(input);
    println!("{:#?}", coverage);

    // re-formatted output:
    //
    // {
    //     ( "cgi_decode",  9),
    //     ( "cgi_decode", 10),
    //     ( "cgi_decode", 11),
    //     ( "cgi_decode", 13),
    //     ( "cgi_decode", 14),
    //     ( "cgi_decode", 15),
    //     ( "cgi_decode", 16),
    //     ( "cgi_decode", 17),
    //     ( "cgi_decode", 18),
    //     ( "cgi_decode", 19),
    //     ( "cgi_decode", 20),
    //     ( "cgi_decode", 21),
    //     ( "cgi_decode", 22),
    //     ( "cgi_decode", 24),
    //     ( "cgi_decode", 25),
    //     ( "cgi_decode", 26),
    //     ( "cgi_decode", 27),
    //     ( "cgi_decode", 28),
    //     ( "cgi_decode", 29),
    //     ( "cgi_decode", 31),
    //     ( "cgi_decode", 32),
    //     ( "cgi_decode", 33),
    //     ( "cgi_decode", 34),
    //     ( "cgi_decode", 35),
    //     ( "cgi_decode", 36),
    //     ( "cgi_decode", 37),
    //     ( "cgi_decode", 39),
    //     ( "cgi_decode", 40),
    //     ( "cgi_decode", 41),
    //     ( "cgi_decode", 43),
    //     ( "cgi_decode", 51),
    //     ( "cgi_decode", 52),
    //     ( "cgi_decode", 54),
    //     ( "cgi_decode", 55),
    //     ( "cgi_decode", 58),
    //     ( "cgi_decode", 59),
    //     ( "cgi_decode", 61),
    //     ( "cgi_decode", 62),
    //     ( "cgi_decode", 64),
    //     ( "cgi_decode", 65),
    //     ( "cgi_decode", 66),
    //     ( "cgi_decode", 67),
    // }
}
