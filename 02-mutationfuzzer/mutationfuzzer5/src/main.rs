// SPDX-FileCopyrightText: 2019 Rough structure: adapted from gamozo https://github.com/gamozolabs/guifuzz
// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/MutationFuzzer.html Guiding by Coverage
// But refactored adapted from https://github.com/gamozolabs/guifuzz.

mod fuzzer;
mod rng;

use rng::Rng;

// [+] Running with random seed 15755431227689406735
//
// [+] Final population
// http://www.google.com/search?q=fuzzing
// ht|p:/www.gog1le.com/earch?=fuzzing
// htp://www.google.com/seach?q=Sfuzzng
// (T|p:/www.gg1le.com/earch?fuzzig
// htx:/$/www.google.com/each`?q=Sfuzzng
// ht|p:/wwv.go/g1le.cm/earc`=fuZ9zine
// t|p:/wwv~.go/gJ.cm/earc`"=fuZ9ziYne
// (T|p/:www.gg1le.Hol/eqrcH?uzzig
// (T|p:/www.gg1le.com/earchU?fuzzg
// (T|p:/www.gg1le.com/earci?$fuzzig
// h&t|p*/ww.cSg1lecom/earkh?=fuzzing
// Ahtxp:/www.gg&V1ld.com/arch?=fuzzing
// htp://ww.Ygoogle.coM/seach?q=Sfuzuzng
// T,|p:/wwwh'le.com/earci?$fuzzig
// (T|p:/wwwgg1le.com/eaarcz?fuzzig
// A(htp:/.ww.Yg/~ogld&coM/+seach?qSfuzuz^ng
// (T|p:/www.g1lie.kol/eArchU?uzzg
// htx:/$/wwwXgooglec\om/each`?=SfuzzngE
// (T|p:/wwwgg1le.com/gaarcz?fuzzi
// ht|p:/wZwv.go/g1le.cm/eac`=fuZ9zine
// htx:/_$/wwXgooglec\o/each`=SuzzngE
// htp:/ww.google.cOm/seach?q=Suzzng
// htx:/_$f/wwX|ooglec\o/easch`_=Suzz~gE
// htp:/ww.googl.cOm/seqch?q:=Suzng
// htr9?//ww.goog9le.cM/sesach?Uq=Sfuzung
// (T`|p/:www.gg1le.Hl/eq~rc?uzzig
// htt://wwggooGle.com/usmarch?q=fuzing
// htp://www.google.com/sech?q=Sfuzzng
// ht|p:/www.goUg1l.com/earh?=fuzzing
// (T|p:www.g1Kle.co}/eapchU>fuzzg
// (T|pM/www.gg1sle.GcoXBm/arcWhUfuzzg
// (Ty|pM/www.VggH1wleGcoXBm/arcWhUfuzzg
// htp:/ww$g.groogne.com/searIshq=fuzzin
// _t<|p:/wwv~.g/J.cm/earc`=fuZ9ziYe
// xvp:/Xwww>google.com/sech?q=^fuzz6ng
// Ahtxp:/www.gg&V1ld.com'/ar,ch?=fuzzing
// T|p:/www.gDg1l.co}/earci?$fwzzig
// (T|pM/www.go1sle.GcoXBm/rcWhUfuzzg
// htr9?//ww.googp9le.cM/sesach?Uq=SfYuzung
// AhtxP:/www.g&V1ll.m'F/ar,ch?4uzzing
// htx:/$/www.gJoogl.comK/ecui`?q=SfqjZno
//
// [+] Final coverage: 43

fn main() {
    let mut rng = Rng::new();
    println!("[+] Running with random seed {}", rng.initialseed);
    println!();

    let input = fuzzer::Input::from_str("http://www.google.com/search?q=fuzzing");

    let mut stats = fuzzer::Statistics::new(vec![input]);
    fuzzer::run(&mut rng, &mut stats, 40);

    println!("[+] Final population");
    for el in stats.population_list {
        println!("{}", el);
    }
    println!();

    println!("[+] Final coverage: {}", stats.coverage_all.len());

    // Output gnuplot file; generate plot: ./plot.plt
    fuzzer::plot_cumulative_coverage(&stats.cumulative_coverage);
}
