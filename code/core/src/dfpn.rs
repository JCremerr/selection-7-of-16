//! Depth-first proof-number search (df-pn, Nagai 2002) for the budgeted AND/OR game
//! "is V(P, i) <= b?".
//!
//! * OR node  (key, b): canonical reduced problem (iso + duality, `canon.rs`) and a
//!   budget b. PROVED if solved (n = 1); DISPROVED if LB_best > b (proven bound,
//!   `lbbest.rs`); otherwise PROVED iff some move (incomparable pair {x, y}) has
//!   both children (red(P+x<y), b-1), (red(P+y<x), b-1) proved.
//! * AND node = a move; not stored, its numbers are computed from its 2 children.
//! * Transposition table (bounded, 4-way buckets, replacement by subtree work)
//!   keyed by the canonical key. Each entry holds budget-independent PROVEN facts
//!   lb <= V <= ub (monotonicity: proved at b => proved at b' > b, disproved at
//!   b => disproved at b' < b) plus the (pn, dn) of ONE budget (the last stored).
//!   ub comes with the proving move (for certificate extraction).
//! * The search graph is a DAG (budget strictly decreases; posets strictly gain
//!   information along a path), so there is no GHI problem; only the usual DAG
//!   over-counting of sums, mitigated optionally by weak sums (WPNS-style).
//! * Leaf initialisation: heuristic (pn, dn) from the slack b - LB_best and the
//!   problem size (empirical gap distribution V - LB_best from the ground-truth
//!   table, extrapolated), see `Heur`.
//! * 1+epsilon trick (Pawlewicz & Lew 2007) for child thresholds.
//! * A move cache keeps the generated child lists of recently expanded nodes
//!   (df-pn re-expands nodes; canonical labeling is the dominant cost).
//!
//! Soundness: nothing here is load-bearing for an UPPER bound claim; the
//! certificate (cert.rs) is checked by the independent verifier. Disproofs rely on
//! the proven LB_best and on exhaustive enumeration of moves (twin pruning and
//! duplicate-child-pair removal only drop equivalent moves).

use crate::bits::{bits, hash_key, low_mask};
use crate::canon::{decode_key, key_n, Canonizer, Key};
use crate::lbbest::lb_info;
use crate::poset::Problem;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

pub type Num = u64;
/// Proof/disproof number of a solved node.
pub const INF: Num = 1 << 62;
/// Saturation cap for finite sums (strictly below INF).
pub const BIG: Num = 1 << 60;
pub const NO_UB: u8 = 255;

#[inline]
fn sat(x: Num) -> Num {
    x.min(BIG)
}

#[inline]
fn add(a: Num, b: Num) -> Num {
    if a >= INF || b >= INF {
        INF
    } else {
        sat(a + b)
    }
}

/// Leaf initialisation heuristics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Heur {
    /// pn = dn = 1 (classical).
    Unit,
    /// p = estimated P(V <= b) from slack s = b - LB_best and n:
    /// gap V - LB ~ logistic(mean mu(n) = mu_a*(n - mu_n0), scale sigma);
    /// pn = scale/p, dn = scale/(1-p).
    Slack { mu_a: f32, mu_n0: f32, sigma: f32, scale: f32, size_pow: f32 },
    /// Continuous slack: p = P(V <= b) with V - log2 Phi* ~ logistic(mean
    /// mu_c + mu_a*(n-12), normal-equivalent sd sigma) (fitted on the ground-truth
    /// table n <= 12: mean 1.27/1.46/1.70 for n = 10/11/12, sd 0.40-0.48).
    Real { mu_c: f32, mu_a: f32, sigma: f32, scale: f32 },
    /// Regression model: Vpred = log2 Phi* + w . features(P) (least squares on the
    /// ground-truth table, residual sd 0.33 at n = 12); p = logistic((b - Vpred - shift)/sigma).
    Model { sigma: f32, scale: f32, shift: f32 },
    /// cs model: Vpred = CS_W . (1, n, log2|C|) (least squares on the ground-truth
    /// table, sd 0.34 at n = 12; ranks moves like the cs ordering, which predicts
    /// the moves of upstream's optimal algorithms best); p = logistic((b-Vpred-shift)/sigma).
    Cs { sigma: f32, scale: f32, shift: f32 },
}

/// cs-model coefficients (lbcheck regress table.tsv 0,1,9).
pub const CS_W: [f64; 3] = [-1.076454, -0.081455, 1.780707];

/// Gap-model coefficients (lbcheck regress on data/groundtruth_table.tsv, n >= 6;
/// residual sd 0.30 at n = 12).
pub const GAP_W: [f64; 9] = [3.377951, -0.371112, 1.050139, -1.093396, 0.000281, 0.132329, 0.137989, -6.104265, 0.950495];

/// Predicted V of a problem by the gap model.
pub fn predict_v(prob: &Problem) -> f32 {
    let f = features(prob);
    let g: f64 = (0..9).map(|j| GAP_W[j] * f[j]).sum();
    (f[2] + g) as f32
}

