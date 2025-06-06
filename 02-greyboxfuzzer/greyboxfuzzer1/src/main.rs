// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

mod fuzzer;
mod rng;

use std::collections::BTreeMap;

use fuzzer::{power_schedule_uniform_choose, Input};

fn main() {
    let mut rng = rng::Rng::new();

    let population = vec![
        Input::from_str("A"),
        Input::from_str("B"),
        Input::from_str("C"),
    ];

    let mut hits = BTreeMap::new();
    hits.insert("A".as_bytes(), 0);
    hits.insert("B".as_bytes(), 0);
    hits.insert("C".as_bytes(), 0);

    for _ in 0..10000 {
        let val = power_schedule_uniform_choose(&mut rng, &population);
        *hits.get_mut(&val.0[..]).unwrap() += 1;
    }

    println!("{:?}", hits);
    // {[65]: 3346, [66]: 3368, [67]: 3286}
}
