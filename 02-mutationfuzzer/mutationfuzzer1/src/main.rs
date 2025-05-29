// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

// From https://www.fuzzingbook.org/html/MutationFuzzer.html

mod rng;

use rng::Rng;

fn main() {
    let seed_input = Bytes("A quick brown fox".as_bytes().to_vec());

    for _ in 0..10 {
        let inp = delete_random_character(seed_input.clone());
        println!("{}", inp);
    }
    println!();
    // A quick brown fx
    // A quickbrown fox
    // A quick brow fox
    // A qick brown fox
    // A quick brown fx
    // A qick brown fox
    // A quick brownfox
    // A quick brownfox
    // A quck brown fox
    // A quick brow fox

    for _ in 0..10 {
        let inp = insert_random_character(seed_input.clone());
        println!("{}", inp);
    }
    println!();
    // Ak quick brown fox
    // A quick brownW fox
    // A quick %brown fox
    // A quick br5own fox
    // A quiuck brown fox
    // CA quick brown fox
    // A Mquick brown fox
    // Aq quick brown fox
    // A quick brow(n fox
    // A qui/ck brown fox

    for _ in 0..10 {
        let inp = flip_random_bit(seed_input.clone());
        println!("{}", inp);
    }
    println!();
    // A quick bsown fox
    // A quick$brown fox
    // A uuick brown fox
    // A quick brown foz
    // Q quick brown fox
    // A quick brnwn fox
    // A quick brow~ fox
    // A quick brown dox
    // A quick brown fx
    // A$quick brown fox

    for _ in 0..10 {
        println!("{}", mutate(seed_input.clone()));
    }
    println!();
    // a quick brown fox
    // Aquick brown fox
    // A quick brownfox
    // >A quick brown fox
    // A quick brown fo
    // A quickbrown fox
    // A quic brown fox
    // A quik brown fox
    // A quick bown fox
    // A quick rown fox

    println!("Hello, world!");
}

/// Represents the structure that the fuzzer operates on. Here we use a
/// dedicated newtype instead of a type alias for being able to implement
/// integrated printing routines.
#[derive(Clone, Debug)]
struct Bytes(Vec<u8>);

// This assumes that the string is valid utf8.
impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

fn mutate(s: Bytes) -> Bytes {
    match Rng::new().int(3) {
        0 => delete_random_character(s),
        1 => insert_random_character(s),
        2 => flip_random_bit(s),
        _ => panic!("Can't happen"),
    }
}

fn delete_random_character(mut s: Bytes) -> Bytes {
    if s.0.is_empty() {
        s
    } else {
        let mut rng = Rng::new();
        let pos = rng.int(s.0.len() as u64) as usize;
        s.0.remove(pos);
        s
    }
}

fn insert_random_character(mut s: Bytes) -> Bytes {
    let mut rng = Rng::new();
    let pos = rng.int((s.0.len() + 1) as u64) as usize;
    let chr = rng.range(32, 127 + 1) as u8;
    s.0.insert(pos, chr);
    s
}

fn flip_random_bit(mut s: Bytes) -> Bytes {
    let mut rng = Rng::new();
    let pos = rng.int(s.0.len() as u64) as usize;
    let bit = 1 << rng.int(7);
    s.0[pos] ^= bit;
    s
}