#[derive(Clone, Debug)]
pub struct Config {
    /// TT has 2^tt_bits buckets of 4 entries (40 bytes each).
    pub tt_bits: u32,
    /// Move cache: 2^mc_bits slots; at most mc_max_moves move records in total.
    pub mc_bits: u32,
    pub mc_max_moves: usize,
    /// LB cache: 2^lbc_bits direct-mapped slots.
    pub lbc_bits: u32,
    /// 1+eps trick.
    pub eps: f64,
    pub heur: Heur,
    /// pn(AND) = max + 1 (weak) instead of sum.
    pub and_weak: bool,
    /// increment of the weak AND sum: pn = max(pn1, pn2) + and_add
    pub and_add: u64,
    /// if > 0: leaf numbers are -logp * ln(probability) (see init_numbers)
    pub logp: f64,
    /// static order key starts with max(LB_best of the children)
    pub order_lb_first: bool,
    /// early-terminated LB_best for LB-dead children in move generation
    pub lb_early: bool,
    /// static move order: cs (max log2|C|, max/min log2 Phi*) instead of (max h, sum h)
    pub order_cs: bool,
    /// seed for tie-breaking / jitter (0 = deterministic static order)
    pub seed: u64,
    /// relative noise on leaf numbers: x * (1 + jitter * u), u in [0,1) from hash(key, seed)
    pub jitter: f32,
    /// dn(OR) = max + (#alive - 1) (weak) instead of sum.
    pub or_weak: bool,
    /// Optional exact-value oracle (canonical key -> V), e.g. the ground-truth table.
    pub oracle: Option<Rc<HashMap<Key, u8>>>,
    pub time_limit: f64,
    pub node_limit: u64,
    pub progress_secs: f64,
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tt_bits: 20,
            mc_bits: 14,
            mc_max_moves: 4_000_000,
            lbc_bits: 20,
            eps: 1.0,
            heur: Heur::Cs { sigma: 0.2, scale: 10.0, shift: 1.0 },
            and_weak: true,
            and_add: 1,
            logp: 0.0,
            order_cs: true,
            lb_early: false,
            order_lb_first: true,
            seed: 0,
            jitter: 0.0,
            or_weak: false,
            oracle: None,
            time_limit: f64::INFINITY,
            node_limit: u64::MAX,
            progress_secs: 30.0,
            verbose: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Stats {
    pub expansions: u64,
    pub movegens: u64,
    pub mc_hits: u64,
    pub canon_calls: u64,
    pub lb_computed: u64,
    pub tt_stores: u64,
    pub tt_replaced: u64,
    pub tt_used: u64,
    pub oracle_hits: u64,
    pub max_depth: u32,
    /// expansions per depth (0 = root)
    pub by_depth: [u64; 40],
    /// seconds spent in canonical labeling / LB computation (move generation)
    pub t_canon: f64,
    pub t_lb: f64,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            expansions: 0,
            movegens: 0,
            mc_hits: 0,
            canon_calls: 0,
            lb_computed: 0,
            tt_stores: 0,
            tt_replaced: 0,
            tt_used: 0,
            oracle_hits: 0,
            max_depth: 0,
            by_depth: [0; 40],
            t_canon: 0.0,
            t_lb: 0.0,
        }
    }
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
struct Entry {
    k0: u64,
    k1: u64,
    pn: Num,
    dn: Num,
    work: u32,
    b: u8,
    lb: u8,
    ub: u8,
    mv: u8,
}

impl Entry {
    #[inline]
    fn prio(&self) -> u64 {
        let solved_bonus = if self.ub != NO_UB { 64 } else { 0 } + if self.lb != 0 { 16 } else { 0 };
        self.work as u64 + solved_bonus
    }
}

const WAYS: usize = 4;

struct Tt {
    e: Vec<Entry>,
    mask: usize,
}

impl Tt {
    fn new(bits: u32) -> Self {
        let nb = 1usize << bits;
        Tt { e: vec![Entry::default(); nb * WAYS], mask: nb - 1 }
    }
    #[inline]
    fn bucket(&self, k: Key) -> usize {
        (hash_key(k) as usize & self.mask) * WAYS
    }
    #[inline]
    fn find(&self, k: Key) -> Option<&Entry> {
        let b = self.bucket(k);
        let (k0, k1) = (k as u64, (k >> 64) as u64);
        self.e[b..b + WAYS].iter().find(|e| e.k0 == k0 && e.k1 == k1)
    }
    /// Entry for k (existing, or a fresh one replacing the lowest-priority slot).
    #[inline]
    fn slot(&mut self, k: Key, st: &mut Stats) -> &mut Entry {
        let b = self.bucket(k);
        let (k0, k1) = (k as u64, (k >> 64) as u64);
        let mut victim = b;
        let mut vp = u64::MAX;
        for j in b..b + WAYS {
            let e = &self.e[j];
            if e.k0 == k0 && e.k1 == k1 {
                return &mut self.e[j];
            }
            let p = if e.k0 == 0 && e.k1 == 0 { 0 } else { e.prio() + 1 };
            if p < vp {
                vp = p;
                victim = j;
            }
        }
        if vp == 0 {
            st.tt_used += 1;
        } else {
            st.tt_replaced += 1;
        }
        self.e[victim] = Entry { k0, k1, pn: 1, dn: 1, work: 0, b: 255, lb: 0, ub: NO_UB, mv: 0 };
        &mut self.e[victim]
    }
}

/// A move of a canonical problem: compare canonical elements (mv & 15) and (mv >> 4);
/// children keys k1, k2 with static lower bounds and log2 Phi*.
#[derive(Clone, Copy)]
pub struct MoveRec {
    pub k1: Key,
    pub k2: Key,
    pub h1: f32,
    pub h2: f32,
    /// static order key (smaller = tried first)
    pub ord: [f32; 3],
    pub lb1: u8,
    pub lb2: u8,
    pub mv: u8,
}

