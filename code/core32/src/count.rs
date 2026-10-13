//! Compatible-solution counts and simple lower bounds.
//!
//! C(P, i) = set of (e, L, U): e an element, L a set of i-1 other elements, U the
//! remaining n-i elements, such that declaring L < e < U (and L < U) contradicts no
//! relation of P. Equivalently: L is a down-set of size i-1, e not in L,
//! below(e) is a subset of L (then L+e is a down-set and U an up-set). So |C| is the
//! number of pairs (D, e) with D a down-set of size i and e maximal in D.
//!
//! Every comparison outcome keeps a subset of C and the two outcomes cover C, and
//! a problem is solved iff |C| = 1, so V(P, i) >= ceil(log2 |C|). |C| is invariant
//! under reduction.

use crate::bits::bits;
use crate::poset::{Poset, Problem, MAXN};

/// A linear extension of P (elements sorted by |below|).
pub fn linear_extension(p: &Poset) -> [u8; MAXN] {
    let n = p.n();
    let mut ord = [0u8; MAXN];
    let mut k = 0;
    for h in 0..n as u32 {
        for x in 0..n {
            if p.below[x].count_ones() == h {
                ord[k] = x as u8;
                k += 1;
            }
        }
    }
    ord
}

/// |C(P, i)|.
pub fn compat_count(prob: &Problem) -> u64 {
    let p = &prob.p;
    let n = p.n();
    let k = prob.i as u32 - 1;
    // enumerate down-sets L of size i-1 by deciding elements in linear-extension
    // order (x may join L only if below(x) is already in L)
    let ord = linear_extension(p);
    rec(p, &ord, n, 0, 0, 0, k)
}

fn rec(p: &Poset, ord: &[u8; MAXN], n: usize, idx: usize, l: u32, size: u32, k: u32) -> u64 {
    if size == k {
        let all = p.all();
        let mut c = 0;
        for e in bits(all & !l) {
            if p.below[e] & !l == 0 {
                c += 1;
            }
        }
        return c;
    }
    if (n - idx) < (k - size) as usize {
        return 0;
    }
    let x = ord[idx] as usize;
    let mut t = 0;
    if p.below[x] & !l == 0 {
        t += rec(p, ord, n, idx + 1, l | 1 << x, size + 1, k);
    }
    t + rec(p, ord, n, idx + 1, l, size, k)
}

/// Brute-force |C(P, i)| straight from the definition (for tests): all (e, L) with
/// |L| = i-1 such that no relation of P is contradicted by L < e < U, L < U.
pub fn compat_count_brute(prob: &Problem) -> u64 {
    let p = &prob.p;
    let n = p.n();
    let i = prob.i as usize;
    let mut c = 0;
    for e in 0..n {
        for lmask in 0u32..(1u32 << n) {
            let l = lmask;
            if l >> e & 1 != 0 || l.count_ones() as usize != i - 1 {
                continue;
            }
            let u = p.all() & !l & !(1 << e);
            // rank: L -> 0, e -> 1, U -> 2 ; relation x<y contradicted iff rank(x) > rank(y) or (x,y both e)
            let rank = |x: usize| -> u8 {
                if l >> x & 1 != 0 {
                    0
                } else if x == e {
                    1
                } else {
                    debug_assert!(u >> x & 1 != 0);
                    2
                }
            };
            let mut ok = true;
            'outer: for x in 0..n {
                for y in bits(p.above[x]) {
                    if rank(x) > rank(y) {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            if ok {
                c += 1;
            }
        }
    }
    c
}

/// ceil(log2 x) for x >= 1.
#[inline]
pub fn ceil_log2(x: u64) -> u32 {
    if x <= 1 {
        0
    } else {
        64 - (x - 1).leading_zeros()
    }
}

/// Static lower bound: max(ceil(log2 |C|), #components - 1). Also returns |C|.
#[inline]
pub fn static_lb(prob: &Problem) -> (u32, u64) {
    if prob.n() == 1 {
        return (0, 1);
    }
    let c = compat_count(prob);
    let comps = prob.p.components() as u32;
    (ceil_log2(c).max(comps - 1), c)
}
