//! LB_best(P, i) = max(ceil(log2 Phi*), KEY), Phi* = FG + FGdual - |C|.
//!
//! Proven lower bound (code/analysis/LOWER_BOUNDS.txt, Thms 2-5). This is a
//! line-by-line port of `bounds_fast` in code/analysis/lb_candidates.py (the
//! tested Python reference); `lbcheck` + tests/lb_compare.py check exact
//! equality of C, FG, FGdual and KEY against it on random posets.
//!
//! One pass over the down-sets L of size k = i-1 (DFS in a linear-extension
//! order; x may join L only if below(x) is inside L):
//!   M = V \ L, minM = Min(P|M), maxL = Max(P|L), b = |minM| - 1
//!   C += b+1, FG += 2^b
//!   for e in minM: rest = maxL \ below(e), a = |rest| (= |Max(P|L+e)| - 1)
//!       KEY = max(KEY, a+b); if e is the highest index of Max(L+e): FGdual += 2^a

use crate::bits::bits;
use crate::count::{ceil_log2, linear_extension};
use crate::poset::{Poset, Problem, MAXN};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LbInfo {
    pub c: u64,
    pub fg: u64,
    pub fgd: u64,
    pub key: u32,
}

impl LbInfo {
    /// Phi* = FG + FGdual - |C| (>= 1).
    #[inline]
    pub fn phistar(&self) -> u64 {
        self.fg + self.fgd - self.c
    }
    /// LB_best = max(ceil(log2 Phi*), KEY).
    #[inline]
    pub fn lb(&self) -> u32 {
        ceil_log2(self.phistar()).max(self.key)
    }
    /// log2(Phi*) as a real number (heuristics only).
    #[inline]
    pub fn log2_phi(&self) -> f32 {
        (self.phistar() as f64).log2() as f32
    }
}

struct Ctx<'a> {
    p: &'a Poset,
    ord: [u8; MAXN],
    n: usize,
    k: u32,
    full: u32,
    out: LbInfo,
    /// stop as soon as LB_best > cap is certain (partial Phi* and KEY only grow)
    cap: u32,
    stopped: bool,
}

impl<'a> Ctx<'a> {
    #[inline]
    fn leaf(&mut self, l: u32) {
        let p = self.p;
        let m = self.full & !l;
        // branchless: x in Min(P|M) iff x in M and below(x) misses M;
        //             x in Max(P|L) iff x in L and above(x) misses L
        let mut min_m = 0u32;
        let mut max_l = 0u32;
        for x in 0..self.n {
            let bit = 1u32 << x;
            min_m |= bit & m & (((p.below[x] & m) == 0) as u32).wrapping_neg();
            max_l |= bit & l & (((p.above[x] & l) == 0) as u32).wrapping_neg();
        }
        let b = min_m.count_ones() - 1;
        self.out.c += b as u64 + 1;
        self.out.fg += 1u64 << b;
        for e in bits(min_m) {
            let rest = max_l & !p.below[e];
            let a = rest.count_ones();
            if a + b > self.out.key {
                self.out.key = a + b;
            }
            if rest >> e == 0 {
                self.out.fgd += 1u64 << a;
            }
        }
        if self.cap < 64 && (self.out.key > self.cap || self.out.fg + self.out.fgd - self.out.c > (1u64 << self.cap)) {
            self.stopped = true;
        }
    }

    fn rec(&mut self, j: usize, l: u32, sz: u32) {
        if self.stopped {
            return;
        }
        if sz == self.k {
            self.leaf(l);
            return;
        }
        if self.n - j < (self.k - sz) as usize {
            return;
        }
        let x = self.ord[j] as usize;
        if self.p.below[x] & !l == 0 {
            self.rec(j + 1, l | 1 << x, sz + 1);
        }
        self.rec(j + 1, l, sz);
    }
}

/// (|C|, FG, FGdual, KEY) of the problem (P, i). Works for any (also unreduced) P.
pub fn lb_info(prob: &Problem) -> LbInfo {
    let p = &prob.p;
    let mut ctx = Ctx { p, ord: linear_extension(p), n: p.n(), k: prob.i as u32 - 1, full: p.all(), out: LbInfo::default(), cap: 64, stopped: false };
    ctx.rec(0, 0, 0);
    ctx.out
}

/// Like `lb_info`, but returns None as soon as LB_best > cap is certain (the
/// partial sums Phi* = sum(2^b - b - 1) + FGdual and KEY are nondecreasing over
/// the enumeration, so the early answer is exact: LB_best > cap).
pub fn lb_info_capped(prob: &Problem, cap: u32) -> Option<LbInfo> {
    let p = &prob.p;
    let mut ctx = Ctx { p, ord: linear_extension(p), n: p.n(), k: prob.i as u32 - 1, full: p.all(), out: LbInfo::default(), cap: cap.min(63), stopped: false };
    ctx.rec(0, 0, 0);
    if ctx.stopped {
        None
    } else {
        Some(ctx.out)
    }
}

/// LB_best of the problem.
#[inline]
pub fn lb_best(prob: &Problem) -> u32 {
    lb_info(prob).lb()
}