/// Value of a node: proof/disproof numbers and proven bounds lo <= V <= hi.
#[derive(Clone, Copy, Debug)]
pub struct Val {
    pub pn: Num,
    pub dn: Num,
    pub lo: u8,
    pub hi: u8,
    /// proving move (canonical labels) if pn == 0 and the node is an expanded OR node
    pub mv: u8,
}

impl Val {
    #[inline]
    fn proved(hi: u8, mv: u8) -> Val {
        Val { pn: 0, dn: INF, lo: 0, hi, mv }
    }
    #[inline]
    fn disproved(lo: u8) -> Val {
        Val { pn: INF, dn: 0, lo, hi: NO_UB, mv: 0 }
    }
}

#[derive(Clone, Copy)]
struct LbSlot {
    key: Key,
    lb: u8,
    h: f32,
    /// log2 |C|, log2 Phi*
    lc: f32,
    lp: f32,
    valid: bool,
}

pub struct Dfpn {
    pub cfg: Config,
    pub canon: Canonizer,
    tt: Tt,
    /// move cache: (key, budget generated for, dead lower bound, alive moves)
    mc: Vec<Option<(Key, u8, u8, Rc<[MoveRec]>)>>,
    mc_moves: usize,
    lbc: Vec<LbSlot>,
    pub st: Stats,
    t0: Instant,
    last_progress: f64,
    pub aborted: bool,
    /// root bookkeeping for progress lines
    root_key: Key,
    root_b: u8,
    /// last (pn, dn) computed at depth 0
    pub root_num: (Num, Num),
}

impl Dfpn {
    pub fn new(cfg: Config) -> Self {
        let mc = vec![None; 1usize << cfg.mc_bits];
        let lbc = vec![LbSlot { key: 0, lb: 0, h: 0.0, lc: 0.0, lp: 0.0, valid: false }; 1usize << cfg.lbc_bits];
        Dfpn {
            tt: Tt::new(cfg.tt_bits),
            mc,
            mc_moves: 0,
            lbc,
            canon: Canonizer::new(),
            st: Stats::default(),
            t0: Instant::now(),
            last_progress: 0.0,
            aborted: false,
            root_key: 0,
            root_b: 0,
            root_num: (0, 0),
            cfg,
        }
    }

    pub fn elapsed(&self) -> f64 {
        self.t0.elapsed().as_secs_f64()
    }

    /// Approximate memory of the fixed tables in bytes.
    pub fn table_bytes(&self) -> usize {
        self.tt.e.len() * std::mem::size_of::<Entry>()
            + self.mc.len() * std::mem::size_of::<Option<(Key, u8, u8, Rc<[MoveRec]>)>>()
            + self.mc_moves * std::mem::size_of::<MoveRec>()
            + self.lbc.len() * std::mem::size_of::<LbSlot>()
    }

    /// (LB_best, heuristic value h) of a canonical key (cached).
    #[inline]
    pub fn static_info(&mut self, key: Key) -> (u8, f32) {
        let s = self.static_full(key);
        (s.lb, s.h)
    }

    /// static_full, unless LB_best > cap is certain: then None (computed with early
    /// termination, nothing cached).
    #[inline]
    fn static_capped(&mut self, key: Key, cap: u8) -> Option<LbSlot> {
        if key_n(key) == 1 {
            return Some(self.static_full(key));
        }
        let h = hash_key(key) as usize & (self.lbc.len() - 1);
        let s = self.lbc[h];
        if s.valid && s.key == key {
            return if s.lb > cap { None } else { Some(s) };
        }
        if !self.cfg.lb_early {
            let s = self.static_full(key);
            return if s.lb > cap { None } else { Some(s) };
        }
        self.st.lb_computed += 1;
        let t0 = Instant::now();
        let prob = decode_key(key);
        let r = crate::lbbest::lb_info_capped(&prob, cap as u32);
        let out = match r {
            None => None,
            Some(li) => {
                let lb = li.lb() as u8;
                let hv = match self.cfg.heur {
                    Heur::Model { .. } => predict_v(&prob),
                    Heur::Cs { .. } => (CS_W[0] + CS_W[1] * prob.n() as f64 + CS_W[2] * (li.c as f64).log2()) as f32,
                    _ => li.log2_phi(),
                };
                let slot = LbSlot { key, lb, h: hv, lc: (li.c as f64).log2() as f32, lp: li.log2_phi(), valid: true };
                self.lbc[h] = slot;
                if lb > cap {
                    None
                } else {
                    Some(slot)
                }
            }
        };
        self.st.t_lb += t0.elapsed().as_secs_f64();
        out
    }

