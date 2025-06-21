// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 This implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

use crate::grammarfuzzer::{alt, s, seq, Ebnf, Expr, Grammar};

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

pub fn cgi_grammar() -> Grammar {
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

    let other: Vec<_> = ((0..26).map(|x| char::from(x + b'a').to_string()))
        .chain((0..10).map(|x| x.to_string()))
        .chain(["-", "_"].into_iter().map(|x| x.to_string()))
        .collect();
    grammar.add_productions(
        "<other>",
        &other.iter().map(|x| x.as_str()).collect::<Vec<_>>(),
    );

    grammar
}

pub fn title_grammar() -> Grammar {
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

pub fn json_grammar() -> Ebnf {
    let mut grammar = Ebnf::new();

    grammar.add_production("start", s("<json>"));

    grammar.add_production("json", s("<element>"));

    grammar.add_production(
        "value",
        alt(&[
            s("<object>"),
            s("<array>"),
            s("<string>"),
            s("<number>"),
            s("true"),
            s("false"),
            s("null"),
        ]),
    );

    grammar.add_production(
        "object",
        alt(&[
            seq(&[s("{"), s("<ws>"), s("}")]),
            seq(&[s("{"), s("<members>"), s("}")]),
        ]),
    );

    grammar.add_production(
        "members",
        alt(&[s("<member>"), seq(&[s("<member>"), s(","), s("<members>")])]),
    );
    grammar.add_production(
        "member",
        seq(&[s("<ws>"), s("<string>"), s("<ws>"), s(":"), s("<element>")]),
    );

    grammar.add_production(
        "array",
        alt(&[
            seq(&[s("["), s("<ws>"), s("]")]),
            seq(&[s("["), s("<elements>"), s("]")]),
        ]),
    );

    grammar.add_production(
        "elements",
        alt(&[
            s("<element>"),
            seq(&[s("<element>"), s(","), s("<elements>")]),
        ]),
    );
    grammar.add_production("element", seq(&[s("<ws>"), s("<value>"), s("<ws>")]));

    grammar.add_production("string", seq(&[s("\""), s("<characters>"), s("\"")]));

    grammar.add_production(
        "characters",
        alt(&[s(""), seq(&[s("<character>"), s("<characters>")])]),
    );

    // Here we only add printable ASCII characters.
    let valid_chars: Vec<_> = (0x20..0x7e)
        .filter(|x| *x != b'"' && *x != b'\\')
        .map(|x| char::from_u32(x.into()).unwrap().to_string())
        .map(Expr::T)
        .collect();
    grammar.add_production(
        "character",
        alt(&[Expr::Alt(valid_chars), seq(&[s("\\"), s("<escape>")])]),
    );

    grammar.add_production(
        "escape",
        alt(&[
            s("\""),
            s("\\"),
            s("/"),
            s("b"),
            s("f"),
            s("n"),
            s("r"),
            s("t"),
            seq(&[s("u"), s("<hex>"), s("<hex>"), s("<hex>"), s("<hex>")]),
        ]),
    );

    grammar.add_production(
        "hex",
        alt(&[
            s("<digit>"),
            s("A"),
            s("B"),
            s("C"),
            s("D"),
            s("E"),
            s("F"),
            s("a"),
            s("b"),
            s("c"),
            s("d"),
            s("e"),
            s("f"),
        ]),
    );

    grammar.add_production(
        "number",
        seq(&[s("<integer>"), s("<fraction>"), s("<exponent>")]),
    );

    grammar.add_production(
        "integer",
        alt(&[
            s("<digit>"),
            seq(&[s("<onenine>"), s("<digits>")]),
            seq(&[s("-"), s("<digit>")]),
            seq(&[s("-"), s("<onenine>"), s("<digits>")]),
        ]),
    );

    grammar.add_production(
        "digits",
        alt(&[s("<digit>"), seq(&[s("<digit>"), s("<digits>")])]),
    );
    grammar.add_production("digit", alt(&[s("0"), s("<onenine>")]));
    grammar.add_production(
        "onenine",
        alt(&[
            s("1"),
            s("2"),
            s("3"),
            s("4"),
            s("5"),
            s("6"),
            s("7"),
            s("8"),
            s("9"),
        ]),
    );

    grammar.add_production("fraction", alt(&[s(""), seq(&[s("."), s("<digits>")])]));
    grammar.add_production(
        "exponent",
        alt(&[
            s(""),
            seq(&[s("E"), s("<sign>"), s("<digits>")]),
            seq(&[s("e"), s("<sign>"), s("<digits>")]),
        ]),
    );
    grammar.add_production("sign", alt(&[s(""), s("+"), s("-")]));

    grammar.add_production("ws", alt(&[s(""), s(" "), s("\r"), s("\n"), s("\t")]));

    grammar
}
