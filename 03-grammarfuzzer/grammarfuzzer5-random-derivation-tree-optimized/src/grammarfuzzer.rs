// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

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
pub struct Grammar(HashMap<Nonterminal, Vec<Expansion>>);
pub type Nonterminal = String;
pub type Expansion = Vec<String>; // Right-hand-side of a production rule.

/// Context-free grammar annotated with pre-computed cost values for symbols /
/// expansions.
pub struct GrammarCost {
    grammar: Grammar,
    cost_by_expansion: HashMap<Expansion, SymbolCost>,
}

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
        Self(HashMap::new())
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
        let expansions: Vec<_> = expansions.iter().map(|x| vec![x.to_string()]).collect();
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
        let mut seen_nonterminals = HashSet::new();

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

    /// Trim nonterminal symbol name angle brackets.
    fn trim_angle_brackets(s: &str) -> &str {
        s.trim_start_matches("<").trim_end_matches(">")
    }
}

/// Pre-compute expansion costs.
impl std::convert::From<Grammar> for GrammarCost {
    fn from(grammar: Grammar) -> Self {
        let mut cost_by_expansion = HashMap::new();

        for (_symbol, expansions) in grammar.0.iter() {
            for expansion in expansions.iter() {
                cost_by_expansion.insert(
                    expansion.clone(),
                    expansion_cost(&grammar, expansion, &HashSet::new()),
                );
            }
        }

        Self {
            grammar,
            cost_by_expansion,
        }
    }
}

/// Derivation tree in a given grammar.
#[derive(Clone, Debug)]
pub enum Tree {
    /// Nonterminal symbol (inner node in the tree) consisting of a symbol name
    /// and a list of child nodes / children.
    NT(String, Vec<Tree>),
    /// Terminal symbol (leaf of the tree) consisting only of a symbol name
    /// (= final text for this tree part); it has no children.
    T(String),
}

// Shorthand functions for easier construction of derivation trees.
// Similar to grammar shorthand functions. Prefix `t` stands for `tree`.
#[rustfmt::skip]
fn tnt(name: &str, children: &[Tree]) -> Tree { Tree::NT(name.to_string(), children.to_vec()) }
#[rustfmt::skip]
fn tt(name: &str)                     -> Tree { Tree::T(name.to_string()) }
fn ts(s: &str) -> Tree {
    if Grammar::is_nonterminal(s) {
        tnt(Grammar::trim_angle_brackets(s), &[])
    } else {
        tt(s)
    }
}

impl Tree {
    /// Returns a dot / graphviz definition of the derivation tree / graph.
    /// (Does iterative pre-order traversal of the tree).
    /// It can be rendered e.g. as follows: dot -Tpdf tree.dot -o tree.pdf
    pub fn to_dot(&self) -> String {
        let mut lines = Vec::new();

        lines.push("digraph DerivationTree {".to_string());
        lines.push("".to_string());
        lines.push("    node [shape=plain];".to_string());
        lines.push("".to_string());

        let mut node_count = 0;
        let mut queue: VecDeque<(&Tree, Option<usize>)> = VecDeque::new();
        queue.push_back((self, None));

        while let Some((cur, parent)) = queue.pop_front() {
            node_count += 1;
            lines.push(format!(
                "    n{} [label=\"{}\"];",
                node_count,
                Tree::to_dot_label(&cur.get_name())
            ));

            if let Some(parent) = parent {
                lines.push(format!("    n{} -> n{};", parent, node_count));
                lines.push("".to_string());
            }

            match cur {
                Tree::NT(_, children) => {
                    for child in children.iter() {
                        queue.push_back((child, Some(node_count)));
                    }
                }

                Tree::T(_) => {
                    // Edge to this node was already added previously.
                    // Since there are no children for terminal symbols, there
                    // is nothing left to do.
                }
            }
        }

        lines.push("}".to_string());
        lines.join("\n")
    }

    /// Get the symbol name as a string. Depending on the kind of symbol, the
    /// symbol name is wrapped into either double quotes (terminal symbol), or
    /// angle brackets (nonterminal symbols).
    fn get_name(&self) -> String {
        match self {
            Tree::NT(name, _) => format!("<{}>", name),
            Tree::T(name) => format!("\"{}\"", name),
        }
    }

    /// Escape symbol name for usage as vertex/node label in a dot/graphviz file.
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
            Tree::NT(name, children) => {
                if children.is_empty() {
                    res.push(format!(" <{}> ", name));
                }
                for child in children.iter() {
                    child.all_leafs_(res);
                }
            }

            Tree::T(name) => res.push(name.clone()),
        }
    }

    /// Collect pointers to nodes that can be expanded (nonterminals that do not
    /// yet have any children assigned).
    fn get_expandable_nonterminals(&mut self) -> Vec<&mut Tree> {
        let mut res: Vec<&mut Tree> = Vec::new();

        let mut queue: VecDeque<&mut Tree> = VecDeque::new();
        queue.push_back(self);

        while let Some(cur) = queue.pop_front() {
            // We first determine whether this node is a nonterminal with empty
            // / no children (then it is expandable).
            // As far as I know, we can't do what we want here in a single match
            // since we would then have to borrow children either as mutable
            // (for iterating over them and pushing mutable refs to the queue)
            // or as immutable (for pushing cur to the result list), depending
            // on its inner/destructured value.

            let mut expandable = false;
            if let Tree::NT(_, children) = cur {
                if children.is_empty() {
                    expandable = true;
                }
            }

            if expandable {
                res.push(cur);
            } else {
                // `if` is only there for destructuring.
                if let Tree::NT(_, children) = cur {
                    for child in children.iter_mut() {
                        queue.push_back(child);
                    }
                }
            }
        }

        res
    }
}