    /// Cached static information: LB_best, heuristic value h (log2 Phi* or a
    /// predicted V, depending on the heuristic), log2 |C|, log2 Phi*.
    #[inline]
    fn static_full(&mut self, key: Key) -> LbSlot {
        if key_n(key) == 1 {
            return LbSlot { key, lb: 0, h: 0.0, lc: 0.0, lp: 0.0, valid: true };
        }
        let h = hash_key(key) as usize & (self.lbc.len() - 1);
        let s = self.lbc[h];
        if s.valid && s.key == key {
            return s;
        }
        self.st.lb_computed += 1;
        let t0 = Instant::now();
        let prob = decode_key(key);
        let li = lb_info(&prob);
        let lb = li.lb() as u8;
        let hv = match self.cfg.heur {
            Heur::Model { .. } => predict_v(&prob),
            Heur::Cs { .. } => (CS_W[0] + CS_W[1] * prob.n() as f64 + CS_W[2] * (li.c as f64).log2()) as f32,
            _ => li.log2_phi(),
        };
        let slot = LbSlot { key, lb, h: hv, lc: (li.c as f64).log2() as f32, lp: li.log2_phi(), valid: true };
        self.st.t_lb += t0.elapsed().as_secs_f64();
        self.lbc[h] = slot;
        slot
    }

    /// Proven bounds (lo, hi) of a key from LB_best, oracle and TT.
    pub fn bounds(&mut self, key: Key) -> (u8, u8) {
        if key_n(key) == 1 {
            return (0, 0);
        }
        if let Some(o) = &self.cfg.oracle {
            if let Some(&v) = o.get(&key) {
                return (v, v);
            }
        }
        let (mut lo, _) = self.static_info(key);
        let mut hi = NO_UB;
        if let Some(e) = self.tt.find(key) {
            lo = lo.max(e.lb);
            hi = e.ub;
        }
        (lo, hi)
    }

    /// Proving move (canonical labels) stored for key, if V <= ub with ub <= b is known.
    pub fn proof_move(&self, key: Key, b: u8) -> Option<(u8, u8)> {
        let e = self.tt.find(key)?;
        if e.ub <= b {
            Some((e.mv & 15, e.mv >> 4))
        } else {
            None
        }
    }

    /// Estimated P(V <= b) of an unexpanded node (None for Heur::Unit).
    #[inline]
    fn prob_provable(&self, n: usize, lb: u8, h: f32, b: u8) -> Option<f32> {
        let p = match self.cfg.heur {
            Heur::Unit => return None,
            Heur::Slack { mu_a, mu_n0, sigma, .. } => {
                let s = (b - lb) as f32;
                let mu = (mu_a * (n as f32 - mu_n0)).max(0.0);
                1.0 / (1.0 + (-(s + 0.5 - mu) / sigma).exp())
            }
            Heur::Model { sigma, shift, .. } | Heur::Cs { sigma, shift, .. } => 1.0 / (1.0 + (-1.7 * (b as f32 - h - shift) / sigma).exp()),
            Heur::Real { mu_c, mu_a, sigma, .. } => {
                let mu = mu_c + mu_a * (n as f32 - 12.0);
                1.0 / (1.0 + (-1.7 * (b as f32 - h - mu) / sigma).exp())
            }
        };
        Some(p.clamp(1e-6, 1.0 - 1e-6))
    }

    #[inline]
    pub fn init_numbers(&self, key: Key, n: usize, lb: u8, h: f32, b: u8) -> (Num, Num) {
        let (pn, dn) = self.init_numbers0(n, lb, h, b);
        if self.cfg.jitter > 0.0 {
            let r = hash_key(key ^ (self.cfg.seed as u128) << 7 ^ b as u128);
            let u1 = (r & 0xffff) as f32 / 65536.0;
            let u2 = ((r >> 16) & 0xffff) as f32 / 65536.0;
            let j = self.cfg.jitter;
            (((pn as f32) * (1.0 + j * u1)).round().max(1.0) as Num, ((dn as f32) * (1.0 + j * u2)).round().max(1.0) as Num)
        } else {
            (pn, dn)
        }
    }

    #[inline]
    fn init_numbers0(&self, n: usize, lb: u8, h: f32, b: u8) -> (Num, Num) {
        let p = match self.prob_provable(n, lb, h, b) {
            None => return (1, 1),
            Some(p) => p,
        };
        let scale = match self.cfg.heur {
            Heur::Slack { scale, size_pow, .. } => scale * if size_pow != 0.0 { (n as f32).powf(size_pow) } else { 1.0 },
            Heur::Model { scale, .. } | Heur::Real { scale, .. } | Heur::Cs { scale, .. } => scale,
            Heur::Unit => 1.0,
        };
        if self.cfg.logp > 0.0 {
            // -log-probability costs: sums at AND (pn) and OR (dn) = products of probabilities
            let pn = (-(p as f64).ln() * self.cfg.logp).round().max(1.0) as Num;
            let dn = (-(1.0 - p as f64).ln() * self.cfg.logp).round().max(1.0) as Num;
            (pn, dn)
        } else {
            let pn = (scale / p).round().max(1.0) as Num;
            let dn = (scale / (1.0 - p)).round().max(1.0) as Num;
            (pn, dn)
        }
    }

    /// Current value of the OR node (key, b) without expanding it.
    #[inline]
    fn val(&mut self, key: Key, slb: u8, h: f32, b: u8, local: &[(Key, Val)]) -> Val {
        let n = key_n(key);
        if n == 1 {
            return Val::proved(0, 0);
        }
        if slb > b {
            return Val::disproved(slb);
        }
        if let Some(o) = &self.cfg.oracle {
            if let Some(&v) = o.get(&key) {
                self.st.oracle_hits += 1;
                return if v <= b { Val::proved(v, 0) } else { Val::disproved(v) };
            }
        }
        let mut lo = slb;
        if let Some(e) = self.tt.find(key) {
            lo = lo.max(e.lb);
            if e.ub <= b {
                return Val::proved(e.ub, e.mv);
            }
            if lo > b {
                return Val::disproved(lo);
            }
            if e.b == b {
                return Val { pn: e.pn, dn: e.dn, lo, hi: e.ub, mv: 0 };
            }
        }
        if let Some((_, v)) = local.iter().find(|(k, _)| *k == key) {
            return *v;
        }
        let (pn, dn) = self.init_numbers(key, n, lo, h, b);
        Val { pn, dn, lo, hi: NO_UB, mv: 0 }
    }

