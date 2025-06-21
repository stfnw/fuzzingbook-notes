// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

mod grammarfuzzer;
mod rng;

use grammarfuzzer::{
    expand_node, expand_tree_once, expr_grammar_ebnf, fuzz, fuzz_tree, symbol_cost, tnt, ts, tt,
    SymbolCost,
};
use rng::Rng;

fn main() {
    let grammar = expr_grammar_ebnf().to_bnf();
    println!("[+] Expression grammar");
    println!("{:?}", grammar);
    println!();

    let derivation = tnt(
        "start",
        &[tnt("expr", &[tnt("expr", &[]), tt("+"), tnt("term", &[])])],
    );
    println!("[+] Graphviz / dot format:");
    println!("{}", derivation.to_dot());
    println!();

    let mut rng = Rng::new();
    println!("[+] Running with random seed {}", rng.initialseed);

    let derivation = expand_tree_once(&mut rng, &grammar, derivation);
    let derivation = expand_tree_once(&mut rng, &grammar, derivation);
    println!("{}", derivation.to_dot());

    /*
    let randomtree = fuzz_tree(&mut rng, &grammar);
    println!("{:?}", randomtree);
    println!("{}", randomtree.all_leafs());
    println!();

    println!("{}", fuzz(&mut rng, &grammar));
    println!();
    */
}
