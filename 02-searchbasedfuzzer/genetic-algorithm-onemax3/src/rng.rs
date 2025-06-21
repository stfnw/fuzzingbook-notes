// SPDX-FileCopyrightText: xorshift64 implementation from G. Marsaglia, “Xorshift RNGs,” J. Stat. Soft., vol. 8, no. 14, pp. 1–6, Jul. 2003, doi: 10.18637/jss.v008.i14.
//
// SPDX-FileCopyrightText: 2025 Choices and bisect implementation translated from Python; original code: Python Software Foundation
// SPDX-License-Identifier: PSF-2.0
//
// SPDX-FileCopyrightText: 2025 Rest of implementation and scaffolding: stfnw
// SPDX-License-Identifier: MIT

pub struct Rng {
    pub state: u64,
}

#[allow(dead_code)]
impl Rng {
    /// Create a new PRNG with a seed based on current time.
    pub fn new() -> Self {
        Self::seeded(unsafe { core::arch::x86_64::_rdtsc() })
    }

    /// Create a new PRNG from a seed value.
    pub fn seeded(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Create new random number and advance the internal state.
    pub fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    /// Create random u64.
    pub fn u64(&mut self) -> u64 {
        self.next()
    }

    /// Create random float in [0,1.0)
    pub fn f64(&mut self) -> f64 {
        (self.u64() as f64) / (u64::MAX as f64)
    }

    /// Create random number in given range [min,max).
    /// Uses naive way that leads to slightly non-uniform distribution.
    pub fn range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min < max, "{} >= {}", min, max);
        let range = max - min;
        min + (self.next() % range)
    }

    /// Create random number in range [0,max).
    pub fn int(&mut self, max: u64) -> u64 {
        self.range(0, max)
    }

    /// Create a random boolean value.
    pub fn bool(&mut self) -> bool {
        match self.int(2) {
            0 => false,
            1 => true,
            _ => panic!("Can't happen"),
        }
    }

    /// Create a random sequence of bytes.
    pub fn bytes(&mut self, len: u64) -> Vec<u8> {
        let mut res = Vec::new();
        for _ in 0..len {
            res.push(self.int(0x100) as u8);
        }
        res
    }

    /// Create a random sequence of bytes where each byte lies in
    /// [byte_min, byte_max).
    pub fn bytes_range(&mut self, len: u64, min: u64, max: u64) -> Vec<u8> {
        let mut res = Vec::new();
        for _ in 0..len {
            res.push(self.range(min, max) as u8);
        }
        res
    }

    /// Create a random ascii string.
    pub fn ascii(&mut self, len: u64) -> String {
        let res = self.bytes_range(len, 0, 0x7f + 1);
        String::from_utf8(res).unwrap()
    }

    /// Create a random printable ascii string.
    pub fn ascii_printable(&mut self, len: u64) -> String {
        let res = self.bytes_range(len, 0x20, 0x7e + 1);
        String::from_utf8(res).unwrap()
    }

    /// Randomly choose an element of a slice.
    pub fn choice<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        let pos = self.int(v.len() as u64) as usize;
        &v[pos]
    }

    /// Randomly choose one element from a slice given weights/propabilities.
    /// Translated from https://github.com/python/cpython/blob/9634085af3670b1eb654e3c7820aca66f358f39f/Lib/random.py#L460
    /// and https://github.com/python/cpython/blob/9634085af3670b1eb654e3c7820aca66f358f39f/Lib/bisect.py#L21
    pub fn choice_w<'a, T>(&mut self, v: &'a [T], weights: &[f64]) -> &'a T {
        assert!(v.len() == weights.len(), "{} != {}", v.len(), weights.len());
        let mut cumuluative_weights = Vec::new();
        let mut tmp = 0.0;
        for w in weights {
            assert!(*w >= 0.0, "Weight must be non-negative {}", w);
            tmp += w;
            cumuluative_weights.push(tmp);
        }
        self.choice_cw(v, &cumuluative_weights)
    }

    pub fn choice_cw<'a, T>(&mut self, v: &'a [T], cumulative_weights: &[f64]) -> &'a T {
        assert!(
            v.len() == cumulative_weights.len(),
            "{} != {}",
            v.len(),
            cumulative_weights.len()
        );

        let total = *cumulative_weights.last().unwrap();
        assert!(total > 0.0, "Total weight must be non-zero: {}", total);

        let pos = bisect(
            cumulative_weights,
            self.f64() * total,
            0,
            cumulative_weights.len() - 1,
        );

        &v[pos]
    }
}

fn bisect(v: &[f64], x: f64, mut lo: usize, mut hi: usize) -> usize {
    while lo < hi {
        let mid = (lo + hi) / 2;
        if x < v[mid] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}
