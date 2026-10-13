//! Exact solver for V(P, i) = minimum worst-case number of comparisons.
//!
//! V(P,i) = 0 if solved, else 1 + min over incomparable {a,b} of
//! max(V(red(P+a<b)), V(red(P+b<a))).
//!
//! `search(P, k)` decides V(P) <= k by a depth-first AND/OR search over canonical
//! reduced problems. A shared memo stores, per canonical key, proven bounds
//! lb <= V <= ub (ub = INF if unknown). Exact values come from iterative deepening:
//! repeatedly call search(P, lb) until lb == ub. Every stored bound is a proven
//! bound; nothing is stored when a search is aborted.

use crate::bits::{BuildKeyHasher, Rng};
use crate::canon::{decode_key, Canonizer, Key};
use crate::count::static_lb;
use crate::poset::Problem;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering as AO};
use std::sync::Mutex;

pub const INF: u8 = 255;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bounds {
    pub lb: u8,
    pub ub: u8,
}

/// Memo key stored as three u64 (core32: the 192-bit key itself).
type MKey = [u64; 3];

#[inline]
fn mk(k: Key) -> MKey {
    k.0
}

#[inline]
fn unmk(m: &MKey) -> Key {
    Key(*m)
}

/// Sharded concurrent memo: canonical key -> proven bounds (lb, ub).
pub struct Memo {
    shards: Vec<Mutex<HashMap<MKey, Bounds, BuildKeyHasher>>>,
    mask: usize,
}

impl Memo {
    pub fn new(shard_bits: u32) -> Self {
        let n = 1usize << shard_bits;
        Memo { shards: (0..n).map(|_| Mutex::new(HashMap::default())).collect(), mask: n - 1 }
    }

    #[inline]
    fn shard(&self, k: Key) -> &Mutex<HashMap<MKey, Bounds, BuildKeyHasher>> {
        let (f, e) = k.fold();
        let h = ((f as u64) ^ ((f >> 64) as u64).rotate_left(17) ^ e).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        &self.shards[(h >> 40) as usize & self.mask]
    }

    #[inline]
    pub fn get(&self, k: Key) -> Option<Bounds> {
        self.shard(k).lock().unwrap().get(&mk(k)).copied()
    }

    /// Merge bounds (lb = max, ub = min). Returns the merged bounds.
    #[inline]
    pub fn update(&self, k: Key, lb: u8, ub: u8) -> Bounds {
        let mut g = self.shard(k).lock().unwrap();
        let e = g.entry(mk(k)).or_insert(Bounds { lb: 0, ub: INF });
        e.lb = e.lb.max(lb);
        e.ub = e.ub.min(ub);
        assert!(e.lb <= e.ub, "inconsistent bounds for key {:x}: lb {} > ub {}", k, e.lb, e.ub);
        *e
    }

