// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use std::collections::VecDeque;
use std::collections::{BTreeMap, BTreeSet};

use crate::rng::Rng;

pub fn expr_grammar_ebnf() -> Ebnf {
    let mut grammar = Ebnf::new();

    grammar.add_production("<start>", s("<expr>"));

    grammar.add_production(
        "<expr>",
        alt(&[
            seq(&[s("<term>"), s("+"), s("<expr>")]),
            seq(&[s("<term>"), s("-"), s("<expr>")]),
            s("<term>"),
        ]),
    );

    grammar.add_production(
        "<term>",
        alt(&[
            seq(&[s("<factor>"), s("*"), s("<term>")]),
            seq(&[s("<factor>"), s("/"), s("<term>")]),
            s("<factor>"),
        ]),
    );

    grammar.add_production(
        "<factor>",
        alt(&[
            seq(&[opt(s("<sign>")), s("<factor>")]),
            seq(&[s("("), s("<expr>"), s(")")]),
            seq(&[s("<integer>"), opt(seq(&[s("."), s("<integer>")]))]),
        ]),
    );

    grammar.add_production("<sign>", alt(&[s("+"), s("-")]));

    grammar.add_production("<integer>", plus(s("<digit>")));

    grammar.add_production(
        "<digit>",
        Expr::Alt(Ebnf::to_terminals(&(0..10).collect::<Vec<_>>())),
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
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Add a single production of the form: nonterminal -> [symbols]
    /// to the grammar.
    fn add_production(&mut self, nonterminal: &str, expansion: &[&str]) {
        self.add_production_(
            nonterminal.to_string(),
            expansion.iter().map(|x| x.to_string()).collect(),
        );
    }

    /// Add a single production of the form: nonterminal -> [symbols]
    /// to the grammar (for owned values).
    fn add_production_(&mut self, nonterminal: Nonterminal, expansion: Expansion) {
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

    /// Create a copy of the grammar with only the actually used/reachable
    /// production rules. Fails if the grammar is invalid.
    fn trim(&self) -> Result<Grammar, String> {
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
    fn is_valid(&self) -> bool {
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

/// Context-free-grammar with support for EBNF constructs.
#[derive(PartialEq, Eq)]
pub struct Ebnf(BTreeMap<Nonterminal, Expr>);

/// EBNF syntax expression.
#[derive(Clone, PartialEq, Eq)]
pub enum Expr {
    Alt(Vec<Expr>),  // Alternative/choice between elements.
    Seq(Vec<Expr>),  // Sequence of elements.
    Opt(Box<Expr>),  // Optional occurrence of zero or one times (?).
    Plus(Box<Expr>), // Occurrence of one or more times (+).
    Star(Box<Expr>), // Occurrence of an arbitrary number of times (including zero) (*).
    NT(String),      // Nonterminal symbol.
    T(String),       // Terminal symbol.
}

// Shorthand functions for easier construction of Expr variants.
// (Handle cloning/boxing/slicing).
#[rustfmt::skip]
fn alt(expr: &[Expr])   -> Expr { Expr::Alt(expr.to_vec()) }
#[rustfmt::skip]
fn seq(expr: &[Expr])   -> Expr { Expr::Seq(expr.to_vec()) }
#[rustfmt::skip]
fn opt(expr: Expr)      -> Expr { Expr::Opt(Box::new(expr)) }
#[rustfmt::skip]
fn plus(expr: Expr)     -> Expr { Expr::Plus(Box::new(expr)) }
#[rustfmt::skip]
fn star(expr: Expr)     -> Expr { Expr::Star(Box::new(expr)) }
#[rustfmt::skip]
fn nt(s: &str)          -> Expr { Expr::NT(s.to_string()) }
#[rustfmt::skip]
fn t(s: &str)           -> Expr { Expr::T(s.to_string()) }

/// Create new symbol and dispatch to nonterminal or terminal symbol based
/// on the name and wether it is enclosed in angle brackets or not.
fn s(s: &str) -> Expr {
    if s.starts_with("<") && s.ends_with(">") {
        nt(s.trim_start_matches("<").trim_end_matches(">"))
    } else {
        t(s)
    }
}

impl std::fmt::Display for Ebnf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let maxnonterminallength = self.0.keys().map(|x| x.len()).max().unwrap_or(10);
        for (nonterminal, expr) in self.0.iter() {
            writeln!(f, "{:maxnonterminallength$} -> {}", nonterminal, expr)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Alt(v) => write!(
                f,
                "{}",
                v.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("  |  ")
            ),
            Expr::Seq(v) => write!(
                f,
                "{}",
                v.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Expr::Opt(expr) => write!(f, "({})?", expr),
            Expr::Plus(expr) => write!(f, "({})+", expr),
            Expr::Star(expr) => write!(f, "({})*", expr),
            Expr::NT(s) => write!(f, "<{}>", s),
            Expr::T(s) => write!(f, "\"{}\"", s),
        }
    }
}

impl Ebnf {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Add a production rule to the grammar.
    fn add_production(&mut self, nonterminal: &str, expr: Expr) {
        match self.0.get_mut(nonterminal) {
            Some(_) => panic!(
                "Can't add production for same nonterminal twice {}",
                nonterminal
            ),
            None => {
                self.0.insert(nonterminal.to_string(), expr);
            }
        }
    }

    /// Convert a slice of printable values to a list of terminal expressions.
    /// This allows easy construction of alternatives of ranges/iterators.
    fn to_terminals<T: ToString>(v: &[T]) -> Vec<Expr> {
        let mut res = Vec::new();
        for el in v.iter() {
            res.push(Expr::T(el.to_string()));
        }
        res
    }

    /// Convert a grammar from EBNF to BNF by replacing regular language constructs
    /// / expressions with direct production rules.
    pub fn to_bnf(&self) -> Grammar {
        let mut bnf = Grammar::new();

        // Iterate over each production rule and expand out and flatten all extended
        // syntax constructs.
        for (nonterminal, expression) in self.0.iter() {
            let mut symbolcounter = 0; // Needed for generating fresh new symbol names.
            let expansions = Ebnf::to_bnf_expr(&mut bnf, expression, &mut symbolcounter);
            for expansion in expansions.into_iter() {
                bnf.add_production_(nonterminal.to_string(), expansion);
            }
        }

        bnf
    }

    /// Generate a unique nonterminal symbol name that does not yet occur in the
    /// given grammar.
    fn new_nonterminal(bnf: &Grammar, i: &mut usize) -> String {
        loop {
            let symbol = format!("<symbol{}>", i);
            if !bnf.0.contains_key(&symbol) {
                return symbol;
            }
            *i += 1;
        }
    }

    /// Convert an EBNF expression into our BNF CFG grammar representation.
    /// This requires translating regular constructs like `?`/`+`/"`*`,
    /// as well as fully flattening nested groupings (alternatives and sequences).
    fn to_bnf_expr(bnf: &mut Grammar, expression: &Expr, i: &mut usize) -> Vec<Expansion> {
        match expression {
            // Alternatives are represented as top-level Vecs.
            Expr::Alt(exprs) => {
                let mut res = Vec::new();
                for expr in exprs {
                    res.extend(Ebnf::to_bnf_expr(bnf, expr, i));
                }
                res
            }

            // Sequences are represented as inner Vecs.
            // Therefore we need to expand each nested expression.
            // If an expression expands to multiple alternatives or to one
            // alternative with multiple elements in the sequence, we need
            // to introduce a new nonterminal symbol and insert one level of
            // indirection, in order to be able to fully flatten the grammar
            // representation.
            Expr::Seq(exprs) => {
                let mut res = Vec::new();
                for expr in exprs {
                    let expr_expansions = Ebnf::to_bnf_expr(bnf, expr, i);
                    if expr_expansions.len() == 1 && expr_expansions[0].len() == 1 {
                        // We can shortcut and don't need to add a useless new
                        // intermediate nonterminal symbol that would only expand
                        // to *one single* other symbol anyway.
                        res.push(expr_expansions[0][0].clone());
                    } else {
                        let s = Ebnf::new_nonterminal(bnf, i);
                        for expr_expansion in expr_expansions.into_iter() {
                            bnf.add_production_(s.clone(), expr_expansion);
                        }
                        res.push(s);
                    }
                }
                vec![res]
            }

            // > An expression <symbol>? becomes <new-symbol>, where <new-symbol> ::= <empty>  | <symbol>.
            // Since an expression can expand to multiple alternatives/sequences,
            // we need to perform this substitution for all possible candidates.
            Expr::Opt(expr) => {
                let s = Ebnf::new_nonterminal(bnf, i);
                let expr_expansions = Ebnf::to_bnf_expr(bnf, expr, i);
                for expr_expansion in expr_expansions.into_iter() {
                    bnf.add_production_(s.clone(), expr_expansion);
                }
                // Since the empty string / epsilon does not depend on the expansion,
                // we can avoid duplicates and insert it once at the end (and not
                // over and over inside the loop).
                bnf.add_production_(s.clone(), vec!["".to_string()]);
                vec![vec![s]]
            }

            // > An expression <symbol>+ becomes <new-symbol>, where <new-symbol> ::= <symbol> | <symbol><new-symbol>.
            // Since an expression can expand to multiple alternatives/sequences,
            // we need to perform this substitution for all possible candidates.
            Expr::Plus(expr) => {
                let s = Ebnf::new_nonterminal(bnf, i);
                let expr_expansions = Ebnf::to_bnf_expr(bnf, expr, i);
                for mut expr_expansion in expr_expansions.into_iter() {
                    bnf.add_production_(s.clone(), expr_expansion.clone());
                    expr_expansion.push(s.clone());
                    bnf.add_production_(s.clone(), expr_expansion);
                }
                vec![vec![s]]
            }

            // > An expression <symbol>* becomes <new-symbol>, where <new-symbol> ::= <empty>  | <symbol><new-symbol>.
            // Since an expression can expand to multiple alternatives/sequences,
            // we need to perform this substitution for all possible candidates.
            Expr::Star(expr) => {
                let s = Ebnf::new_nonterminal(bnf, i);
                let expr_expansions = Ebnf::to_bnf_expr(bnf, expr, i);
                for mut expr_expansion in expr_expansions.into_iter() {
                    expr_expansion.push(s.clone());
                    bnf.add_production_(s.clone(), expr_expansion);
                }
                // Since the empty string / epsilon does not depend on the expansion,
                // we can avoid duplicates and insert it once at the end (and not
                // over and over inside the loop).
                bnf.add_production_(s.clone(), vec!["".to_string()]);
                vec![vec![s]]
            }

            Expr::NT(s) => vec![vec![format!("<{}>", s)]],
            Expr::T(s) => vec![vec![s.clone()]],
        }
    }

    /// Create a copy of the grammar with only the actually used/reachable
    /// production rules. Fails if the grammar is invalid.
    fn trim(&self) -> Result<Ebnf, String> {
        let mut res = Ebnf::new();

        let mut seen_nonterminals = BTreeSet::new();

        let mut stack_nonterminals = Vec::new();
        stack_nonterminals.push("<start>".to_string());

        // Iterate over all reachable nonterminals/production rules and add each
        // production rule to the new grammar.
        while let Some(nonterminal) = stack_nonterminals.pop() {
            if seen_nonterminals.contains(&nonterminal) {
                continue;
            }
            seen_nonterminals.insert(nonterminal.clone());

            if !self.0.contains_key(&nonterminal) {
                // A referenced nonterminal is not actually defined.
                // The grammar is invalid.
                return Err(format!(
                    "Nonterminal {} is referenced/used in the \
                        RHS but not defined in the LHS of any production rule",
                    nonterminal
                ));
            }

            // Iterate over the expression and extract all nonterminals.
            let expr_root = self.0.get(&nonterminal).unwrap();
            res.add_production(&nonterminal, expr_root.clone());

            let mut stack_exprs: Vec<&Expr> = Vec::new();
            stack_exprs.push(expr_root);

            while let Some(expr) = stack_exprs.pop() {
                match expr {
                    Expr::Alt(exprs) => stack_exprs.extend(exprs),
                    Expr::Seq(exprs) => stack_exprs.extend(exprs),
                    Expr::Opt(expr) => stack_exprs.push(expr),
                    Expr::Plus(expr) => stack_exprs.push(expr),
                    Expr::Star(expr) => stack_exprs.push(expr),
                    Expr::NT(s) => stack_nonterminals.push(s.clone()),
                    Expr::T(_) => (),
                }
            }
        }

        Ok(res)
    }

    fn is_valid(&self) -> bool {
        match self.trim() {
            Ok(grammar) => *self == grammar,
            Err(_) => false,
        }
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
pub fn tnt(name: &str, children: &[Derivation]) -> Derivation { Derivation::NT(name.to_string(), children.to_vec()) }
#[rustfmt::skip]
pub fn tt(name: &str)                           -> Derivation { Derivation::T(name.to_string()) }
pub fn ts(s: &str) -> Derivation {
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
            Derivation::T(name) => name.clone(),
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

        let mut queue: VecDeque<&Derivation> = VecDeque::new();
        queue.push_back(self);

        while let Some(cur) = queue.pop_front() {
            match cur {
                Derivation::NT(name, children) => {
                    if children.is_empty() {
                        res.push(format!(" <{}> ", name));
                    }

                    for child in children.iter() {
                        queue.push_back(child);
                    }
                }

                Derivation::T(name) => res.push(name.clone()),
            }
        }

        res.join("")
    }

    /// Count the number of nodes that can be expanded (nonterminals that do
    /// not yet have any children assigned).
    pub fn possible_expansions(&self) -> usize {
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

    pub fn any_possible_expansions(&self) -> bool {
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
    let mut tree = Derivation::NT("start".to_string(), Vec::new());
    let possible_expansions = grammar
        .0
        .get("expr")
        .expect(&format!("Couldn't get key {}", "exprTODO"));

    let random_expansion = rng.choice(possible_expansions);

    fuzz_tree_(rng, grammar, &mut tree);
    tree
}

pub fn fuzz_tree_(rng: &mut Rng, grammar: &Grammar, derivation: &mut Derivation) {
    todo!()
}

pub fn expand_tree_once(rng: &mut Rng, grammar: &Grammar, tree: Derivation) -> Derivation {
    match tree {
        Derivation::NT(name, children) => {
            if children.is_empty() {
                expand_node(rng, grammar, &Derivation::NT(name, children))
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
                            expand_tree_once(rng, grammar, c)
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

pub fn expand_node(rng: &mut Rng, grammar: &Grammar, tree: &Derivation) -> Derivation {
    // expand_node_randomly(rng, grammar, tree)
    expand_node_min_cost(grammar, tree)
}

fn expand_node_randomly(rng: &mut Rng, grammar: &Grammar, tree: &Derivation) -> Derivation {
    let name = tree.get_name();
    let expansions = grammar
        .0
        .get(&name)
        .expect(&format!("Couldn't get expansion for symbol {}", name));

    let random_expansion = rng.choice(expansions);
    let random_expansion = random_expansion
        .into_iter()
        .map(|s| ts(s))
        .collect::<Vec<_>>();

    Derivation::NT(
        name.trim_start_matches("<")
            .trim_end_matches(">")
            .to_string(),
        random_expansion,
    )
}

/// Minimum cost of all expansions of a symbol. Infinite recursion is mapped
/// to the value `Infinite`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SymbolCost {
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

pub fn symbol_cost(grammar: &Grammar, symbol: &str) -> SymbolCost {
    symbol_cost_(grammar, symbol, &BTreeSet::new())
}

pub fn symbol_cost_(grammar: &Grammar, symbol: &str, seen: &BTreeSet<String>) -> SymbolCost {
    let mut min = SymbolCost::Infinite;
    for expansion in grammar.0.get(symbol).unwrap() {
        let mut seen = seen.clone();
        seen.insert(symbol.to_string());
        let tmp = expansion_cost(grammar, expansion, &seen);
        min = std::cmp::min(tmp, min);
    }
    min
}

pub fn expansion_cost(
    grammar: &Grammar,
    expansion: &Expansion,
    seen: &BTreeSet<String>,
) -> SymbolCost {
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

enum ExpandStrategy {
    MinCost,
    MaxCost,
}

fn expand_node_min_cost(grammar: &Grammar, tree: &Derivation) -> Derivation {
    expand_node_by_cost(grammar, tree, ExpandStrategy::MinCost)
}

fn expand_node_max_cost(grammar: &Grammar, tree: &Derivation) -> Derivation {
    expand_node_by_cost(grammar, tree, ExpandStrategy::MaxCost)
}

fn expand_node_by_cost(
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
        ExpandStrategy::MinCost => expansions
            .iter()
            .min_by_key(|expansion| expansion_cost(grammar, expansion, &seen)),
        ExpandStrategy::MaxCost => expansions
            .iter()
            .max_by_key(|expansion| expansion_cost(grammar, expansion, &seen)),
    }
    .unwrap();
    let expansion = expansion.into_iter().map(|s| ts(s)).collect::<Vec<_>>();

    Derivation::NT(
        name.trim_start_matches("<")
            .trim_end_matches(">")
            .to_string(),
        expansion,
    )
}
