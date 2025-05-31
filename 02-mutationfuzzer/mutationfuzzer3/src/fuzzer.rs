// SPDX-FileCopyrightText: 2025 Original python code: fuzzingbook, https://www.fuzzingbook.org, Saarland University, CISPA, authors, and contributors
// SPDX-FileCopyrightText: 2025 Implementation/refactoring/adaptation: stfnw
//
// SPDX-License-Identifier: MIT

/// Represents the structure that the fuzzer operates on. Here we use a
/// dedicated newtype instead of a type alias for being able to implement
/// integrated printing routines.
#[derive(Clone, Debug)]
pub struct Input(pub Vec<u8>);

impl Input {
    /// Convert a `&str` to `Input`. I choose to do it this way and not use
    /// `FromStr` trait since that returns a Result which has to be unwrapped.
    /// This is unnecessary since in this case the conversion can never fail
    /// (Vec<u8> is a super-set of &str).
    pub fn from_str(s: &str) -> Self {
        Self(s.as_bytes().to_vec())
    }
}

// This assumes that the string is valid utf8.
impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}
