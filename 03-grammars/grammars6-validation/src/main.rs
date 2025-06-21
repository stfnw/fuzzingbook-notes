// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// https://www.fuzzingbook.org/html/Grammars.html

use std::collections::{BTreeMap, BTreeSet};

// [+] Expression grammar
// <digit>   -> "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
// <expr>    -> <term> "+" <expr> | <term> "-" <expr> | <term>
// <factor>  -> "+" <factor> | "-" <factor> | "(" <expr> ")" | <integer> "." <integer> | <integer>
// <integer> -> <digit> <integer> | <digit>
// <start>   -> <expr>
// <term>    -> <factor> "*" <term> | <factor> "/" <term> | <factor>
//
// [+] Expression grammar (minimized)
// <digit>   -> "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
// <expr>    -> <term> "+" <expr> | <term> "-" <expr> | <term>
// <factor>  -> "+" <factor> | "-" <factor> | "(" <expr> ")" | <integer> "." <integer> | <integer>
// <integer> -> <digit> <integer> | <digit>
// <start>   -> <expr>
// <term>    -> <factor> "*" <term> | <factor> "/" <term> | <factor>
//
// [+] Is expression grammar valid: true

// [+] CGI grammar
// <hexdigit> -> "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
// <letter>   -> <plus> | <percent> | <other>
// <other>    -> "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "-" | "_"
// <percent>  -> "%" <hexdigit> <hexdigit>
// <plus>     -> "+"
// <start>    -> <string>
// <string>   -> <letter> | <letter> <string>
//
// [+] CGI grammar (minimized)
// <hexdigit> -> "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
// <letter>   -> <plus> | <percent> | <other>
// <other>    -> "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "-" | "_"
// <percent>  -> "%" <hexdigit> <hexdigit>
// <plus>     -> "+"
// <start>    -> <string>
// <string>   -> <letter> | <letter> <string>
//
// [+] Is CGI grammar valid: true

// [+] Title grammar
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
//
// [+] Title grammar (minimized)
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
//
// [+] Is title grammar valid: true

// [+] EBNF expression grammar
// <start> -> <expr>
// digit   -> "0"  |  "1"  |  "2"  |  "3"  |  "4"  |  "5"  |  "6"  |  "7"  |  "8"  |  "9"
// expr    -> <term> "+" <expr>  |  <term> "-" <expr>  |  <term>
// factor  -> (<sign>)? <factor>  |  "(" <expr> ")"  |  <integer> ("." <integer>)?
// integer -> (<digit>)+
// sign    -> "+"  |  "-"
// term    -> <factor> "*" <term>  |  <factor> "/" <term>  |  <factor>
//
// [+] EBNF expression grammar (minimized)
// <start> -> <expr>
// digit   -> "0"  |  "1"  |  "2"  |  "3"  |  "4"  |  "5"  |  "6"  |  "7"  |  "8"  |  "9"
// expr    -> <term> "+" <expr>  |  <term> "-" <expr>  |  <term>
// factor  -> (<sign>)? <factor>  |  "(" <expr> ")"  |  <integer> ("." <integer>)?
// integer -> (<digit>)+
// sign    -> "+"  |  "-"
// term    -> <factor> "*" <term>  |  <factor> "/" <term>  |  <factor>
//
// [+] Is EBNF expression grammar valid: false

// [+] Tmp grammar: <start> -> <x>
// <y>     -> "1"
//
// [+] Is grammar valid: false
//
// [+] Tmp grammar: <start> -> <x>
// <y>     -> "1"
//
// [+] Is grammar valid: false