    /// Move list of a canonical key for budget b (cached): only moves whose two
    /// children both have LB_best <= b-1 are generated in full. For every other
    /// pair, the outcome that adds fewer relations (more likely to be LB-dead) is
    /// evaluated first and the other outcome is skipped when the first is dead (no
    /// canonical labeling, no LB). Returns (alive moves sorted by the static order,
    /// min over the dead moves of a lower bound on max(V(c1), V(c2))). A list made
    /// for budget b is valid for every budget <= b; a visit at a higher budget
    /// regenerates it.
    pub fn moves(&mut self, key: Key, b: u8) -> (Rc<[MoveRec]>, u8) {
        let slot = (hash_key(key) >> 20) as usize & (self.mc.len() - 1);
        if let Some((k, gb, dl, m)) = &self.mc[slot] {
            if *k == key && *gb >= b {
                self.st.mc_hits += 1;
                return (m.clone(), *dl);
            }
        }
        self.st.movegens += 1;
        let bc = b.saturating_sub(1);
        let prob = decode_key(key);
        let p = &prob.p;
        let n = prob.n();
        let tw = p.twins();
        let mut v: Vec<MoveRec> = Vec::with_capacity(64);
        let mut dead_lo: u8 = NO_UB;
        for a in 0..n {
            if tw[a] & low_mask(a) != 0 {
                continue;
            }
            let inc = !(p.above[a] | p.below[a]) & p.all() & !low_mask(a + 1);
            for b2 in bits(inc) {
                if tw[b2] == tw[a] {
                    if (tw[b2] & low_mask(b2)).count_ones() != 1 {
                        continue;
                    }
                } else if tw[b2] & low_mask(b2) != 0 {
                    continue;
                }
                // outcome adding fewer relations first (keeps more solutions)
                let r_ab = (p.below[a].count_ones() + 1) * (p.above[b2].count_ones() + 1);
                let r_ba = (p.below[b2].count_ones() + 1) * (p.above[a].count_ones() + 1);
                let (x, y) = if r_ab <= r_ba { (a, b2) } else { (b2, a) };
                let cf = prob.child(x, y);
                self.st.canon_calls += 1;
                let t0 = Instant::now();
                let kf = self.canon.key(&cf);
                self.st.t_canon += t0.elapsed().as_secs_f64();
                let sf = match self.static_capped(kf, bc) {
                    Some(x) => x,
                    None => {
                        dead_lo = dead_lo.min(bc + 1);
                        continue;
                    }
                };
                let cs = prob.child(y, x);
                self.st.canon_calls += 1;
                let t0 = Instant::now();
                let ks = self.canon.key(&cs);
                self.st.t_canon += t0.elapsed().as_secs_f64();
                let ss = match self.static_capped(ks, bc) {
                    Some(x) => x,
                    None => {
                        dead_lo = dead_lo.min(bc + 1);
                        continue;
                    }
                };
                let (mut k1, mut k2, mut s1, mut s2) = (kf, ks, sf, ss);
                let mut mv = (x | y << 4) as u8;
                if k1 > k2 {
                    std::mem::swap(&mut k1, &mut k2);
                    std::mem::swap(&mut s1, &mut s2);
                    mv = (y | x << 4) as u8;
                }
                let ord = if self.cfg.order_cs {
                    // max log2|C|, then max log2 Phi*, then min log2 Phi* (best top-1 rate
                    // on the moves of upstream's optimal (13,7), (15,6), (15,7) algorithms)
                    [s1.lc.max(s2.lc), s1.lp.max(s2.lp), s1.lp.min(s2.lp)]
                } else {
                    [s1.h.max(s2.h), s1.h + s2.h, 0.0]
                };
                v.push(MoveRec { k1, k2, h1: s1.h, h2: s2.h, ord, lb1: s1.lb, lb2: s2.lb, mv });
            }
        }
        v.sort_unstable_by(|x, y| (x.k1, x.k2).cmp(&(y.k1, y.k2)));
        v.dedup_by(|x, y| x.k1 == y.k1 && x.k2 == y.k2);
        let seed = self.cfg.seed as u128;
        let tb = |m: &MoveRec| if seed == 0 { 0 } else { hash_key(m.k1 ^ m.k2.rotate_left(17) ^ seed) };
        let lbf = self.cfg.order_lb_first;
        v.sort_by(|x, y| {
            let kx = (if lbf { x.lb1.max(x.lb2) } else { 0 }, x.ord[0], x.ord[1], x.ord[2]);
            let ky = (if lbf { y.lb1.max(y.lb2) } else { 0 }, y.ord[0], y.ord[1], y.ord[2]);
            kx.partial_cmp(&ky).unwrap().then(tb(x).cmp(&tb(y)))
        });
        let rc: Rc<[MoveRec]> = v.into();
        // bounded total size: evict this slot's old list; if over budget, flush all
        if let Some((_, _, _, old)) = &self.mc[slot] {
            self.mc_moves -= old.len();
        }
        if self.mc_moves + rc.len() > self.cfg.mc_max_moves {
            for s in self.mc.iter_mut() {
                *s = None;
            }
            self.mc_moves = 0;
        }
        self.mc_moves += rc.len();
        self.mc[slot] = Some((key, b, dead_lo, rc.clone()));
        (rc, dead_lo)
    }

