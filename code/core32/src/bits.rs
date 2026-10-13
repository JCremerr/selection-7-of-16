//! Small bit utilities, a fast hasher and a tiny RNG (std only).
//! core32 port: element sets are u32 (n <= 19, see poset::MAXN).

use std::hash::{BuildHasherDefault, Hasher};

/// Iterator over the set bits of a u32 (ascending).
#[derive(Clone, Copy)]
pub struct Bits(pub u32);

impl Iterator for Bits {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            let t = self.0.trailing_zeros() as usize;
            self.0 &= self.0 - 1;
            Some(t)
        }
    }
}

#[inline]
pub fn bits(m: u32) -> Bits {
    Bits(m)
}

/// Mask with the low `n` bits set (n <= 31).
#[inline]
pub fn low_mask(n: usize) -> u32 {
    debug_assert!(n < 32);
    (1u32 << n) - 1
}

/// Parallel bit extract: gather the bits of `x` selected by `mask` into the low bits.
#[inline]
pub fn pext32(x: u32, mask: u32) -> u32 {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
    unsafe {
        core::arch::x86_64::_pext_u32(x, mask)
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
    {
        let mut r = 0u32;
        let mut k = 0;
        for b in bits(mask) {
            if x >> b & 1 != 0 {
                r |= 1 << k;
            }
            k += 1;
        }
        r
    }
}

/// Multiply-xorshift hasher; good enough for canonical keys.
#[derive(Default, Clone, Copy)]
pub struct KeyHasher(u64);

impl Hasher for KeyHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.write_u64(b as u64);
        }
    }
    #[inline]
    fn write_u64(&mut self, x: u64) {
        self.0 = mix64(self.0 ^ x);
    }
    #[inline]
    fn write_u128(&mut self, x: u128) {
        self.0 = mix64(mix64(self.0 ^ x as u64) ^ (x >> 64) as u64);
    }
}

pub type BuildKeyHasher = BuildHasherDefault<KeyHasher>;

#[inline]
pub fn mix64(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Hash of a canonical key (equal to code/core's `hash_key` for n <= 16).
#[inline]
pub fn hash_key(k: crate::canon::Key) -> u64 {
    k.hash()
}

/// Hash of a canonical key, given its 128-bit fold `f` and the extra bits `e`
/// (see `canon::Key::fold`). For keys of problems with n <= 16, e == 0 and f is
/// the old (code/core) u128 key, so this equals code/core's `hash_key` and the
/// df-pn search visits the same TT buckets / cache slots as the old core.
#[inline]
pub fn hash_fold(f: u128, e: u64) -> u64 {
    let h = mix64(mix64(f as u64) ^ (f >> 64) as u64);
    if e == 0 {
        h
    } else {
        mix64(h ^ e)
    }
}

/// SplitMix64 RNG (deterministic, for tests and tie-breaking).
#[derive(Clone)]
pub struct Rng(pub u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in 0..n (n > 0).
    #[inline]
    pub fn below(&mut self, n: usize) -> usize {
        ((self.next_u64() as u128 * n as u128) >> 64) as usize
    }
    /// Bernoulli(p).
    #[inline]
    pub fn chance(&mut self, p: f64) -> bool {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64) < p
    }
    /// Random permutation of 0..n as a [u8; MAXN] (entries >= n are identity).
    pub fn perm(&mut self, n: usize) -> [u8; crate::poset::MAXN] {
        let mut p = [0u8; crate::poset::MAXN];
        for (k, x) in p.iter_mut().enumerate() {
            *x = k as u8;
        }
        for k in (1..n).rev() {
            let j = self.below(k + 1);
            p.swap(k, j);
        }
        p
    }
}
