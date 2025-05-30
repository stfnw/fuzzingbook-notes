// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/MutationFuzzer.html Multiple Mutations

mod fuzzer;
mod rng;

use fuzzer::{mutate, Bytes, Fuzzer, MutationFuzzer};
use rng::Rng;

fn main() {
    let mut rng = Rng::new();

    let seed_input = Bytes::from_str("http://www.google.com/search?q=fuzzing");

    let mutations = 50;
    let mut input = seed_input.clone();
    for i in 0..mutations {
        if i % 5 == 0 {
            println!("{:3} {}", i, input);
        }
        input = mutate(&mut rng, input);
    }
    println!();
    //  0 http://www.google.com/search?q=fuzzing
    //  5 htt:/?www.google.co/search>q=fuxzing
    // 10 htt:/wvw.goog*le.co/seach>q<fuxzing
    // 15 htt:/wvw.go]og*le.co'seachf>q<fu[xzig
    // 20 htt:/wvw.gco]og*le.co'seachfq<u[xzhgF
    // 25 htt/wvw.gco]og*le.co'sejauchfYq<u[xzhgF
    // 30 htt/owvw.gog*le.cosejauchfYq<u[xzhgF
    // 35 htt/owvw.g/*la.cksejauchfY<u[xzhgF
    // 40 htt/ovsw.g/*la.bksejauchfI<u[xzgF
    // 45 htt/ovwg/*la.bksgyejuchfI<u[xzgF

    let mutation_fuzzer = MutationFuzzer::new(vec![seed_input]);
    let mutations = 10;
    for _ in 0..mutations {
        println!("{}", mutation_fuzzer.fuzz(&mut rng));
    }
    // http://www.google.com/search?q=fuzzing
    // http:/www.googlecom/search?fuzing
    // http:./www.googl#enco3e/se`arch?q=%f}zzing
    // ht4://www.googhe.com/search?q=fuzzing
    // htt://www.googLe.com/search?q=fuzzing
    // htt://www&'oogleX.com'3earch?q=f}z&zing
    // http://w.google.com/search?qf{zynYg%
    // htup//www.google.com/search?q=fuzzbg
    // http~://fww.googl.com/search?q=fuzziDng
    // http:/Qwww.goo'{le.com/srhq=fuzz4kng
}
