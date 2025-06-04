// SPDX-FileCopyrightText: 2019 Structure/architecture: adapted from gamozo https://github.com/gamozolabs/guifuzz
// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/MutationFuzzer.html Guiding by Coverage
// But refactored to multi-threaded runner adapted from https://github.com/gamozolabs/guifuzz.

mod fuzzer;
mod rng;

use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn main() {
    let stats = Arc::new(Mutex::new(fuzzer::Statistics::default()));

    let mut logfile = fs::File::create("plot.data").unwrap();

    let start_time = Instant::now();

    fuzzer::compile_program();

    let nthreads = 6;
    for _ in 0..nthreads {
        let stats = Arc::clone(&stats);
        let input = fuzzer::Input::from_str("http://www.google.com/search?q=fuzzing");

        std::thread::spawn(move || {
            fuzzer::run(stats, vec![input].as_slice());
        });
    }

    loop {
        std::thread::sleep(Duration::from_millis(1000));

        let uptime = (Instant::now() - start_time).as_secs_f64();

        let stats = stats.lock().unwrap();
        let curstats = (
            stats.fuzz_cases,
            stats.coverage_all.len(),
            stats.population_list.len(),
        );
        drop(stats);

        println!(
            "{:12.2} uptime | {:7} fuzz cases | {:8} coverage | {:5} inputs",
            uptime, curstats.0, curstats.1, curstats.2,
        );

        writeln!(
            logfile,
            "{:12.2} {:7} {:8} {:5}",
            uptime, curstats.0, curstats.1, curstats.2
        )
        .unwrap();
        logfile.flush().unwrap();
    }
}
