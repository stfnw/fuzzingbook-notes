// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// https://www.fuzzingbook.org/html/Grammars.html

use std::collections::BTreeMap;

fn main() {
    let EXPR_GRAMMAR = {
        let mut tmp: Grammar = BTreeMap::new();
        tmp.insert("<start>", vec![vec!["<expr>"]]);
        tmp.insert(
            "<expr>",
            vec![
                vec!["<term>", "+", "<expr>"],
                vec!["<term>", "-", "<expr>"],
                vec!["<term>"],
            ],
        );
        tmp.insert(
            "<term>",
            vec![
                vec!["<factor>", "*", "<term>"],
                vec!["<factor>", "/", "<term>"],
                vec!["<factor>"],
            ],
        );
        tmp.insert(
            "<factor>",
            vec![
                vec!["+", "<factor>"],
                vec!["-", "<factor>"],
                vec!["(", "<factor>", ")"],
                vec!["<integer>", ".", "<integer>"],
                vec!["<integer>"],
            ],
        );
        tmp.insert(
            "<integer>",
            vec![vec!["<digit>", "<integer>"], vec!["<digit>"]],
        );
        tmp.insert(
            "<digit>",
            vec![
                vec!["0"],
                vec!["1"],
                vec!["2"],
                vec!["3"],
                vec!["4"],
                vec!["5"],
                vec!["6"],
                vec!["7"],
                vec!["8"],
                vec!["9"],
            ],
        );
        tmp
    };

    for (n, e) in EXPR_GRAMMAR.iter() {
        println!("{:?} -> {:?}", n, e);
    }

    // "<digit>" -> [["0"], ["1"], ["2"], ["3"], ["4"], ["5"], ["6"], ["7"], ["8"], ["9"]]
    // "<expr>" -> [["<term>", "+", "<expr>"], ["<term>", "-", "<expr>"], ["<term>"]]
    // "<factor>" -> [["+", "<factor>"], ["-", "<factor>"], ["(", "<factor>", ")"], ["<integer>", ".", "<integer>"], ["<integer>"]]
    // "<integer>" -> [["<digit>", "<integer>"], ["<digit>"]]
    // "<start>" -> [["<expr>"]]
    // "<term>" -> [["<factor>", "*", "<term>"], ["<factor>", "/", "<term>"], ["<factor>"]]
}

/// Represents a context free grammar as a set/map of production rules.
/// For easier processability the expansions of the production rules are grouped
/// by nonterminal.
type Grammar<'a> = BTreeMap<Nonterminal<'a>, Vec<Expansion<'a>>>;
type Nonterminal<'a> = &'a str;
type Expansion<'a> = Vec<&'a str>;