/// Create a random string from a context-free grammar.
pub fn fuzz(rng: &mut Rng, grammar: Grammar) -> String {
    fuzz_tree(rng, grammar).all_leafs()
}

/// Create a random derivation tree from a context-free grammar.
pub fn fuzz_tree(rng: &mut Rng, grammar: Grammar) -> Tree {
    let grammar_cost: GrammarCost = grammar.into();
    let mut tree = Tree::NT("start".to_string(), Vec::new());
    expand_tree(rng, &grammar_cost, &mut tree, 10, 500);
    tree
}

/// Expand nonterminals in the derivation tree in three phases:
///
///   1. Increase as much as possible by choosing expansions that lead to largest
///      number of children.
///
///   2. Randomly expand leaf-nonterminals.
///
///   3. Shrink as much as possible by choosing expansions that lead to smallest
///      number of children.
fn expand_tree(
    rng: &mut Rng,
    grammar: &GrammarCost,
    tree: &mut Tree,
    min_nonterminals: usize,
    max_nonterminals: usize,
) {
    // Traverse down the tree to find non-expanded leaf-nonterminals.
    let mut expandable = tree.get_expandable_nonterminals();

    // Max expansion (increase size as much as possible).
    while !expandable.is_empty() && expandable.len() < min_nonterminals {
        expand_node_by_strategy(rng, grammar, &mut expandable, ExpandStrategy::MaxCost);
    }

    // Random expansion.
    while !expandable.is_empty() && expandable.len() < max_nonterminals {
        expand_node_by_strategy(rng, grammar, &mut expandable, ExpandStrategy::Random);
    }

    // Min expansion (increase size as little as possible / shrink).
    while !expandable.is_empty() {
        expand_node_by_strategy(rng, grammar, &mut expandable, ExpandStrategy::MinCost);
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

fn symbol_cost(grammar: &Grammar, symbol: &str, seen: &HashSet<String>) -> SymbolCost {
    let mut min = SymbolCost::Infinite;
    for expansion in grammar
        .0
        .get(symbol)
        .unwrap_or_else(|| panic!("Couldn't get expansion for symbol {}", symbol))
    {
        let mut seen = seen.clone();
        seen.insert(symbol.to_string());
        let tmp = expansion_cost(grammar, expansion, &seen);
        min = std::cmp::min(tmp, min);
    }
    min
}

fn expansion_cost(grammar: &Grammar, expansion: &Expansion, seen: &HashSet<String>) -> SymbolCost {
    let nonterminals: Vec<_> = expansion
        .iter()
        .filter(|symbol| Grammar::is_nonterminal(symbol))
        .collect();
    if nonterminals.iter().any(|symbol| seen.contains(*symbol)) {
        SymbolCost::Infinite
    } else {
        nonterminals
            .iter()
            .map(|symbol| symbol_cost(grammar, symbol, seen))
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

/// Expand a leaf-non-terminal symbol with rules from a specific grammar
/// while following a specific expansion strategy.
fn expand_node_by_strategy(
    rng: &mut Rng,
    grammar: &GrammarCost,
    expandable: &mut Vec<&mut Tree>,
    strategy: ExpandStrategy,
) {
    // Choose random not-yet-expanded nonterminal symbol / node.
    let treeidx = rng.int(expandable.len() as u64) as usize;
    let tree: &mut Tree = expandable.remove(treeidx);

    // I don't know how to assert destructured enum values concisely...
    // All these conditions should have been checked before calling this function.
    if let Tree::NT(_, children) = tree {
        if !children.is_empty() {
            panic!("Can't happen");
        }
    } else {
        panic!("Can't happen");
    }

    let name = tree.get_name();
    let expansions = grammar
        .grammar
        .0
        .get(&name)
        .unwrap_or_else(|| panic!("Couldn't get expansion for symbol {}", name));

    let expansion = match strategy {
        ExpandStrategy::Random => rng.choice(expansions),
        ExpandStrategy::MinCost | ExpandStrategy::MaxCost => {
            let costs: Vec<_> = expansions
                .iter()
                .map(|expansion| (expansion, grammar.cost_by_expansion.get(expansion).unwrap()))
                .collect();

            let cost = match strategy {
                ExpandStrategy::MinCost => *costs.iter().map(|(_, c)| c).min().unwrap(),
                ExpandStrategy::MaxCost => *costs.iter().map(|(_, c)| c).max().unwrap(),
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

            // Randomly choose expansion from all valid expansions.
            *rng.choice(&choices)
        }
    };
    let expansion = expansion.iter().map(|s| ts(s)).collect::<Vec<_>>();

    // Modify derivation tree with expanded children.
    *tree = Tree::NT(Grammar::trim_angle_brackets(&name).to_string(), expansion);

    // Update expandable nonterminals: Add newly created not-yet expanded
    // nonterminals / tree leafs to the list.
    match tree {
        Tree::NT(_, children) => {
            for symbol in children.iter_mut() {
                match symbol {
                    Tree::NT(_, children2) => {
                        assert!(children2.is_empty());
                        expandable.push(symbol);
                    }
                    _ => (), // Ignore terminal symbols.
                }
            }
        }
        _ => panic!("Can't happen"),
    }
}
