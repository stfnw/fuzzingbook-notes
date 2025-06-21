// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// https://www.fuzzingbook.org/html/Grammars.html

use std::collections::{BTreeMap, BTreeSet};

fn main() {
    println!("{}", expr_grammar());
    // <start> -> "expr"
    // digit   -> "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
    // expr    -> "term" "+" "expr" | "term" "-" "expr" | "term"
    // factor  -> "+" "factor" | "-" "factor" | "(" "expr" ")" | "integer" "." "integer" | "integer"
    // integer -> "digit" "integer" | "digit"
    // term    -> "factor" "*" "term" | "factor" "/" "term" | "factor"

    println!("{}", cgi_grammar());
    // <start>  -> "string"
    // hexdigit -> "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
    // letter   -> "plus" | "percent" | "other"
    // other    -> "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "-" | "_"
    // percent  -> "%" "hexdigit" "hexdigit"
    // plus     -> "+"
    // string   -> "letter" | "letter" "string"

    println!("{}", title_grammar());
    // <fuzzing-prefix>    -> "" | "The Art of " | "The Joy of "
    // <reader-property>   -> "Fun" | "Profit"
    // <software-property> -> "Robustness" | "Reliability" | "Security"
    // <start>             -> <title>
    // <subtopic-main>     -> "Breaking Software" | "Generating Software Tests" | "Principles, Techniques and Tools"
    // <subtopic-prefix>   -> "" | "Tools and Techniques for "
    // <subtopic-suffix>   -> " for " <reader-property> " and " <reader-property> | " for " <software-property> " and " <software-property>
    // <subtopic>          -> <subtopic-main> | <subtopic-prefix> <subtopic-main> | <subtopic-main> <subtopic-suffix>
    // <title>             -> <topic> ": " <subtopic>
    // <topic>             -> "Generating Software Tests" | <fuzzing-prefix> "Fuzzing" | "The Fuzzing Book"
}

fn expr_grammar() -> Grammar {
    let mut grammar = Grammar::new();

    grammar.add_production("<start>", &["expr"]);

    grammar.add_production("expr", &["term", "+", "expr"]);
    grammar.add_production("expr", &["term", "-", "expr"]);
    grammar.add_production("expr", &["term"]);

    grammar.add_production("term", &["factor", "*", "term"]);
    grammar.add_production("term", &["factor", "/", "term"]);
    grammar.add_production("term", &["factor"]);

    grammar.add_production("factor", &["+", "factor"]);
    grammar.add_production("factor", &["-", "factor"]);
    grammar.add_production("factor", &["(", "expr", ")"]);
    grammar.add_production("factor", &["integer", ".", "integer"]);
    grammar.add_production("factor", &["integer"]);

    grammar.add_production("integer", &["digit", "integer"]);
    grammar.add_production("integer", &["digit"]);

    let digits: Vec<_> = (0..10).map(|x| format!("{}", x)).collect();
    grammar.add_productions(
        "digit",
        &digits.iter().map(|x| x.as_str()).collect::<Vec<_>>(),
    );

    grammar
}

fn cgi_grammar() -> Grammar {
    let mut grammar = Grammar::new();

    grammar.add_production("<start>", &["string"]);

    grammar.add_production("string", &["letter"]);
    grammar.add_production("string", &["letter", "string"]);

    grammar.add_production("letter", &["plus"]);
    grammar.add_production("letter", &["percent"]);
    grammar.add_production("letter", &["other"]);

    grammar.add_production("plus", &["+"]);

    grammar.add_production("percent", &["%", "hexdigit", "hexdigit"]);

    for i in 0..10 {
        grammar.add_production("hexdigit", &[format!("{}", i).as_str()]);
    }

    let other: Vec<_> = ((0..26).map(|x| char::from(x + ('a' as u8)).to_string()))
        .chain((0..10).map(|x| x.to_string()))
        .chain(["-", "_"].into_iter().map(|x| x.to_string()))
        .collect();
    grammar.add_productions(
        "other",
        &other.iter().map(|x| x.as_str()).collect::<Vec<_>>(),
    );
    // grammar.add_production("other", &["0"]);
    // grammar.add_production("other", &["1"]);
    // grammar.add_production("other", &["2"]);
    // grammar.add_production("other", &["3"]);
    // grammar.add_production("other", &["4"]);
    // grammar.add_production("other", &["5"]);
    // grammar.add_production("other", &["a"]);
    // grammar.add_production("other", &["b"]);
    // grammar.add_production("other", &["c"]);
    // grammar.add_production("other", &["d"]);
    // grammar.add_production("other", &["e"]);
    // grammar.add_production("other", &["-"]);
    // grammar.add_production("other", &["_"]);

    grammar
}

