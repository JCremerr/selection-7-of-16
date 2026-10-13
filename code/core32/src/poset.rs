//! Posets on at most 19 elements as u32 bitsets, and selection problems (P, i).
//! (core32 port of code/core: u16 -> u32 sets, MAXN 16 -> 19.)
//!
//! Conventions: elements are 0..n-1. `above[a]` = set of elements strictly greater
//! than a, `below[a]` = set strictly smaller; both always transitively closed.
//! `i` is 1-based: (P, i) asks for the i-th smallest element.

use crate::bits::{bits, low_mask, pext32, Rng};

/// Maximum number of elements. 19 is the largest n whose canonical key
/// (n(n-1)/2 = 171 relation bits + n and i fields) fits the 192-bit `canon::Key`.
pub const MAXN: usize = 19;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Poset {
    pub n: u8,
    pub above: [u32; MAXN],
    pub below: [u32; MAXN],
}

impl Poset {
    /// The antichain on n elements (no known relations).
    pub fn antichain(n: usize) -> Self {
        assert!(n >= 1 && n <= MAXN);
        Poset { n: n as u8, above: [0; MAXN], below: [0; MAXN] }
    }

    /// The chain 0 < 1 < ... < n-1.
    pub fn chain(n: usize) -> Self {
        let mut p = Self::antichain(n);
        for a in 0..n {
            p.above[a] = low_mask(n) & !low_mask(a + 1);
            p.below[a] = low_mask(a);
        }
        p
    }

    /// Build from `above` masks; returns None unless they describe a strict partial
    /// order (irreflexive, transitively closed; antisymmetry follows).
    pub fn from_above(n: usize, above: &[u32]) -> Option<Self> {
        if n == 0 || n > MAXN || above.len() < n {
            return None;
        }
        let mut p = Self::antichain(n);
        for a in 0..n {
            if above[a] & !low_mask(n) != 0 || above[a] >> a & 1 != 0 {
                return None;
            }
            p.above[a] = above[a];
            for b in bits(above[a]) {
                p.below[b] |= 1 << a;
            }
        }
        if p.is_valid() {
            Some(p)
        } else {
            None
        }
    }

    /// Build from a list of relations a<b (transitive closure is taken).
    /// Panics on a cycle.
    pub fn from_relations(n: usize, rels: &[(usize, usize)]) -> Self {
        let mut p = Self::antichain(n);
        for &(a, b) in rels {
            if p.less(a, b) {
                continue;
            }
            assert!(a != b && !p.less(b, a), "relations contain a cycle");
            p.add_less(a, b);
        }
        p
    }

    #[inline]
    pub fn n(&self) -> usize {
        self.n as usize
    }

    #[inline]
    pub fn all(&self) -> u32 {
        low_mask(self.n as usize)
    }

    /// a < b in P?
    #[inline]
    pub fn less(&self, a: usize, b: usize) -> bool {
        self.above[a] >> b & 1 != 0
    }

    #[inline]
    pub fn comparable(&self, a: usize, b: usize) -> bool {
        a == b || (self.above[a] | self.below[a]) >> b & 1 != 0
    }

    /// Add a < b (a, b incomparable) and restore transitive closure. O(n).
    #[inline]
    pub fn add_less(&mut self, a: usize, b: usize) {
        debug_assert!(!self.comparable(a, b));
        let lo = self.below[a] | (1 << a);
        let hi = self.above[b] | (1 << b);
        for x in bits(lo) {
            self.above[x] |= hi;
        }
        for y in bits(hi) {
            self.below[y] |= lo;
        }
    }

    #[inline]
    pub fn with_less(&self, a: usize, b: usize) -> Self {
        let mut q = *self;
        q.add_less(a, b);
        q
    }

    /// Reversed order.
    pub fn dual(&self) -> Self {
        Poset { n: self.n, above: self.below, below: self.above }
    }

    /// Relabel: old element x becomes perm[x]. `perm` must be a permutation of 0..n.
    pub fn relabel(&self, perm: &[u8]) -> Self {
        let n = self.n();
        let mut q = Self::antichain(n);
        for x in 0..n {
            let px = perm[x] as usize;
            let mut a = 0u32;
            for y in bits(self.above[x]) {
                a |= 1 << perm[y];
            }
            let mut b = 0u32;
            for y in bits(self.below[x]) {
                b |= 1 << perm[y];
            }
            q.above[px] = a;
            q.below[px] = b;
        }
        q
    }

    /// Induced subposet on `mask`, relabeled 0.. in increasing order of old labels.
    pub fn induced(&self, mask: u32) -> Self {
        let mut q = Self::antichain(mask.count_ones().max(1) as usize);
        q.n = mask.count_ones() as u8;
        let mut k = 0;
        for x in bits(mask) {
            q.above[k] = pext32(self.above[x], mask);
            q.below[k] = pext32(self.below[x], mask);
            k += 1;
        }
        q
    }

    /// Check: masks inside 0..n, irreflexive, below = transpose(above), transitive.
    pub fn is_valid(&self) -> bool {
        let n = self.n();
        if n == 0 || n > MAXN {
            return false;
        }
        let all = self.all();
        for a in 0..n {
            if self.above[a] & !all != 0 || self.below[a] & !all != 0 {
                return false;
            }
            if (self.above[a] | self.below[a]) >> a & 1 != 0 {
                return false;
            }
            for b in 0..n {
                if self.less(a, b) != (self.below[b] >> a & 1 != 0) {
                    return false;
                }
            }
            for b in bits(self.above[a]) {
                if self.above[b] & !self.above[a] != 0 {
                    return false;
                }
            }
        }
        for a in n..MAXN {
            if self.above[a] != 0 || self.below[a] != 0 {
                return false;
            }
        }
        true
    }

