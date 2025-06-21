// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use std::collections::VecDeque;
use std::collections::{BTreeMap, BTreeSet};

use crate::rng::Rng;

pub fn expr_grammar() -> Grammar {
    let mut grammar = Grammar::new();

    grammar.add_production("<start>", &["<expr>"]);

    grammar.add_production("<expr>", &["<term>", "+", "<expr>"]);
    grammar.add_production("<expr>", &["<term>", "-", "<expr>"]);
    grammar.add_production("<expr>", &["<term>"]);

    grammar.add_production("<term>", &["<factor>", "*", "<term>"]);
    grammar.add_production("<term>", &["<factor>", "/", "<term>"]);
    grammar.add_production("<term>", &["<factor>"]);

    grammar.add_production("<factor>", &["+", "<factor>"]);
    grammar.add_production("<factor>", &["-", "<factor>"]);
    grammar.add_production("<factor>", &["(", "<expr>", ")"]);
    grammar.add_production("<factor>", &["<integer>", ".", "<integer>"]);
    grammar.add_production("<factor>", &["<integer>"]);

    grammar.add_production("<integer>", &["<digit>", "<integer>"]);
    grammar.add_production("<integer>", &["<digit>"]);

    let digits: Vec<_> = (0..10).map(|x| format!("{}", x)).collect();
    grammar.add_productions(
        "<digit>",
        &digits.iter().map(|x| x.as_str()).collect::<Vec<_>>(),
    );

    grammar
}

