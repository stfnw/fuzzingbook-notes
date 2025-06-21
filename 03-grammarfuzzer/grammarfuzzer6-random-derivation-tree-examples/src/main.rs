// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

mod examplegrammars;
mod grammarfuzzer;
mod rng;

use grammarfuzzer::{fuzz_tree, Grammar};

use rng::Rng;

fn main() {
    let mut rng = Rng::seeded(42);
    println!("[+] Running with random seed {}", rng.initialseed);
    println!();

    // Number of example derivation trees / expressions to generate from each
    // grammar.
    let n_examples = 10;

    for _ in 0..n_examples {
        run_grammar(
            &mut rng,
            examplegrammars::expr_grammar(),
            "expression-grammar",
        );
    }

    for _ in 0..n_examples {
        run_grammar(&mut rng, examplegrammars::cgi_grammar(), "cgi-grammar");
    }

    for _ in 0..n_examples {
        run_grammar(&mut rng, examplegrammars::title_grammar(), "title-grammar");
    }

    for _ in 0..n_examples {
        run_grammar(
            &mut rng,
            examplegrammars::json_grammar().to_bnf(),
            "json-grammar",
        );
    }
}

/// Create a random derivation tree from a grammar, write it out to dot/graphviz
/// format, and render it as PDF-file.
fn run_grammar(rng: &mut Rng, grammar: Grammar, grammarname: &str) {
    let filebase = format!("output/{}-{}", grammarname, rng.next());
    println!("[+] {}", filebase);

    let _ = std::fs::create_dir("output");

    let tree = fuzz_tree(rng, grammar);

    let terminals = tree.all_leafs();
    std::fs::write(format!("{}.txt", filebase), terminals).unwrap();

    let dot = tree.to_dot();
    std::fs::write(format!("{}.dot", filebase), dot).unwrap();
    std::process::Command::new("dot")
        .args([
            "-Tpdf",
            format!("{}.dot", filebase).as_str(),
            "-o",
            format!("{}.pdf", filebase).as_str(),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    println!();
}

// Debug trap: rust-lldb target
// unsafe { core::arch::asm!("int3"); }
