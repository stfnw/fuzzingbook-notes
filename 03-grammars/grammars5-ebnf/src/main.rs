// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// https://www.fuzzingbook.org/html/Grammars.html

use std::collections::BTreeMap;

fn main() {
    let ebnf = expr_grammar_ebnf();

    println!("[+] EBNF context-free grammar");
    println!("{}", ebnf);
    // [+] EBNF context-free grammar
    // <start> -> <expr>
    // digit   -> "0"  |  "1"  |  "2"  |  "3"  |  "4"  |  "5"  |  "6"  |  "7"  |  "8"  |  "9"
    // expr    -> <term> "+" <expr>  |  <term> "-" <expr>  |  <term>
    // factor  -> (<sign>)? <factor>  |  "(" <expr> ")"  |  <integer> ("." <integer>)?
    // integer -> (<digit>)+
    // sign    -> "+"  |  "-"
    // term    -> <factor> "*" <term>  |  <factor> "/" <term>  |  <factor>

    let bnf = ebnf_to_bnf(&ebnf);

    println!("[+] BNF context-free grammar (converted)");
    println!("{}", bnf);
    // [+] BNF context-free grammar (converted)
    // <start>   -> <expr>
    // <symbol0> -> <sign> | ""
    // <symbol1> -> "." <integer> | ""
    // <symbol2> -> <digit> | <digit> <symbol2>
    // digit     -> "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
    // expr      -> <term> "+" <expr> | <term> "-" <expr> | <term>
    // factor    -> <symbol0> <factor> | "(" <expr> ")" | <integer> <symbol1>
    // integer   -> <symbol2>
    // sign      -> "+" | "-"
    // term      -> <factor> "*" <term> | <factor> "/" <term> | <factor>

    let ebnf = cgi_grammar();
    println!("{}", ebnf);
    // <start> -> (<letter>)+
    // letter  -> <plus>  |  <percent>  |  <other>
    // other   -> "0"  |  "1"  |  "2"  |  "3"  |  "4"  |  "5"  |  "a"  |  "b"  |  "c"  |  "d"  |  "e"  |  "-"  |  "_"
    // percent -> "%" <hexdigit> <hexdigit>
    // plus    -> "+"
    println!("{}", ebnf_to_bnf(&ebnf));
    // <start>   -> <symbol0>
    // <symbol0> -> <letter> | <letter> <symbol0>
    // letter    -> <plus> | <percent> | <other>
    // other     -> "0" | "1" | "2" | "3" | "4" | "5" | "a" | "b" | "c" | "d" | "e" | "-" | "_"
    // percent   -> "%" <hexdigit> <hexdigit>
    // plus      -> "+"

    let ebnf = title_grammar();
    println!("{}", ebnf);
    // <fuzzing-prefix>    -> ""  |  "The Art of "  |  "The Joy of "
    // <reader-property>   -> "Fun"  |  "Profit"
    // <software-property> -> "Robustness"  |  "Reliability"  |  "Security"
    // <start>             -> <title>
    // <subtopic-main>     -> "Breaking Software"  |  "Generating Software Tests"  |  "Principles, Techniques and Tools"
    // <subtopic-prefix>   -> ""  |  "Tools and Techniques for "
    // <subtopic-suffix>   -> " for " <reader-property> " and " <reader-property>  |  " for " <software-property> " and " <software-property>
    // <subtopic>          -> <subtopic-main>  |  <subtopic-prefix> <subtopic-main>  |  <subtopic-main> <subtopic-suffix>
    // <title>             -> <topic> ": " <subtopic>
    // <topic>             -> "Generating Software Tests"  |  <fuzzing-prefix> "Fuzzing"  |  "The Fuzzing Book"
    // println!("{}", ebnf_to_bnf(&ebnf)); // is the same
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

fn cgi_grammar() -> Ebnf {
    let mut grammar = Ebnf::new();

    grammar.add_production("<start>", plus(s("<letter>")));
    grammar.add_production("letter", alt(&[s("<plus>"), s("<percent>"), s("<other>")]));
    grammar.add_production("plus", s("+"));
    grammar.add_production("percent", seq(&[s("%"), s("<hexdigit>"), s("<hexdigit>")]));
    grammar.add_production(
        "other",
        alt(&[
            s("0"),
            s("1"),
            s("2"),
            s("3"),
            s("4"),
            s("5"),
            s("a"),
            s("b"),
            s("c"),
            s("d"),
            s("e"),
            s("-"),
            s("_"),
        ]),
    );

    grammar
}

fn title_grammar() -> Ebnf {
    let mut grammar = Ebnf::new();

    grammar.add_production("<start>", s("<title>"));

    grammar.add_production("<title>", seq(&[s("<topic>"), s(": "), s("<subtopic>")]));

    grammar.add_production(
        "<topic>",
        alt(&[
            s("Generating Software Tests"),
            seq(&[s("<fuzzing-prefix>"), s("Fuzzing")]),
            s("The Fuzzing Book"),
        ]),
    );

    grammar.add_production(
        "<fuzzing-prefix>",
        alt(&[s(""), s("The Art of "), s("The Joy of ")]),
    );

    grammar.add_production(
        "<subtopic>",
        alt(&[
            s("<subtopic-main>"),
            seq(&[s("<subtopic-prefix>"), s("<subtopic-main>")]),
            seq(&[s("<subtopic-main>"), s("<subtopic-suffix>")]),
        ]),
    );

    grammar.add_production(
        "<subtopic-main>",
        alt(&[
            s("Breaking Software"),
            s("Generating Software Tests"),
            s("Principles, Techniques and Tools"),
        ]),
    );

    grammar.add_production(
        "<subtopic-prefix>",
        alt(&[s(""), s("Tools and Techniques for ")]),
    );

    grammar.add_production(
        "<subtopic-suffix>",
        alt(&[
            seq(&[
                s(" for "),
                s("<reader-property>"),
                s(" and "),
                s("<reader-property>"),
            ]),
            seq(&[
                s(" for "),
                s("<software-property>"),
                s(" and "),
                s("<software-property>"),
            ]),
        ]),
    );

    grammar.add_production("<reader-property>", alt(&[s("Fun"), s("Profit")]));

    grammar.add_production(
        "<software-property>",
        alt(&[s("Robustness"), s("Reliability"), s("Security")]),
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
}

/// Determines if a given symbol name represents a nonterminal.
/// This is only by convention and not actually enforced anywhere.
fn is_nonterminal(s: &str) -> bool {
    s.starts_with("<") && s.ends_with(">")
}

/// Context-free-grammar with support for EBNF constructs.
#[derive(Debug, Clone)]
struct Ebnf(BTreeMap<Nonterminal, Expr>);

/// EBNF syntax expression.
#[derive(Debug, Clone)]
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
}

/// Convert a grammar from EBNF to BNF by replacing regular language constructs
/// / expressions with direct production rules.
fn ebnf_to_bnf(ebnf: &Ebnf) -> Grammar {
    let mut bnf = Grammar::new();

    // Iterate over each production rule and expand out and flatten all extended
    // syntax constructs.
    for (nonterminal, expression) in ebnf.0.iter() {
        let mut symbolcounter = 0; // Needed for generating fresh new symbol names.
        let expansions = ebnf_to_bnf_expr(&mut bnf, expression, &mut symbolcounter);
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
fn ebnf_to_bnf_expr(bnf: &mut Grammar, expression: &Expr, i: &mut usize) -> Vec<Expansion> {
    match expression {
        // Alternatives are represented as top-level Vecs.
        Expr::Alt(exprs) => {
            let mut res = Vec::new();
            for expr in exprs {
                res.extend(ebnf_to_bnf_expr(bnf, expr, i));
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
                let expr_expansions = ebnf_to_bnf_expr(bnf, expr, i);
                if expr_expansions.len() == 1 && expr_expansions[0].len() == 1 {
                    // We can shortcut and don't need to add a useless new
                    // intermediate nonterminal symbol that would only expand
                    // to *one single* other symbol anyway.
                    res.push(expr_expansions[0][0].clone());
                } else {
                    let s = new_nonterminal(bnf, i);
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
            let s = new_nonterminal(bnf, i);
            let expr_expansions = ebnf_to_bnf_expr(bnf, expr, i);
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
            let s = new_nonterminal(bnf, i);
            let expr_expansions = ebnf_to_bnf_expr(bnf, expr, i);
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
            let s = new_nonterminal(bnf, i);
            let expr_expansions = ebnf_to_bnf_expr(bnf, expr, i);
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