    fn store(&mut self, key: Key, b: u8, v: &Val, work: u64) {
        self.st.tt_stores += 1;
        let e = self.tt.slot(key, &mut self.st);
        e.work = e.work.saturating_add(work.min(u32::MAX as u64) as u32);
        if v.pn == 0 {
            if v.hi < e.ub {
                e.ub = v.hi;
                e.mv = v.mv;
            }
        } else if v.dn == 0 {
            e.lb = e.lb.max(v.lo);
        } else {
            e.b = b;
            e.pn = v.pn;
            e.dn = v.dn;
        }
    }

    fn check_limits(&mut self) {
        if self.st.expansions & 1023 != 0 {
            return;
        }
        let t = self.elapsed();
        if t > self.cfg.time_limit || self.st.expansions >= self.cfg.node_limit {
            self.aborted = true;
        }
        if t - self.last_progress >= self.cfg.progress_secs {
            self.last_progress = t;
            self.progress_line();
        }
    }

    /// Counts of TT entries: (used, with proven ub, with search lb, with pn/dn).
    pub fn tt_census(&self) -> (u64, u64, u64) {
        let (mut used, mut proved, mut disproved) = (0, 0, 0);
        for e in &self.tt.e {
            if e.k0 == 0 && e.k1 == 0 {
                continue;
            }
            used += 1;
            proved += (e.ub != NO_UB) as u64;
            disproved += (e.lb != 0) as u64;
        }
        (used, proved, disproved)
    }

    /// Principal-variation report: from (key, b), for `levels` levels print the
    /// alive / refuted / proved move counts and the best numbers, then follow the
    /// most-proving move and its child with the smallest dn (df-pn's choice).
    pub fn pv_report(&mut self, mut key: Key, mut b: u8, levels: usize) -> String {
        let mut out = String::new();
        for d in 0..levels {
            if key_n(key) == 1 || b == 0 {
                break;
            }
            let (lo, hi) = self.bounds(key);
            let (all, _) = self.moves(key, b);
            let bc = b - 1;
            let moves: Vec<MoveRec> = all.iter().copied().filter(|m| m.lb1 <= bc && m.lb2 <= bc).collect();
            let mut vals: Vec<(Num, Num, usize)> = Vec::new();
            let (mut refuted, mut proved) = (0, 0);
            for (j, m) in moves.iter().enumerate() {
                let v1 = self.val(m.k1, m.lb1, m.h1, bc, &[]);
                let v2 = self.val(m.k2, m.lb2, m.h2, bc, &[]);
                let pn = self.and_pn(v1.pn, v2.pn);
                let dn = v1.dn.min(v2.dn);
                if dn == 0 {
                    refuted += 1;
                } else if pn == 0 {
                    proved += 1;
                }
                vals.push((pn, dn, j));
            }
            vals.sort();
            let best = vals.first().copied();
            out.push_str(&format!(
                "#   pv d={:2} n={:2} b={:2} bounds=[{},{}] moves alive(LB)={:3} refuted={:3} proved={:3}",
                d,
                key_n(key),
                b,
                lo,
                if hi == NO_UB { -1 } else { hi as i32 },
                moves.len(),
                refuted,
                proved
            ));
            match best {
                None => {
                    out.push_str("  (no alive move)\n");
                    break;
                }
                Some((pn, dn, j)) => {
                    let second = vals.get(1).map(|v| v.0).unwrap_or(INF);
                    out.push_str(&format!("  best pn {} dn {} | 2nd pn {}\n", pn, dn, second));
                    if pn == 0 || dn == 0 {
                        break;
                    }
                    let m = moves[j];
                    let v1 = self.val(m.k1, m.lb1, m.h1, bc, &[]);
                    let v2 = self.val(m.k2, m.lb2, m.h2, bc, &[]);
                    key = if (v1.dn, std::cmp::Reverse(v1.pn)) <= (v2.dn, std::cmp::Reverse(v2.pn)) { m.k1 } else { m.k2 };
                    b = bc;
                }
            }
        }
        out
    }

    /// "d:count" list of expansions per depth.
    pub fn depth_profile(&self) -> String {
        let last = self.st.by_depth.iter().rposition(|&c| c > 0).unwrap_or(0);
        (0..=last).map(|d| format!("{}:{}", d, self.st.by_depth[d])).collect::<Vec<_>>().join(" ")
    }

    pub fn progress_line(&mut self) {
        let t = self.elapsed();
        let rv = self.root_num;
        let cap = self.tt.e.len() as f64;
        println!(
            "[{:8.1}s] exp {:>11} ({:>7.0}/s) movegen {:>10} mc_hit {:>10} canon {:>11} ({:.0}s) lbc {:>10} ({:.0}s) tt_fill {:.3} repl {:>10} maxdepth {} root pn {} dn {} rss {} MB",
            t,
            self.st.expansions,
            self.st.expansions as f64 / t.max(1e-9),
            self.st.movegens,
            self.st.mc_hits,
            self.st.canon_calls,
            self.st.t_canon,
            self.st.lb_computed,
            self.st.t_lb,
            self.st.tt_used as f64 / cap,
            self.st.tt_replaced,
            self.st.max_depth,
            rv.0,
            rv.1,
            rss_mb()
        );
    }