fn title_grammar() -> Grammar {
    let mut grammar = Grammar::new();

    grammar.add_production("<start>", &["<title>"]);

    grammar.add_production("<title>", &["<topic>", ": ", "<subtopic>"]);

    grammar.add_production("<topic>", &["Generating Software Tests"]);
    grammar.add_production("<topic>", &["<fuzzing-prefix>", "Fuzzing"]);
    grammar.add_production("<topic>", &["The Fuzzing Book"]);

    grammar.add_production("<fuzzing-prefix>", &[""]);
    grammar.add_production("<fuzzing-prefix>", &["The Art of "]);
    grammar.add_production("<fuzzing-prefix>", &["The Joy of "]);

    grammar.add_production("<subtopic>", &["<subtopic-main>"]);
    grammar.add_production("<subtopic>", &["<subtopic-prefix>", "<subtopic-main>"]);
    grammar.add_production("<subtopic>", &["<subtopic-main>", "<subtopic-suffix>"]);

    grammar.add_production("<subtopic-main>", &["Breaking Software"]);
    grammar.add_production("<subtopic-main>", &["Generating Software Tests"]);
    grammar.add_production("<subtopic-main>", &["Principles, Techniques and Tools"]);

    grammar.add_production("<subtopic-prefix>", &[""]);
    grammar.add_production("<subtopic-prefix>", &["Tools and Techniques for "]);

    #[rustfmt::skip]
    grammar.add_production("<subtopic-suffix>",
        &[" for ", "<reader-property>", " and ", "<reader-property>"]);
    #[rustfmt::skip]
    grammar.add_production("<subtopic-suffix>",
        &[" for ", "<software-property>", " and ", "<software-property>"]);

    grammar.add_production("<reader-property>", &["Fun"]);
    grammar.add_production("<reader-property>", &["Profit"]);

    grammar.add_production("<software-property>", &["Robustness"]);
    grammar.add_production("<software-property>", &["Reliability"]);
    grammar.add_production("<software-property>", &["Security"]);

    grammar
}

/// Represents a context free grammar as a set/map of production rules.
/// For easier processability the expansions of the production rules are grouped
/// by nonterminal.
struct Grammar(BTreeMap<Nonterminal, Vec<Expansion>>);
type Nonterminal = String;
type Expansion = Vec<String>;

impl std::fmt::Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let maxnonterminallength = self.0.keys().map(|x| x.len()).max().unwrap_or(10);
        for (nonterminal, expansions) in self.0.iter() {
            writeln!(
                f,
                "{:maxnonterminallength$} -> {}",
                nonterminal,
                expansions
                    .iter()
                    .map(|expansion| expansion
                        .iter()
                        .map(|symbol| if is_nonterminal(symbol) {
                            symbol.to_string()
                        } else {
                            format!("\"{}\"", symbol)
                        })
                        .collect::<Vec<_>>()
                        .join(" "))
                    .collect::<Vec<_>>()
                    .join(" | ")
            )?;
        }
        Ok(())
    }
}

impl Grammar {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Add a single production of the form: nonterminal -> [symbols]
    /// to the grammar.
    fn add_production(&mut self, nonterminal: &str, expansion: &[&str]) {
        let tmp = self.0.get_mut(nonterminal);

        let nonterminal = nonterminal.to_string();
        let expansion = expansion.iter().map(|x| x.to_string()).collect();

        match tmp {
            Some(expansions) => expansions.push(expansion),
            None => {
                self.0.insert(nonterminal, vec![expansion]);
            }
        }
    }

    /// Helper function for adding lots of productions which each have only one
    /// alternative to the grammar.
    fn add_productions(&mut self, nonterminal: &str, expansions: &[&str]) {
        let expansions: Vec<_> = expansions
            .into_iter()
            .map(|x| vec![x.to_string()])
            .collect();
        match self.0.get_mut(nonterminal) {
            Some(exps) => exps.extend(expansions),
            None => {
                self.0.insert(nonterminal.to_string(), expansions);
            }
        }
    }

    fn nonterminals(&self) -> BTreeSet<Nonterminal> {
        self.0.keys().cloned().collect()
    }
}

/// Determines if a given symbol name represents a nonterminal.
/// This is only by convention and not actually enforced anywhere.
fn is_nonterminal(s: &str) -> bool {
    s.starts_with("<") && s.ends_with(">")
}