fn main() {
    let tmp = expr_grammar();
    println!("[+] Expression grammar");
    println!("{}", tmp);
    println!("[+] Expression grammar (minimized)");
    println!("{}", tmp.trim().unwrap());
    println!("[+] Is expression grammar valid: {}", tmp.is_valid());
    println!();
    println!();

    let tmp = cgi_grammar();
    println!("[+] CGI grammar");
    println!("{}", tmp);
    println!("[+] CGI grammar (minimized)");
    println!("{}", tmp.trim().unwrap());
    println!("[+] Is CGI grammar valid: {}", tmp.is_valid());
    println!();
    println!();

    let tmp = title_grammar();
    println!("[+] Title grammar");
    println!("{}", tmp);
    println!("[+] Title grammar (minimized)");
    println!("{}", tmp.trim().unwrap());
    println!("[+] Is title grammar valid: {}", tmp.is_valid());
    println!();
    println!();

    let mut tmp = expr_grammar_ebnf();
    println!("[+] EBNF expression grammar");
    println!("{}", tmp);
    // Add superfluous rule that is then removed during trimming.
    tmp.add_production("t", t("t"));
    println!("[+] EBNF expression grammar (minimized)");
    println!("{}", tmp.trim().unwrap());
    println!("[+] Is EBNF expression grammar valid: {}", tmp.is_valid());
    println!();
    println!();

    let mut tmp = Grammar::new();
    tmp.add_production("<start>", &["<x>"]);
    tmp.add_production("<y>", &["1"]);
    println!("[+] Tmp grammar: {}", tmp);
    println!("[+] Is grammar valid: {}", tmp.is_valid());
    println!();
    println!();
}

fn expr_grammar() -> Grammar {
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

fn cgi_grammar() -> Grammar {
    let mut grammar = Grammar::new();

    grammar.add_production("<start>", &["<string>"]);

    grammar.add_production("<string>", &["<letter>"]);
    grammar.add_production("<string>", &["<letter>", "<string>"]);

    grammar.add_production("<letter>", &["<plus>"]);
    grammar.add_production("<letter>", &["<percent>"]);
    grammar.add_production("<letter>", &["<other>"]);

    grammar.add_production("<plus>", &["+"]);

    grammar.add_production("<percent>", &["%", "<hexdigit>", "<hexdigit>"]);

    for i in 0..10 {
        grammar.add_production("<hexdigit>", &[format!("{}", i).as_str()]);
    }

    let other: Vec<_> = ((0..26).map(|x| char::from(x + ('a' as u8)).to_string()))
        .chain((0..10).map(|x| x.to_string()))
        .chain(["-", "_"].into_iter().map(|x| x.to_string()))
        .collect();
    grammar.add_productions(
        "<other>",
        &other.iter().map(|x| x.as_str()).collect::<Vec<_>>(),
    );

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

fn expr_grammar_ebnf() -> Ebnf {
    let mut grammar = Ebnf::new();

    grammar.add_production("<start>", s("<expr>"));

    grammar.add_production(
        "expr",
        alt(&[
            seq(&[s("<term>"), s("+"), s("<expr>")]),
            seq(&[s("<term>"), s("-"), s("<expr>")]),
            s("<term>"),
        ]),
    );

    grammar.add_production(
        "term",
        alt(&[
            seq(&[s("<factor>"), s("*"), s("<term>")]),
            seq(&[s("<factor>"), s("/"), s("<term>")]),
            s("<factor>"),
        ]),
    );

    grammar.add_production(
        "factor",
        alt(&[
            seq(&[opt(s("<sign>")), s("<factor>")]),
            seq(&[s("("), s("<expr>"), s(")")]),
            seq(&[s("<integer>"), opt(seq(&[s("."), s("<integer>")]))]),
        ]),
    );

    grammar.add_production("sign", alt(&[s("+"), s("-")]));

    grammar.add_production("integer", plus(s("<digit>")));

    grammar.add_production(
        "digit",
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
#[derive(PartialEq, Eq)]
struct Grammar(BTreeMap<Nonterminal, Vec<Expansion>>);
type Nonterminal = String;
type Expansion = Vec<String>; // Right-hand-side of a production rule.

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
struct Ebnf(BTreeMap<Nonterminal, Expr>);

/// EBNF syntax expression.
#[derive(Clone, PartialEq, Eq)]
enum Expr {
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
    fn to_bnf(&self) -> Grammar {
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