    /// Solve (key, b) completely (thresholds INF) unless limits are hit.
    pub fn solve(&mut self, key: Key, b: u8) -> Val {
        self.root_key = key;
        self.root_b = b;
        self.aborted = false;
        let (slb, h) = self.static_info(key);
        let v = self.val(key, slb, h, b, &[]);
        if v.pn == 0 || v.dn == 0 {
            return v;
        }
        self.mid_or(key, b, INF, INF, 0)
    }

    /// Like `solve` for a key that is known to be provable at budget b: if the TT
    /// already has a proof move, it is returned without search.
    pub fn proof_of(&mut self, key: Key, b: u8) -> Option<(u8, u8)> {
        if let Some(m) = self.proof_move(key, b) {
            return Some(m);
        }
        let saved = (self.root_key, self.root_b);
        let v = self.mid_or(key, b, INF, INF, 0);
        self.root_key = saved.0;
        self.root_b = saved.1;
        if v.pn == 0 {
            Some((v.mv & 15, v.mv >> 4))
        } else {
            None
        }
    }

    #[inline]
    pub fn and_pn(&self, a: Num, b: Num) -> Num {
        if self.cfg.and_weak {
            if a == 0 {
                b
            } else if b == 0 {
                a
            } else if a >= INF || b >= INF {
                INF
            } else {
                sat(a.max(b) + self.cfg.and_add)
            }
        } else {
            add(a, b)
        }
    }

    /// Expand OR node (key, b) (unsolved, LB_best <= b) until pn >= th_pn or dn >= th_dn.
    fn mid_or(&mut self, key: Key, b: u8, th_pn: Num, th_dn: Num, depth: u32) -> Val {
        self.st.expansions += 1;
        self.st.by_depth[(depth as usize).min(39)] += 1;
        if depth > self.st.max_depth {
            self.st.max_depth = depth;
        }
        self.check_limits();
        let start = self.st.expansions;
        debug_assert!(b >= 1);
        let bc = b - 1;
        let (moves, dead0) = self.moves(key, b);
        let mut alive: Vec<u16> = Vec::with_capacity(moves.len());
        let mut dead_lo: u8 = dead0;
        for (idx, m) in moves.iter().enumerate() {
            if m.lb1 > bc || m.lb2 > bc {
                dead_lo = dead_lo.min(m.lb1.max(m.lb2));
            } else {
                alive.push(idx as u16);
            }
        }
        let mut local: Vec<(Key, Val)> = Vec::new();
        loop {
            let mut best: usize = usize::MAX;
            let mut best_v = (INF, 0 as Num);
            let mut pn2: Num = INF;
            let mut dn_sum: Num = 0;
            let mut dn_max: Num = 0;
            let mut cnt: u64 = 0;
            let mut min_lo = dead_lo;
            let mut ai = 0;
            while ai < alive.len() {
                let m = moves[alive[ai] as usize];
                let v1 = self.val(m.k1, m.lb1, m.h1, bc, &local);
                let v2 = self.val(m.k2, m.lb2, m.h2, bc, &local);
                let mpn = self.and_pn(v1.pn, v2.pn);
                let mdn = v1.dn.min(v2.dn);
                if mpn == 0 {
                    let hi = 1 + v1.hi.max(v2.hi);
                    let v = Val::proved(hi, m.mv);
                    let work = self.st.expansions - start + 1;
                    self.store(key, b, &v, work);
                    return v;
                }
                if mdn == 0 {
                    // move refuted at this budget: drop it for the rest of this visit
                    let lo = 1 + if v1.dn == 0 { v1.lo } else { v2.lo };
                    let lo = lo.max(1 + v1.lo.max(v2.lo));
                    min_lo = min_lo.min(lo - 1);
                    dead_lo = dead_lo.min(lo - 1);
                    alive.remove(ai);
                    continue;
                }
                cnt += 1;
                dn_sum = add(dn_sum, mdn);
                dn_max = dn_max.max(mdn);
                if mpn < best_v.0 || (mpn == best_v.0 && best == usize::MAX) {
                    pn2 = best_v.0;
                    best_v = (mpn, mdn);
                    best = ai;
                } else if mpn < pn2 {
                    pn2 = mpn;
                }
                ai += 1;
            }
            if cnt == 0 {
                let v = Val::disproved((1 + min_lo).max(b + 1));
                let work = self.st.expansions - start + 1;
                self.store(key, b, &v, work);
                return v;
            }
            let pn = best_v.0;
            let dn = if self.cfg.or_weak { sat(dn_max + cnt - 1) } else { dn_sum };
            if depth == 0 {
                self.root_num = (pn, dn);
            }
            if pn >= th_pn || dn >= th_dn || self.aborted {
                let (lo, _) = (dead_lo, 0);
                let _ = lo;
                let v = Val { pn, dn, lo: 0, hi: NO_UB, mv: 0 };
                let work = self.st.expansions - start + 1;
                self.store(key, b, &v, work);
                return v;
            }
            // child thresholds
            let cth_pn = if pn2 >= INF {
                th_pn
            } else {
                let e = ((pn2 as f64) * (1.0 + self.cfg.eps)).ceil() as Num;
                th_pn.min(e.max(pn2 + 1))
            };
            let cth_dn = if th_dn >= INF {
                INF
            } else if self.cfg.or_weak {
                (th_dn - (cnt - 1)).min(INF)
            } else {
                (th_dn - dn + best_v.1).min(INF)
            };
            let m = moves[alive[best] as usize];
            self.mid_and(&m, bc, cth_pn, cth_dn, depth, &mut local);
            if self.aborted {
                let v = Val { pn, dn, lo: 0, hi: NO_UB, mv: 0 };
                return v;
            }
        }
    }

