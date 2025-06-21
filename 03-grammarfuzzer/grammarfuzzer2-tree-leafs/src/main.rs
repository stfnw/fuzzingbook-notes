// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use std::collections::VecDeque;

fn main() {
    let tree = nt(
        "start",
        &[nt("expr", &[nt("expr", &[]), t("+"), nt("term", &[])])],
    );

    println!("[+] Derivation tree:");
    println!("{:#?}", tree);
    println!();

    // [+] Derivation tree:
    // NT(
    //     "start",
    //     [
    //         NT(
    //             "expr",
    //             [
    //                 NT(
    //                     "expr",
    //                     [],
    //                 ),
    //                 T(
    //                     "+",
    //                 ),
    //                 NT(
    //                     "term",
    //                     [],
    //                 ),
    //             ],
    //         ),
    //     ],
    // )

    println!("[+] Graphviz / dot format:");
    println!("{}", tree.to_dot());
    println!();

    // [+] Graphviz / dot format:
    // digraph Derivation {
    //
    //     node [shape=plain];
    //
    //     n1 [label="\<start\>"];
    //     n2 [label="\<expr\>"];
    //     n1 -> n2;
    //
    //     n3 [label="\<expr\>"];
    //     n2 -> n3;
    //
    //     n4 [label="+"];
    //     n2 -> n4;
    //
    //     n5 [label="\<term\>"];
    //     n2 -> n5;
    //
    // }

    println!("[+] All leaf symbols:");
    println!("{}", tree.all_leafs());
    println!();

    // [+] All leaf symbols:
    //  <expr> + <term>
}

/// Derivation tree in a given grammar.
#[derive(Clone, Debug)]
enum Derivation {
    /// Nonterminal symbol (inner node in the tree) consisting of a symbol name
    /// and a list of child nodes / children.
    NT(String, Vec<Derivation>),
    /// Terminal symbol (leaf of the tree) consisting only of a symbol name
    /// (= final text for this tree part); it has no children.
    T(String),
}

// Shorthand functions for easier construction of derivation trees.
#[rustfmt::skip]
fn nt(name: &str, children: &[Derivation]) -> Derivation { Derivation::NT(name.to_string(), children.to_vec()) }
#[rustfmt::skip]
fn t(name: &str)                           -> Derivation { Derivation::T(name.to_string()) }

impl Derivation {
    /// Returns a dot / graphviz definition of the derivation tree / graph.
    /// (Does iterative pre-order traversal of the tree).
    /// It can be rendered e.g. as follows: dot -Tpdf tree.dot -o tree.pdf
    fn to_dot(&self) -> String {
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
    fn all_leafs(&self) -> String {
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
}