    pub fn num_relations(&self) -> usize {
        (0..self.n()).map(|a| self.above[a].count_ones() as usize).sum()
    }

    /// Number of connected components of the comparability graph.
    pub fn components(&self) -> usize {
        let mut left = self.all();
        let mut c = 0;
        while left != 0 {
            let mut comp = left & left.wrapping_neg();
            let mut frontier = comp;
            while frontier != 0 {
                let mut nb = 0u32;
                for x in bits(frontier) {
                    nb |= self.above[x] | self.below[x];
                }
                frontier = nb & !comp;
                comp |= nb;
            }
            left &= !comp;
            c += 1;
        }
        c
    }

    /// twins[x] = set of elements with exactly the same above and below sets as x
    /// (always contains x). Twins are pairwise incomparable and interchangeable.
    pub fn twins(&self) -> [u32; MAXN] {
        let n = self.n();
        let mut t = [0u32; MAXN];
        for x in 0..n {
            if t[x] != 0 {
                continue;
            }
            let mut m = 1u32 << x;
            for y in x + 1..n {
                if self.above[y] == self.above[x] && self.below[y] == self.below[x] {
                    m |= 1 << y;
                }
            }
            for y in bits(m) {
                t[y] = m;
            }
        }
        t
    }

    /// Random poset: random order of n elements, each forward pair related with prob. p,
    /// then closed and randomly relabeled.
    pub fn random(n: usize, p: f64, rng: &mut Rng) -> Self {
        let mut q = Self::antichain(n);
        for b in 1..n {
            for a in 0..b {
                if !q.less(a, b) && rng.chance(p) {
                    q.add_less(a, b);
                }
            }
        }
        let perm = rng.perm(n);
        q.relabel(&perm)
    }

    /// Above-masks as comma-separated lowercase hex (row a = elements > a).
    pub fn above_hex(&self) -> String {
        let mut s = String::with_capacity(5 * self.n());
        for a in 0..self.n() {
            if a > 0 {
                s.push(',');
            }
            s.push_str(&format!("{:x}", self.above[a]));
        }
        s
    }

    /// Parse the format written by `above_hex`.
    pub fn from_above_hex(s: &str) -> Option<Self> {
        let v: Result<Vec<u32>, _> = s.split(',').map(|t| u32::from_str_radix(t, 16)).collect();
        let v = v.ok()?;
        Self::from_above(v.len(), &v)
    }
}

/// A selection problem: find the i-th smallest (1-based) of the poset P.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Problem {
    pub p: Poset,
    pub i: u8,
}

impl Problem {
    pub fn new(p: Poset, i: usize) -> Self {
        assert!(i >= 1 && i <= p.n());
        Problem { p, i: i as u8 }
    }

    /// (E_n, i): no information yet.
    pub fn selection(n: usize, i: usize) -> Self {
        Self::new(Poset::antichain(n), i)
    }

    #[inline]
    pub fn n(&self) -> usize {
        self.p.n as usize
    }

    /// Solved iff some element has exactly i-1 elements below and n-i above.
    pub fn is_solved(&self) -> bool {
        let n = self.n() as u32;
        let i = self.i as u32;
        (0..self.n()).any(|x| self.p.below[x].count_ones() == i - 1 && self.p.above[x].count_ones() == n - i)
    }

    /// The set of elements that can never be the answer and are removed by `reduced`
    /// in one round: (>= i below) or (>= n-i+1 above).
    #[inline]
    fn deletable(p: &Poset, alive: u32, n: u32, i: u32) -> (u32, u32) {
        let mut hi = 0u32;
        let mut lo = 0u32;
        for x in bits(alive) {
            if (p.below[x] & alive).count_ones() >= i {
                hi |= 1 << x;
            } else if (p.above[x] & alive).count_ones() >= n - i + 1 {
                lo |= 1 << x;
            }
        }
        (hi, lo)
    }

    /// Reduction red(P, i): repeatedly delete elements that cannot be the answer
    /// (decrementing i for deleted elements known to be below the answer). The
    /// remaining elements keep their relative label order. Cost is unchanged.
    pub fn reduced(&self) -> Problem {
        let mut alive = self.p.all();
        let mut n = self.n() as u32;
        let mut i = self.i as u32;
        loop {
            let (hi, lo) = Self::deletable(&self.p, alive, n, i);
            if hi | lo == 0 {
                break;
            }
            alive &= !(hi | lo);
            n -= (hi | lo).count_ones();
            i -= lo.count_ones();
        }
        if alive == self.p.all() {
            *self
        } else {
            Problem { p: self.p.induced(alive), i: i as u8 }
        }
    }

    pub fn is_reduced(&self) -> bool {
        let (hi, lo) = Self::deletable(&self.p, self.p.all(), self.n() as u32, self.i as u32);
        hi | lo == 0
    }

    /// Duality: V_i(P) = V_{n-i+1}(P reversed).
    pub fn dual(&self) -> Problem {
        Problem { p: self.p.dual(), i: (self.n() + 1 - self.i as usize) as u8 }
    }

    /// P + (a<b), not reduced.
    #[inline]
    pub fn with_less(&self, a: usize, b: usize) -> Problem {
        Problem { p: self.p.with_less(a, b), i: self.i }
    }

    /// red(P + (a<b), i).
    #[inline]
    pub fn child(&self, a: usize, b: usize) -> Problem {
        self.with_less(a, b).reduced()
    }

    pub fn relabel(&self, perm: &[u8]) -> Problem {
        Problem { p: self.p.relabel(perm), i: self.i }
    }
}