    /// AND node = move m at child budget bc.
    fn mid_and(&mut self, m: &MoveRec, bc: u8, th_pn: Num, th_dn: Num, depth: u32, local: &mut Vec<(Key, Val)>) {
        loop {
            let v1 = self.val(m.k1, m.lb1, m.h1, bc, local);
            let v2 = self.val(m.k2, m.lb2, m.h2, bc, local);
            let pn = self.and_pn(v1.pn, v2.pn);
            let dn = v1.dn.min(v2.dn);
            if pn == 0 || dn == 0 || pn >= th_pn || dn >= th_dn || self.aborted {
                return;
            }
            // child most likely to fail first: min dn; tie -> larger pn
            let first = (v1.dn, std::cmp::Reverse(v1.pn)) <= (v2.dn, std::cmp::Reverse(v2.pn));
            let (kc, vc, vo) = if first { (m.k1, v1, v2) } else { (m.k2, v2, v1) };
            let cth_dn = if vo.dn >= INF {
                th_dn
            } else {
                let e = ((vo.dn as f64) * (1.0 + self.cfg.eps)).ceil() as Num;
                th_dn.min(e.max(vo.dn + 1))
            };
            let cth_pn = if th_pn >= INF {
                INF
            } else if self.cfg.and_weak {
                if vo.pn == 0 {
                    th_pn
                } else {
                    th_pn - self.cfg.and_add
                }
            } else {
                (th_pn - pn + vc.pn).min(INF)
            };
            let v = self.mid_or(kc, bc, cth_pn, cth_dn, depth + 1);
            if let Some(slot) = local.iter_mut().find(|(k, _)| *k == kc) {
                slot.1 = v;
            } else {
                local.push((kc, v));
            }
            if self.aborted {
                return;
            }
        }
    }
}

/// Features for the gap model V - log2 Phi* (see Heur::Model):
/// [1, n, x = log2 Phi*, related-pair fraction, #components, KEY - x,
///  (#minimal + #maximal)/n, x/n].
pub fn features(prob: &Problem) -> [f64; 9] {
    let p = &prob.p;
    let n = prob.n() as f64;
    let li = lb_info(prob);
    let x = li.log2_phi() as f64;
    let pairs = (n * (n - 1.0) / 2.0).max(1.0);
    let rel = p.num_relations() as f64 / pairs;
    let comps = p.components() as f64;
    let mut mm = 0u32;
    for a in 0..prob.n() {
        mm += (p.below[a] == 0) as u32 + (p.above[a] == 0) as u32;
    }
    [1.0, n, x, rel, comps, li.key as f64 - x, mm as f64 / n, x / n, (li.c as f64).log2() - x]
}

/// Resident set size of this process in MB (Linux /proc).
pub fn rss_mb() -> u64 {
    std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| s.split_whitespace().nth(1).and_then(|x| x.parse::<u64>().ok()))
        .map(|pages| pages * 4096 / (1 << 20))
        .unwrap_or(0)
}

/// Root problem key helper.
pub fn root_key(canon: &mut Canonizer, n: usize, i: usize) -> Key {
    let p = Problem::selection(n, i).reduced();
    canon.key(&p)
}

/// Load an exact-value table (data/groundtruth_table.tsv: n, i, V, above_hex, ...,
/// flag "exact") as an oracle keyed by OUR canonical key (every row is
/// re-canonicalised, so the table's labeling does not matter). Rows with n > max_n
/// are skipped. Returns (map, rows, rows whose stored poset was not already in
/// canonical form).
pub fn load_oracle(path: &str, max_n: usize) -> std::io::Result<(HashMap<Key, u8>, usize, usize)> {
    use std::io::BufRead;
    let f = std::fs::File::open(path)?;
    let r = std::io::BufReader::new(f);
    let mut canon = Canonizer::new();
    let mut map = HashMap::new();
    let (mut rows, mut noncanon) = (0usize, 0usize);
    for line in r.lines() {
        let line = line?;
        let t: Vec<&str> = line.split('\t').collect();
        if t.len() < 7 || t[6] != "exact" {
            continue;
        }
        let n: usize = t[0].parse().unwrap();
        if n > max_n || n < 2 {
            continue;
        }
        let i: usize = t[1].parse().unwrap();
        let v: u8 = t[2].parse().unwrap();
        let p = crate::poset::Poset::from_above_hex(t[3]).expect("bad poset in table");
        let prob = Problem::new(p, i);
        let k = canon.key(&prob.reduced());
        if decode_key(k) != prob {
            noncanon += 1;
        }
        if let Some(old) = map.insert(k, v) {
            assert_eq!(old, v, "oracle table inconsistent");
        }
        rows += 1;
    }
    Ok((map, rows, noncanon))
}
