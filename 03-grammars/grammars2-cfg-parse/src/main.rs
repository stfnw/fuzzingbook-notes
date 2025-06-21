// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// https://www.fuzzingbook.org/html/Grammars.html

use std::collections::BTreeMap;

fn main() {
    let s = std::fs::read_to_string("expr_grammar.bnf").unwrap();
    let expr_grammar = parse(&s).unwrap();

    println!("{}", expr_grammar);
    // digit      -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
    // expr       -> term + expr | term - expr | term
    // factor     -> + factor | - factor | ( expr ) | integer . integer | integer
    // integer    -> digit integer | digit
    // start      -> expr
    // term       -> factor * expr | factor / expr | factor
}

/// Represents a context free grammar as a set/map of production rules.
/// For easier processability the expansions of the production rules are grouped
/// by nonterminal.
#[derive(Debug)]
struct Grammar(BTreeMap<Nonterminal, Vec<Expansion>>);
type Nonterminal = String;
type Expansion = Vec<String>;

impl std::fmt::Display for Grammar {
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

/// Buggy ad-hoc parser of BNF grammars.
/// grammar    := production +
/// production := expansion +
/// expansion  := symbol *
fn parse(s: &str) -> Result<Grammar, String> {
    let mut grammar = BTreeMap::new();

    for line in s.lines() {
        let (n, ve) = parse_production(line)?;
        grammar.insert(n, ve);
    }

    Ok(Grammar(grammar))
}

fn parse_production(s: &str) -> Result<(Nonterminal, Vec<Expansion>), String> {
    let sv: Vec<_> = s.split(":=").collect();

    if sv.len() != 2 {
        return Err(format!("Production must contain := {:?}", s));
    }

    let nonterminal = sv[0].trim().to_string();

    let es: Vec<_> = sv[1].split("|").map(|x| x.to_string()).collect();
    let mut expansions = Vec::new();

    for e in es {
        let e: Vec<_> = e
            .split_whitespace()
            .map(|x| x.trim().trim_matches('"').to_string())
            .collect();
        expansions.push(e);
    }

    Ok((nonterminal, expansions))
}
