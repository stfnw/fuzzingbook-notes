// SPDX-FileCopyrightText: 2018 xoshiro256** and splitmix64 implementation: David Blackman and Sebastiano Vigna
// SPDX-License-Identifier: LicenseRef-PublicDomainRng
//
// SPDX-FileCopyrightText: 2025 Rest of implementation and scaffolding: stfnw
// SPDX-License-Identifier: MIT

/// Pseudo-random generator xoshiro256** seeded with splitmix64.
/// From https://prng.di.unimi.it/xoshiro256starstar.c
/// and https://prng.di.unimi.it/splitmix64.c
/// by David Blackman and Sebastiano Vigna.
#[allow(dead_code)]
pub struct Rng {
    pub initialseed: u64,
    state: [u64; 4],
}

#[allow(dead_code)]
impl Rng {
    pub fn new() -> Self {
        Self::seeded(unsafe { core::arch::x86_64::_rdtsc() })
    }

    /// Create a new PRNG from a seed value.
    pub fn seeded(mut seed: u64) -> Self {
        let mut state = [0, 0, 0, 0];
        (state[0], seed) = Self::splitmix64(seed);
        (state[1], seed) = Self::splitmix64(seed);
        (state[2], seed) = Self::splitmix64(seed);
        (state[3], _) = Self::splitmix64(seed);
        Self {
            initialseed: seed,
            state,
        }
    }

    fn splitmix64(seed: u64) -> (u64, u64) {
        let seed_ = seed.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = seed_;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        (z ^ (z >> 31), seed_)
    }

    /// Create new random number and advance the internal state.
    pub fn next(&mut self) -> u64 {
        let result = (self.state[1].wrapping_mul(5))
            .rotate_left(7)
            .wrapping_mul(9);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;

        self.state[3] = self.state[3].rotate_left(45);

        result
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
        assert!(min < max);
        let range = max - min;
        min + (self.next() % range)
    }

    /// Create random number in range [0,max).
    pub fn int(&mut self, max: u64) -> u64 {
        self.range(0, max)
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
}