    pub fn len(&self) -> usize {
        self.shards.iter().map(|s| s.lock().unwrap().len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Visit all entries (shard by shard; do not call memo methods from `f`).
    pub fn for_each(&self, mut f: impl FnMut(Key, Bounds)) {
        for s in &self.shards {
            for (k, b) in s.lock().unwrap().iter() {
                f(unmk(k), *b);
            }
        }
    }

    /// Snapshot of all entries satisfying `pred`.
    pub fn entries_where(&self, pred: impl Fn(Key, Bounds) -> bool) -> Vec<(Key, Bounds)> {
        let mut v = Vec::new();
        self.for_each(|k, b| {
            if pred(k, b) {
                v.push((k, b));
            }
        });
        v
    }

    /// Snapshot of all entries.
    pub fn entries(&self) -> Vec<(Key, Bounds)> {
        self.entries_where(|_, _| true)
    }

    /// Snapshot of keys whose bounds are not yet exact.
    pub fn inexact_keys(&self) -> Vec<Key> {
        self.entries_where(|_, b| b.lb != b.ub).into_iter().map(|(k, _)| k).collect()
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct SearchStats {
    pub searches: u64,
    pub expansions: u64,
    pub children: u64,
    pub lb_cache_miss: u64,
}

impl SearchStats {
    pub fn add(&mut self, o: &SearchStats) {
        self.searches += o.searches;
        self.expansions += o.expansions;
        self.children += o.children;
        self.lb_cache_miss += o.lb_cache_miss;
    }
}

#[derive(Clone, Copy)]
struct LbSlot {
    key: Key,
    count: u64,
    lb: u8,
}

#[derive(Clone, Copy)]
struct ChildInfo {
    key: Key,
    lb: u8,
    ub: u8,
    count: u64,
}

#[derive(Clone, Copy)]
struct Move {
    c: [ChildInfo; 2],
    tie: u64,
}

/// One search thread. Many may share one `Memo`.
pub struct Searcher<'a> {
    pub memo: &'a Memo,
    pub canon: Canonizer,
    lbc: Vec<LbSlot>,
    lbc_mask: usize,
    pub stats: SearchStats,
    abort: Option<&'a AtomicBool>,
    rng: Rng,
    /// Randomize move order ties at depth < `jitter_depth` (for parallel helpers).
    pub jitter_depth: u32,
}

impl<'a> Searcher<'a> {
    pub fn new(memo: &'a Memo, lb_cache_bits: u32) -> Self {
        let sz = 1usize << lb_cache_bits;
        Searcher {
            memo,
            canon: Canonizer::new(),
            lbc: vec![LbSlot { key: Key::ZERO, count: 0, lb: INF }; sz],
            lbc_mask: sz - 1,
            stats: SearchStats::default(),
            abort: None,
            rng: Rng::new(0x1234_5678),
            jitter_depth: 0,
        }
    }

    pub fn with_abort(mut self, flag: &'a AtomicBool, seed: u64, jitter_depth: u32) -> Self {
        self.abort = Some(flag);
        self.rng = Rng::new(seed);
        self.jitter_depth = jitter_depth;
        self
    }

    #[inline]
    fn aborted(&self) -> bool {
        match self.abort {
            Some(f) => f.load(AO::Relaxed),
            None => false,
        }
    }

    /// (static lb, |C|) of a canonical problem, via a lossy direct-mapped cache.
    #[inline]
    fn static_info(&mut self, key: Key) -> (u8, u64) {
        let h = crate::bits::hash_key(key) as usize & self.lbc_mask;
        let s = self.lbc[h];
        if s.key == key && s.lb != INF {
            return (s.lb, s.count);
        }
        self.stats.lb_cache_miss += 1;
        let prob = decode_key(key);
        let (lb, count) = static_lb(&prob);
        self.lbc[h] = LbSlot { key, count, lb: lb as u8 };
        (lb as u8, count)
    }

    #[inline]
    fn child_info(&mut self, key: Key) -> ChildInfo {
        if crate::canon::key_n(key) == 1 {
            return ChildInfo { key, lb: 0, ub: 0, count: 1 };
        }
        let (slb, count) = self.static_info(key);
        match self.memo.get(key) {
            Some(b) => ChildInfo { key, lb: b.lb.max(slb), ub: b.ub, count },
            None => ChildInfo { key, lb: slb, ub: INF, count },
        }
    }

    /// Current proven bounds of a canonical key (memo + static).
    pub fn bounds(&mut self, key: Key) -> Bounds {
        let c = self.child_info(key);
        Bounds { lb: c.lb, ub: c.ub }
    }

    /// Canonical key of any problem (reduces first).
    pub fn key_of(&mut self, prob: &Problem) -> Key {
        let r = prob.reduced();
        self.canon.key(&r)
    }

    /// Exact V of a canonical key by iterative deepening on the lower bound.
    /// Returns None if aborted.
    pub fn exact(&mut self, key: Key) -> Option<u8> {
        loop {
            let b = self.bounds(key);
            if b.lb == b.ub {
                return Some(b.lb);
            }
            let prob = decode_key(key);
            self.search(&prob, key, b.lb, 0);
            if self.aborted() {
                return None;
            }
        }
    }

    /// Exact V(P, i) of any problem.
    pub fn value(&mut self, prob: &Problem) -> u8 {
        let key = self.key_of(prob);
        self.exact(key).expect("aborted")
    }

    /// Decide V(prob) <= k, where prob is the canonical problem of `key`.
    /// Updates the memo with proven bounds (unless aborted).
    pub fn search(&mut self, prob: &Problem, key: Key, k: u8, depth: u32) -> bool {
        self.stats.searches += 1;
        let n = prob.n();
        if n == 1 {
            return true;
        }
        let me = self.child_info(key);
        if me.ub <= k {
            return true;
        }
        if me.lb > k {
            return false;
        }
        // me.lb <= k < me.ub, k >= 1 since lb >= 1 for unsolved problems
        self.stats.expansions += 1;
        let km1 = k - 1;
        let p = &prob.p;
        let tw = p.twins();
        let mut moves: Vec<Move> = Vec::with_capacity(64);
        let mut fail_lb: u8 = INF; // min over moves of max(child lb)
        for a in 0..n {
            if tw[a] & crate::bits::low_mask(a) != 0 {
                continue; // not the first of its twin class
            }
            let inc = !(p.above[a] | p.below[a]) & p.all() & !crate::bits::low_mask(a + 1);
            for b in crate::bits::bits(inc) {
                if tw[b] == tw[a] {
                    if (tw[b] & crate::bits::low_mask(b)).count_ones() != 1 {
                        continue; // only (first, second) of the same twin class
                    }
                } else if tw[b] & crate::bits::low_mask(b) != 0 {
                    continue;
                }
                self.stats.children += 2;
                let k1 = self.canon.key(&prob.child(a, b));
                let k2 = self.canon.key(&prob.child(b, a));
                let c1 = self.child_info(k1);
                let c2 = self.child_info(k2);
                let mlb = c1.lb.max(c2.lb);
                if mlb > km1 {
                    fail_lb = fail_lb.min(mlb);
                    continue;
                }
                if c1.ub <= km1 && c2.ub <= km1 {
                    let ub = 1 + c1.ub.max(c2.ub);
                    if !self.aborted() {
                        self.memo.update(key, me.lb, ub);
                    }
                    return true;
                }
                let tie = if depth < self.jitter_depth { self.rng.next_u64() >> 40 } else { 0 };
                let (x, y) = if c1.key <= c2.key { (c1, c2) } else { (c2, c1) };
                moves.push(Move { c: [x, y], tie });
            }
        }
        // dedupe moves with identical child pairs
        moves.sort_unstable_by(|m1, m2| (m1.c[0].key, m1.c[1].key).cmp(&(m2.c[0].key, m2.c[1].key)));
        moves.dedup_by(|m1, m2| m1.c[0].key == m2.c[0].key && m1.c[1].key == m2.c[1].key);
        // order: smallest max-lb first, then most balanced split (smallest max |C|)
        moves.sort_by_key(|m| {
            let mlb = m.c[0].lb.max(m.c[1].lb);
            let mc = m.c[0].count.max(m.c[1].count);
            let sc = m.c[0].count + m.c[1].count;
            (mlb, mc, sc, m.tie)
        });
        let rot = if depth < self.jitter_depth && moves.len() > 1 { self.rng.below(moves.len()) } else { 0 };
        let nm = moves.len();
        for idx in 0..nm {
            let m = &moves[(idx + rot) % nm];
            // AND node: try the child more likely to fail first
            let (first, second) = {
                let (x, y) = (m.c[0], m.c[1]);
                if (y.lb, y.count) > (x.lb, x.count) {
                    (y, x)
                } else {
                    (x, y)
                }
            };
            let ok1 = self.search_key(first.key, km1, depth + 1);
            if self.aborted() {
                return false;
            }
            let ok2 = ok1 && self.search_key(second.key, km1, depth + 1);
            if self.aborted() {
                return false;
            }
            let f = self.child_info(first.key);
            let s = self.child_info(second.key);
            if ok2 {
                let ub = 1 + f.ub.max(s.ub);
                debug_assert!(ub <= k);
                self.memo.update(key, me.lb, ub);
                return true;
            }
            fail_lb = fail_lb.min(f.lb.max(s.lb));
        }
        debug_assert!(fail_lb != INF && fail_lb >= k);
        let lb = (k + 1).max(fail_lb.saturating_add(1));
        self.memo.update(key, lb, INF);
        false
    }

    #[inline]
    fn search_key(&mut self, key: Key, k: u8, depth: u32) -> bool {
        if crate::canon::key_n(key) == 1 {
            return true;
        }
        let ci = self.child_info(key);
        if ci.ub <= k {
            return true;
        }
        if ci.lb > k {
            return false;
        }
        let prob = decode_key(key);
        self.search(&prob, key, k, depth)
    }
}