/// Represents a context-free-grammar as a set/map of production rules.
/// For easier processability the expansions of the production rules are grouped
/// by nonterminal. This results in a mapping Nonterminal -> Vec<Vec<String>>.
/// The outer Vec are the different alternatives/choices of the rule.
/// The inner Vec is the sequence / string that the nonterminal expands to.
/// Each inner Vec corresponds to one production rule Nonterminal -> Vec<String>
/// in the formal grammar.
/// By convention nonterminal symbols are enclosed in angle brackets (`<nonterminal>`)
/// and terminal symbols are plain strings (`"terminal"`).
#[derive(PartialEq, Eq, Debug)]
pub struct Grammar(BTreeMap<Nonterminal, Vec<Expansion>>);
pub type Nonterminal = String;
pub type Expansion = Vec<String>; // Right-hand-side of a production rule.

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
                        .map(|symbol| if Grammar::is_nonterminal(symbol) {
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
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Add a single production of the form: nonterminal -> [symbols]
    /// to the grammar.
    pub fn add_production(&mut self, nonterminal: &str, expansion: &[&str]) {
        self.add_production_(
            nonterminal.to_string(),
            expansion.iter().map(|x| x.to_string()).collect(),
        );
    }

    /// Add a single production of the form: nonterminal -> [symbols]
    /// to the grammar (for owned values).
    pub fn add_production_(&mut self, nonterminal: Nonterminal, expansion: Expansion) {
        let tmp = self.0.get_mut(&nonterminal);

        match tmp {
            Some(expansions) => expansions.push(expansion),
            None => {
                self.0.insert(nonterminal, vec![expansion]);
            }
        }
    }

    /// Helper function for adding lots of productions which each have only one
    /// alternative to the grammar.
    pub fn add_productions(&mut self, nonterminal: &str, expansions: &[&str]) {
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

    /// Create a copy of the grammar with only the actually used/reachable
    /// production rules. Fails if the grammar is invalid.
    pub fn trim(&self) -> Result<Grammar, String> {
        let mut res = Grammar::new();

        // Set of already processed/seen nonterminals (this prevents infinite
        // loops in recursive productions).
        let mut seen_nonterminals = BTreeSet::new();

        let mut stack = Vec::new();
        stack.push("<start>".to_string());

        while let Some(nonterminal) = stack.pop() {
            if seen_nonterminals.contains(&nonterminal) {
                continue;
            }
            seen_nonterminals.insert(nonterminal.clone());

            match self.0.get(&nonterminal) {
                Some(expansions) => {
                    for expansion in expansions.iter() {
                        res.add_production_(nonterminal.clone(), expansion.clone());
                        for symbol in expansion.iter() {
                            if Grammar::is_nonterminal(symbol) {
                                stack.push(symbol.clone());
                            }
                        }
                    }
                }

                None => {
                    // A referenced nonterminal is not actually defined.
                    // The grammar is invalid.
                    return Err(format!(
                        "Nonterminal {} is referenced/used in the \
                        RHS but not defined in the LHS of any production rule",
                        nonterminal
                    ));
                }
            }
        }

        Ok(res)
    }

    /// Check that the given grammar satisfies some sensible rules.
    pub fn is_valid(&self) -> bool {
        match self.trim() {
            Ok(grammar) => *self == grammar,
            Err(_) => false,
        }
    }

    /// Determines if a given symbol name represents a nonterminal.
    /// This is only by convention and not actually enforced anywhere.
    fn is_nonterminal(s: &str) -> bool {
        s.starts_with("<") && s.ends_with(">")
    }
}

/// Derivation tree in a given grammar.
#[derive(Clone, Debug)]
pub enum Derivation {
    /// Nonterminal symbol (inner node in the tree) consisting of a symbol name
    /// and a list of child nodes / children.
    NT(String, Vec<Derivation>),
    /// Terminal symbol (leaf of the tree) consisting only of a symbol name
    /// (= final text for this tree part); it has no children.
    T(String),
}

// Shorthand functions for easier construction of derivation trees.
// Similar to grammar shorthand functions. Prefix `t` stands for `tree`.
#[rustfmt::skip]
fn tnt(name: &str, children: &[Derivation]) -> Derivation { Derivation::NT(name.to_string(), children.to_vec()) }
#[rustfmt::skip]
fn tt(name: &str)                           -> Derivation { Derivation::T(name.to_string()) }
fn ts(s: &str) -> Derivation {
    if s.starts_with("<") && s.ends_with(">") {
        tnt(s.trim_start_matches("<").trim_end_matches(">"), &[])
    } else {
        tt(s)
    }
}

impl Derivation {
    /// Returns a dot / graphviz definition of the derivation tree / graph.
    /// (Does iterative pre-order traversal of the tree).
    /// It can be rendered e.g. as follows: dot -Tpdf tree.dot -o tree.pdf
    pub fn to_dot(&self) -> String {
        let mut lines = Vec::new();

        lines.push("digraph Derivation {".to_string());
        lines.push("".to_string());
        lines.push("    node [shape=plain];".to_string());
        lines.push("".to_string());

        let mut node_count = 0;
        let mut queue: VecDeque<(&Derivation, Option<usize>)> = VecDeque::new();
        queue.push_back((self, None));

        while let Some((cur, parent)) = queue.pop_front() {
            node_count += 1;
            lines.push(format!(
                "    n{} [label=\"{}\"];",
                node_count,
                Derivation::to_dot_label(&cur.get_name())
            ));

            if let Some(parent) = parent {
                lines.push(format!("    n{} -> n{};", parent, node_count));
                lines.push("".to_string());
            }

            match cur {
                Derivation::NT(_, children) => {
                    for child in children.iter() {
                        queue.push_back((child, Some(node_count)));
                    }
                }

                Derivation::T(_) => {
                    // Edge to this node was already added previously.
                    // Since there are no children for terminal symbols, there
                    // is nothing left to do.
                }
            }
        }

        lines.push("}".to_string());
        lines.join("\n")
    }

    fn get_name(&self) -> String {
        match self {
            Derivation::NT(name, _) => format!("<{}>", name),
            Derivation::T(name) => format!("\"{}\"", name),
        }
    }

    fn to_dot_label(s: &str) -> String {
        s.chars()
            .map(|c| {
                if !(0x21 <= c as u32 && c as u32 <= 0x7d) {
                    "_".to_string()
                } else if [',', '<', '>', '\\', '"'].contains(&c) {
                    format!("\\{}", c)
                } else {
                    c.to_string()
                }
            })
            .collect()
    }

    /// Concatenate all leafs of the derivation tree (terminals, and yet
    /// unexpanded nonterminals) into one string.
    pub fn all_leafs(&self) -> String {
        let mut res: Vec<String> = Vec::new();
        self.all_leafs_(&mut res);
        res.join("")
    }

    fn all_leafs_(&self, res: &mut Vec<String>) {
        match self {
            Derivation::NT(name, children) => {
                if children.is_empty() {
                    res.push(format!(" <{}> ", name));
                }
                for child in children.iter() {
                    child.all_leafs_(res);
                }
            }

            Derivation::T(name) => res.push(name.clone()),
        }
    }

    /// Count the number of nodes that can be expanded (nonterminals that do
    /// not yet have any children assigned).
    fn possible_expansions(&self) -> usize {
        let mut res = 0;

        let mut queue: VecDeque<&Derivation> = VecDeque::new();
        queue.push_back(self);

        while let Some(cur) = queue.pop_front() {
            match cur {
                Derivation::NT(_, children) => {
                    if children.is_empty() {
                        res += 1;
                    }
                    for child in children.iter() {
                        queue.push_back(child);
                    }
                }

                Derivation::T(_) => (),
            }
        }

        res
    }

    fn any_possible_expansions(&self) -> bool {
        let mut queue: VecDeque<&Derivation> = VecDeque::new();
        queue.push_back(self);

        while let Some(cur) = queue.pop_front() {
            match cur {
                Derivation::NT(_, children) => {
                    if children.is_empty() {
                        return true;
                    }
                    for child in children.iter() {
                        queue.push_back(child);
                    }
                }

                Derivation::T(_) => (),
            }
        }

        false
    }
}

/// Create a random string from a context-free grammar.
pub fn fuzz(rng: &mut Rng, grammar: &Grammar) -> String {
    fuzz_tree(rng, grammar).all_leafs()
}

/// Create a random derivation tree from a context-free grammar.
pub fn fuzz_tree(rng: &mut Rng, grammar: &Grammar) -> Derivation {
    let empty_tree = Derivation::NT("start".to_string(), Vec::new());
    let tree = expand_tree(rng, grammar, empty_tree, 300, 500);
    tree
}

fn expand_tree(
    rng: &mut Rng,
    grammar: &Grammar,
    mut tree: Derivation,
    min_nonterminals: usize,
    max_nonterminals: usize,
) -> Derivation {
    while tree.possible_expansions() < min_nonterminals && tree.any_possible_expansions() {
        tree = expand_tree_once(rng, grammar, tree, ExpandStrategy::MaxCost);
    }

    while tree.possible_expansions() < max_nonterminals && tree.any_possible_expansions() {
        tree = expand_tree_once(rng, grammar, tree, ExpandStrategy::Random);
    }

    while tree.any_possible_expansions() {
        tree = expand_tree_once(rng, grammar, tree, ExpandStrategy::MinCost);
    }

    tree
}

fn expand_tree_once(
    rng: &mut Rng,
    grammar: &Grammar,
    tree: Derivation,
    strategy: ExpandStrategy,
) -> Derivation {
    match tree {
        Derivation::NT(name, children) => {
            if children.is_empty() {
                expand_node_by_strategy(rng, grammar, &Derivation::NT(name, children), strategy)
            } else {
                let expandable_children: Vec<_> = children
                    .iter()
                    .enumerate()
                    .filter_map(|(i, c)| {
                        if c.any_possible_expansions() {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();
                let i = *rng.choice(&expandable_children);

                let children = children
                    .into_iter()
                    .enumerate()
                    .map(|(j, c)| {
                        if i == j {
                            expand_tree_once(rng, grammar, c, strategy.clone())
                        } else {
                            c
                        }
                    })
                    .collect();

                Derivation::NT(name, children)
            }
        }
        Derivation::T(_) => tree,
    }
}

/// Minimum cost of all expansions of a symbol. Infinite recursion is mapped
/// to the value `Infinite`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum SymbolCost {
    Finite(usize),
    Infinite,
}

impl std::ops::Add for SymbolCost {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (SymbolCost::Finite(a), SymbolCost::Finite(b)) => SymbolCost::Finite(a + b),
            (SymbolCost::Infinite, _) => SymbolCost::Infinite,
            (_, SymbolCost::Infinite) => SymbolCost::Infinite,
        }
    }
}

fn symbol_cost_(grammar: &Grammar, symbol: &str, seen: &BTreeSet<String>) -> SymbolCost {
    let mut min = SymbolCost::Infinite;
    for expansion in grammar.0.get(symbol).unwrap() {
        let mut seen = seen.clone();
        seen.insert(symbol.to_string());
        let tmp = expansion_cost(grammar, expansion, &seen);
        min = std::cmp::min(tmp, min);
    }
    min
}

fn expansion_cost(grammar: &Grammar, expansion: &Expansion, seen: &BTreeSet<String>) -> SymbolCost {
    let nonterminals: Vec<_> = expansion
        .iter()
        .filter(|symbol| Grammar::is_nonterminal(symbol))
        .collect();
    if nonterminals.iter().any(|symbol| seen.contains(*symbol)) {
        SymbolCost::Infinite
    } else {
        nonterminals
            .iter()
            .map(|symbol| symbol_cost_(grammar, symbol, seen))
            .fold(SymbolCost::Finite(0), |acc, x| acc + x)
            + SymbolCost::Finite(1)
    }
}

#[derive(Clone, Debug)]
enum ExpandStrategy {
    MinCost,
    Random,
    MaxCost,
}

fn expand_node_by_strategy(
    rng: &mut Rng,
    grammar: &Grammar,
    tree: &Derivation,
    strategy: ExpandStrategy,
) -> Derivation {
    let name = tree.get_name();
    let expansions = grammar
        .0
        .get(&name)
        .expect(&format!("Couldn't get expansion for symbol {}", name));

    let mut seen = BTreeSet::new();
    seen.insert(name.clone());

    let expansion = match strategy {
        ExpandStrategy::Random => rng.choice(expansions),
        ExpandStrategy::MinCost | ExpandStrategy::MaxCost => {
            let costs: Vec<_> = expansions
                .into_iter()
                .map(|expansion| (expansion, expansion_cost(grammar, expansion, &seen)))
                .collect();

            let cost = match strategy {
                ExpandStrategy::MinCost => costs.iter().map(|(_, c)| c).min().unwrap().clone(),
                ExpandStrategy::MaxCost => costs.iter().map(|(_, c)| c).max().unwrap().clone(),
                _ => panic!("Can't happen"),
            };

            let choices: Vec<_> = costs
                .into_iter()
                .filter(|(_, c)| match strategy {
                    ExpandStrategy::MinCost => *c <= cost,
                    ExpandStrategy::MaxCost => *c >= cost,
                    _ => panic!("Can't happen"),
                })
                .map(|(exp, _)| exp)
                .collect();

            *rng.choice(&choices)
        }
    };
    let expansion = expansion.into_iter().map(|s| ts(s)).collect::<Vec<_>>();

    Derivation::NT(
        name.trim_start_matches("<")
            .trim_end_matches(">")
            .to_string(),
        expansion,
    )
}
