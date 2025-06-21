// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

mod grammarfuzzer;
mod rng;

use grammarfuzzer::{expr_grammar, fuzz_tree};
use rng::Rng;

fn main() {
    let grammar = expr_grammar();
    println!("[+] Expression grammar");
    println!("{}", grammar);
    println!();

    let mut rng = Rng::seeded(42);
    println!("[+] Running with random seed {}", rng.initialseed);
    println!();

    let tree = fuzz_tree(&mut rng, grammar);
    println!("{}", tree.to_dot());
    println!("{}", tree.all_leafs());
    // +((-7/0/6/4+4*5+4)*(9+6)-7*2.5-5+0/4-2)*(-4/3-6+5-5)/+6*-8*6*-4--++(7*9-8-8-6)/29/++7.33/9*7+-(4-8)/(8-3+5)*7*2+-(5)*2/0--8+9-8
}

// Debug trap: rust-lldb target
// unsafe { core::arch::asm!("int3"); }
