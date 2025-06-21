// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// https://www.fuzzingbook.org/html/Grammars.html

use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let grammar = expr_grammar();
    println!("{}", grammar);
    // digit      -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8
    // expr       -> term + expr | term - expr | term
    // factor     -> + factor | - factor | ( expr ) | integer . integer | integer
    // integer    -> digit integer | digit
    // start      -> expr
    // term       -> factor * term | factor / term | factor

    println!("{:?}", grammar.nonterminals());
    // {"digit", "expr", "factor", "integer", "start", "term"}
}

fn expr_grammar<'a>() -> Grammar<'a> {
    let mut grammar = Grammar::new();
    grammar.add_production("start", vec!["expr"]);
    grammar.add_production("expr", vec!["term", "+", "expr"]);
    grammar.add_production("expr", vec!["term", "-", "expr"]);
    grammar.add_production("expr", vec!["term"]);
    grammar.add_production("term", vec!["factor", "*", "term"]);
    grammar.add_production("term", vec!["factor", "/", "term"]);
    grammar.add_production("term", vec!["factor"]);
    grammar.add_production("factor", vec!["+", "factor"]);
    grammar.add_production("factor", vec!["-", "factor"]);
    grammar.add_production("factor", vec!["(", "expr", ")"]);
    grammar.add_production("factor", vec!["integer", ".", "integer"]);
    grammar.add_production("factor", vec!["integer"]);
    grammar.add_production("integer", vec!["digit", "integer"]);
    grammar.add_production("integer", vec!["digit"]);
    grammar.add_production("digit", vec!["0"]);
    grammar.add_production("digit", vec!["1"]);
    grammar.add_production("digit", vec!["2"]);
    grammar.add_production("digit", vec!["3"]);
    grammar.add_production("digit", vec!["4"]);
    grammar.add_production("digit", vec!["5"]);
    grammar.add_production("digit", vec!["6"]);
    grammar.add_production("digit", vec!["7"]);
    grammar.add_production("digit", vec!["8"]);
    grammar
}

/// Represents a context free grammar as a set/map of production rules.
/// For easier processability the expansions of the production rules are grouped
/// by nonterminal.
struct Grammar<'a>(BTreeMap<Nonterminal<'a>, Vec<Expansion<'a>>>);
type Nonterminal<'a> = &'a str;
type Expansion<'a> = Vec<&'a str>;

impl std::fmt::Display for Grammar<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (nonterminal, expansions) in self.0.iter() {
            writeln!(
                f,
                "{:10} -> {}",
                nonterminal,
                expansions
                    .iter()
                    .map(|exp| exp.join(" "))
                    .collect::<Vec<_>>()
                    .join(" | ")
            )?;
        }
        Ok(())
    }
}

impl<'a> Grammar<'a> {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn add_production(&mut self, nonterminal: Nonterminal<'a>, expansion: Expansion<'a>) {
        let tmp = self.0.get_mut(nonterminal);
        match tmp {
            Some(expansions) => expansions.push(expansion),
            None => {
                self.0.insert(nonterminal, vec![expansion]);
            }
        };
    }

    fn nonterminals(&self) -> BTreeSet<Nonterminal> {
        self.0.iter().map(|(k, _)| *k).collect()
    }
}
