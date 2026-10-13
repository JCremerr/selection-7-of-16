//! Exact canonical forms of posets (n <= 19) and of selection problems up to
//! isomorphism and duality.  (core32 port: u32 sets, 192-bit `Key`.)
//!
//! Method: individualization-refinement (as in nauty/bliss), specialised to posets.
//!
//! * Colouring/refinement: ordered partitions of the elements. A cell S splits a
//!   cell C by the pair (|below(v) & S|, |above(v) & S|) for v in C; sub-cells are
//!   ordered by this key. Iterated to the coarsest equitable refinement. The
//!   initial refinement (splitter = all elements) orders elements by
//!   (|below|, |above|), so a < b always puts a in an earlier cell than b; hence
//!   every leaf labeling is a linear extension and the relation matrix is strictly
//!   upper triangular (171 bits for n = 19).
//! * Search tree: individualize each element of the first smallest non-singleton
//!   cell, refine, recurse. Each node carries a label-invariant trace hash of the
//!   refinement. Leaves are totally (pre)ordered by (trace sequence, certificate);
//!   the canonical form is the certificate of the least leaf. Correctness does not
//!   depend on hash collisions (traces are only used as an invariant order key).
//! * Pruning (all sound): (1) trace pruning against the best leaf; (2) twins
//!   (elements with identical above/below sets) - only one child per twin class;
//!   (3) automorphisms found from equal certificates: backjumping (only after
//!   explicitly verifying that the automorphism maps the current path prefix to
//!   the earlier path) and orbit pruning at first-path nodes.
//!
//! Certificate bit layout: for positions x < y, relation x<y is bit
//! y*(y-1)/2 + x (bits 0..170 for n <= 19; identical to code/core for n <= 16).
//! A problem key additionally stores (n-1) in bits 176..180 and (i-1) in bits
//! 184..188 (code/core: bits 120..123 and 124..127). The numeric order of keys is
//! therefore (i, n, certificate) exactly as in code/core, so every comparison of
//! keys or certificates (leaf order, dual choice, move sorting) decides the same
//! way as in code/core for n <= 16, and `Key::fold` reproduces code/core's u128
//! key for n <= 16 (used only for hashing).

use crate::bits::{bits, hash_fold, low_mask};
use crate::poset::{Poset, Problem, MAXN};
use std::cmp::Ordering;

/// 192-bit canonical key / certificate. `.0[0]` holds bits 128..191 (most
/// significant), `.0[1]` bits 64..127, `.0[2]` bits 0..63, so the derived
/// lexicographic order is the numeric order of the 192-bit integer.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Key(pub [u64; 3]);

const N_SHIFT: u32 = 176;
const I_SHIFT: u32 = 184;

impl Key {
    pub const ZERO: Key = Key([0; 3]);

    /// 64-bit word with bits 64*j .. 64*j+63 (j = 0, 1, 2).
    #[inline]
    fn word(&self, j: usize) -> u64 {
        self.0[2 - j]
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == [0; 3]
    }

    /// Set bit p (0 <= p < 192).
    #[inline]
    pub fn set_bit(&mut self, p: u32) {
        self.0[2 - (p >> 6) as usize] |= 1u64 << (p & 63);
    }

    /// Bits off .. off+len-1 (len <= 32) as an integer.
    #[inline]
    pub fn get_bits(&self, off: u32, len: u32) -> u64 {
        debug_assert!(len <= 32 && off + len <= 192);
        let j = (off >> 6) as usize;
        let sh = off & 63;
        let mut v = self.word(j) >> sh;
        if sh + len > 64 {
            v |= self.word(j + 1) << (64 - sh);
        }
        v & ((1u64 << len) - 1)
    }

    /// Low 128 bits.
    #[inline]
    pub fn low128(&self) -> u128 {
        self.0[2] as u128 | (self.0[1] as u128) << 64
    }

    /// (f, e): a 128-bit fold f and extra bits e of a problem key. For n <= 16
    /// (and i <= 16), e == 0 and f equals code/core's u128 key
    /// cert | (n-1) << 120 | (i-1) << 124. Used only for hashing (TT bucket, cache
    /// slots, jitter / tie-break noise); equality and order always use all 192 bits.
    #[inline]
    pub fn fold(&self) -> (u128, u64) {
        let n1 = (self.0[0] >> (N_SHIFT - 128)) & 31;
        let i1 = (self.0[0] >> (I_SHIFT - 128)) & 31;
        let f = self.low128() ^ ((n1 & 15) as u128) << 120 ^ ((i1 & 15) as u128) << 124;
        let rel_hi = self.0[0] & ((1u64 << 43) - 1); // relation bits 128..170
        let e = rel_hi | (n1 >> 4) << 43 | (i1 >> 4) << 44;
        (f, e)
    }

    /// Hash (equal to code/core's hash_key for n <= 16).
    #[inline]
    pub fn hash(&self) -> u64 {
        let (f, e) = self.fold();
        hash_fold(f, e)
    }
}

impl std::fmt::LowerHex for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}{:016x}{:016x}", self.0[0], self.0[1], self.0[2])
    }
}

const TRI: [u32; MAXN + 1] = {
    let mut t = [0u32; MAXN + 1];
    let mut q = 0;
    while q < MAXN + 1 {
        t[q] = (q * q.saturating_sub(1) / 2) as u32;
        q += 1;
    }
    t
};

#[derive(Clone, Copy)]
struct Part {
    /// cells[s] = mask of the cell starting at position s (valid only if bit s of `starts`).
    cells: [u32; MAXN],
    starts: u32,
}

#[inline]
fn hmix(h: u64, x: u64) -> u64 {
    (h.rotate_left(5) ^ x).wrapping_mul(0x51_7cc1_b727_220a_95)
}

/// Refine `part` to the coarsest equitable partition finer than it, using the cells
/// whose start positions are in `pending` as initial splitters. Returns a trace hash.
#[inline]
fn refine(p: &Poset, part: &mut Part, mut pending: u32, n: usize) -> u64 {
    let mut h: u64 = 0x2545_F491_4F6C_DD1D;
    let full = low_mask(n);
    while pending != 0 && part.starts != full {
        let s = pending.trailing_zeros() as usize;
        pending &= pending - 1;
        let sp = part.cells[s];
        let mut st = part.starts;
        while st != 0 {
            let t = st.trailing_zeros() as usize;
            st &= st - 1;
            let c = part.cells[t];
            if c & (c.wrapping_sub(1)) == 0 {
                continue;
            }
            let mut ks = [0u16; MAXN];
            let mut vs = [0u8; MAXN];
            let mut m = 0usize;
            for v in bits(c) {
                let key = (((p.below[v] & sp).count_ones() as u16) << 5) | (p.above[v] & sp).count_ones() as u16;
                // insertion sort by key
                let mut j = m;
                while j > 0 && ks[j - 1] > key {
                    ks[j] = ks[j - 1];
                    vs[j] = vs[j - 1];
                    j -= 1;
                }
                ks[j] = key;
                vs[j] = v as u8;
                m += 1;
            }
            if ks[0] == ks[m - 1] {
                continue;
            }
            let mut pos = t;
            let mut j = 0;
            while j < m {
                let key = ks[j];
                let start = pos;
                let mut mask = 0u32;
                while j < m && ks[j] == key {
                    mask |= 1 << vs[j];
                    j += 1;
                    pos += 1;
                }
                part.cells[start] = mask;
                part.starts |= 1 << start;
                pending |= 1 << start;
                h = hmix(h, (s as u64) | (t as u64) << 8 | (key as u64) << 16 | ((pos - start) as u64) << 32);
            }
        }
    }
    hmix(h, part.starts as u64)
}

/// Union-find over <= MAXN elements.
#[inline]
fn uf_find(par: &mut [u8; MAXN], mut x: usize) -> usize {
    while par[x] as usize != x {
        let g = par[par[x] as usize];
        par[x] = g;
        x = g as usize;
    }
    x
}

#[inline]
fn uf_union(par: &mut [u8; MAXN], a: usize, b: usize) {
    let ra = uf_find(par, a);
    let rb = uf_find(par, b);
    if ra != rb {
        if ra < rb {
            par[rb] = ra as u8;
        } else {
            par[ra] = rb as u8;
        }
    }
}

#[derive(Clone, Copy)]
struct Leaf {
    lab: [u8; MAXN], // vertex -> position
    inv: [u8; MAXN], // position -> vertex
    cert: Key,
    trace: [u64; MAXN + 1],
    path: [u8; MAXN],
    depth: usize,
}

impl Leaf {
    const EMPTY: Leaf = Leaf { lab: [0; MAXN], inv: [0; MAXN], cert: Key::ZERO, trace: [0; MAXN + 1], path: [0; MAXN], depth: 0 };
}

/// Statistics of the search trees (cumulative).
#[derive(Clone, Copy, Default, Debug)]
pub struct CanonStats {
    pub calls: u64,
    pub nodes: u64,
    pub leaves: u64,
    pub autos: u64,
}

/// Reusable canonical-labeling engine.
pub struct Canonizer {
    p: Poset,
    n: usize,
    tw: [u32; MAXN],
    have_first: bool,
    first: Leaf,
    best: Leaf,
    cur_trace: [u64; MAXN + 1],
    cur_path: [u8; MAXN],
    orbits: [[u8; MAXN]; MAXN + 1],
    pub stats: CanonStats,
}

impl Default for Canonizer {
    fn default() -> Self {
        Self::new()
    }
}

const NOJUMP: usize = usize::MAX;

impl Canonizer {
    pub fn new() -> Self {
        Canonizer {
            p: Poset::antichain(1),
            n: 1,
            tw: [0; MAXN],
            have_first: false,
            first: Leaf::EMPTY,
            best: Leaf::EMPTY,
            cur_trace: [0; MAXN + 1],
            cur_path: [0; MAXN],
            orbits: [[0; MAXN]; MAXN + 1],
            stats: CanonStats::default(),
        }
    }

    /// Canonical labeling of P. Returns (certificate, lab) where lab[v] is the
    /// canonical position of element v. The certificate determines P up to
    /// isomorphism: cert(P) == cert(Q) iff P and Q are isomorphic (and n equal).
    pub fn canon_poset(&mut self, p: &Poset) -> (Key, [u8; MAXN]) {
        self.stats.calls += 1;
        let n = p.n();
        self.p = *p;
        self.n = n;
        self.tw = p.twins();
        self.have_first = false;
        for d in 0..=n {
            for v in 0..MAXN {
                self.orbits[d][v] = v as u8;
            }
        }
        let mut part = Part { cells: [0; MAXN], starts: 1 };
        part.cells[0] = low_mask(n);
        let tr = refine(p, &mut part, 1, n);
        self.cur_trace[0] = tr;
        self.dfs(&part, 0);
        (self.best.cert, self.best.lab)
    }

    fn dfs(&mut self, part: &Part, depth: usize) -> usize {
        self.stats.nodes += 1;
        let n = self.n;
        if part.starts == low_mask(n) {
            return self.leaf(part, depth);
        }
        // target cell: first smallest non-singleton cell
        let mut t = 0usize;
        let mut best_size = 99u32;
        for s in bits(part.starts) {
            let sz = part.cells[s].count_ones();
            if sz > 1 && sz < best_size {
                best_size = sz;
                t = s;
                if sz == 2 {
                    break;
                }
            }
        }
        let cell = part.cells[t];
        let mut explored = 0u32;
        for v in bits(cell) {
            // twin pruning: only the smallest twin (within this cell) is a child
            if self.tw[v] & cell & low_mask(v) != 0 {
                continue;
            }
            // orbit pruning at nodes of the first path
            if explored != 0 && self.have_first && depth < self.first.depth && self.cur_path[..depth] == self.first.path[..depth] {
                let orb = &mut self.orbits[depth];
                let r = uf_find(orb, v);
                let mut dup = false;
                for u in bits(explored) {
                    if uf_find(orb, u) == r {
                        dup = true;
                        break;
                    }
                }
                if dup {
                    continue;
                }
            }
            explored |= 1 << v;
            let mut child = *part;
            child.cells[t] = 1 << v;
            child.cells[t + 1] = cell & !(1 << v);
            child.starts |= 1 << (t + 1);
            let tr = refine(&self.p, &mut child, 1 << t, n);
            self.cur_trace[depth + 1] = tr;
            self.cur_path[depth] = v as u8;
            if self.have_first && self.prefix_worse(depth + 1) {
                continue;
            }
            let r = self.dfs(&child, depth + 1);
            if r != NOJUMP && r < depth {
                return r;
            }
        }
        NOJUMP
    }

    /// Is every leaf below the current node (trace prefix 0..=d, node not a leaf)
    /// strictly worse than the best leaf?
    #[inline]
    fn prefix_worse(&self, d: usize) -> bool {
        let bd = self.best.depth;
        let m = d.min(bd);
        for j in 0..=m {
            match self.cur_trace[j].cmp(&self.best.trace[j]) {
                Ordering::Less => return false,
                Ordering::Greater => return true,
                Ordering::Equal => {}
            }
        }
        // equal on the common part. If best's sequence ended (bd <= d), it is a prefix
        // of every leaf sequence below this node (which are strictly longer than d+1
        // unless the node itself is a leaf, handled in `leaf`) -> best is smaller.
        bd < d
    }

    fn leaf(&mut self, part: &Part, depth: usize) -> usize {
        self.stats.leaves += 1;
        let n = self.n;
        let mut lab = [0u8; MAXN];
        let mut inv = [0u8; MAXN];
        for s in bits(part.starts) {
            let v = part.cells[s].trailing_zeros() as usize;
            lab[v] = s as u8;
            inv[s] = v as u8;
        }
        for v in n..MAXN {
            lab[v] = v as u8;
            inv[v] = v as u8;
        }
        let cert = cert_of(&self.p, &lab);
        let cur = Leaf { lab, inv, cert, trace: self.cur_trace, path: self.cur_path, depth };
        if !self.have_first {
            self.have_first = true;
            self.first = cur;
            self.best = cur;
            return NOJUMP;
        }
        let ord = self.cmp_leaf(&cur);
        if ord == Ordering::Less {
            self.best = cur;
        }
        if cert == self.first.cert {
            // same relabeled matrix => automorphism
            let g = compose_auto(&lab, &self.first.inv, n);
            self.record_auto(&g);
            let other = self.first;
            return self.jump_level(&g, &other, depth);
        }
        if ord == Ordering::Equal {
            let g = compose_auto(&lab, &self.best.inv, n);
            self.record_auto(&g);
            let other = self.best;
            return self.jump_level(&g, &other, depth);
        }
        NOJUMP
    }

    /// Compare current leaf with the best leaf: (trace sequence, then certificate).
    fn cmp_leaf(&self, cur: &Leaf) -> Ordering {
        let b = &self.best;
        let m = cur.depth.min(b.depth);
        for j in 0..=m {
            match cur.trace[j].cmp(&b.trace[j]) {
                Ordering::Equal => {}
                o => return o,
            }
        }
        match cur.depth.cmp(&b.depth) {
            Ordering::Equal => cur.cert.cmp(&b.cert),
            o => o,
        }
    }

    /// Union the cycles of automorphism g into the orbit partitions of all
    /// first-path levels whose prefix g fixes pointwise.
    fn record_auto(&mut self, g: &[u8; MAXN]) {
        self.stats.autos += 1;
        let n = self.n;
        for d in 0..self.first.depth {
            if d > 0 {
                let f = self.first.path[d - 1] as usize;
                if g[f] as usize != f {
                    break;
                }
            }
            let orb = &mut self.orbits[d];
            for v in 0..n {
                let w = g[v] as usize;
                if w != v {
                    uf_union(orb, v, w);
                }
            }
        }
    }

    /// Given automorphism g mapping the current leaf onto `other` (an earlier leaf),
    /// return the level to backjump to, after verifying that g fixes the common path
    /// prefix and maps the diverging child onto other's (fully explored) child.
    fn jump_level(&self, g: &[u8; MAXN], other: &Leaf, depth: usize) -> usize {
        let mut l = 0;
        while l < depth && l < other.depth && self.cur_path[l] == other.path[l] {
            l += 1;
        }
        if l >= depth || l >= other.depth {
            return NOJUMP;
        }
        for j in 0..l {
            let w = self.cur_path[j] as usize;
            if g[w] as usize != w {
                return NOJUMP;
            }
        }
        if g[self.cur_path[l] as usize] != other.path[l] {
            return NOJUMP;
        }
        l
    }
}

/// g = other_inv o lab : maps v to the vertex at v's position in the other leaf.
#[inline]
fn compose_auto(lab: &[u8; MAXN], other_inv: &[u8; MAXN], n: usize) -> [u8; MAXN] {
    let mut g = [0u8; MAXN];
    for v in 0..MAXN {
        g[v] = if v < n { other_inv[lab[v] as usize] } else { v as u8 };
    }
    g
}

/// Relation matrix of P under labeling `lab` (vertex -> position) as a strictly
/// upper-triangular bit matrix. Requires lab to be a linear extension.
#[inline]
pub fn cert_of(p: &Poset, lab: &[u8; MAXN]) -> Key {
    let mut c = Key::ZERO;
    for v in 0..p.n() {
        let pv = lab[v] as u32;
        for u in bits(p.above[v]) {
            let q = lab[u] as u32;
            debug_assert!(q > pv);
            c.set_bit(TRI[q as usize] + pv);
        }
    }
    c
}

/// Decode a certificate into the poset on positions 0..n-1.
pub fn decode_cert(cert: Key, n: usize) -> Poset {
    let mut p = Poset::antichain(n);
    for q in 1..n {
        let col = cert.get_bits(TRI[q], q as u32) as u32;
        p.below[q] = col;
        for x in bits(col) {
            p.above[x] |= 1 << q;
        }
    }
    p
}

#[inline]
pub fn make_key(cert: Key, n: usize, i: usize) -> Key {
    debug_assert!(n >= 1 && n <= MAXN && i >= 1 && i <= MAXN);
    debug_assert!(cert.0[0] >> (N_SHIFT - 128) == 0);
    let mut k = cert;
    k.0[0] |= ((n as u64 - 1) << (N_SHIFT - 128)) | ((i as u64 - 1) << (I_SHIFT - 128));
    k
}

#[inline]
pub fn key_n(k: Key) -> usize {
    ((k.0[0] >> (N_SHIFT - 128)) as usize & 31) + 1
}

#[inline]
pub fn key_i(k: Key) -> usize {
    ((k.0[0] >> (I_SHIFT - 128)) as usize & 31) + 1
}

/// Decode a problem key into the canonical problem.
pub fn decode_key(k: Key) -> Problem {
    let n = key_n(k);
    let mut cert = k;
    cert.0[0] &= (1u64 << (N_SHIFT - 128)) - 1;
    Problem { p: decode_cert(cert, n), i: key_i(k) as u8 }
}

impl Canonizer {
    /// Canonical key of the problem (P, i) up to isomorphism AND duality.
    /// Representative choice: the version with the smaller i (i <= n+1-i); if
    /// i == n+1-i, the smaller of the two keys. The problem is not reduced here.
    pub fn key(&mut self, prob: &Problem) -> Key {
        self.key_lab(prob).0
    }

    /// As `key`, also returning (dual_used, lab): the canonical problem is
    /// (P or P^dual) relabeled by lab.
    pub fn key_lab(&mut self, prob: &Problem) -> (Key, bool, [u8; MAXN]) {
        let n = prob.n();
        let i = prob.i as usize;
        let j = n + 1 - i;
        if i < j {
            let (c, lab) = self.canon_poset(&prob.p);
            (make_key(c, n, i), false, lab)
        } else if i > j {
            let (c, lab) = self.canon_poset(&prob.p.dual());
            (make_key(c, n, j), true, lab)
        } else {
            let (c1, lab1) = self.canon_poset(&prob.p);
            let (c2, lab2) = self.canon_poset(&prob.p.dual());
            if c1 <= c2 {
                (make_key(c1, n, i), false, lab1)
            } else {
                (make_key(c2, n, i), true, lab2)
            }
        }
    }

    /// Canonical key and the canonical problem itself.
    #[inline]
    pub fn canon_problem(&mut self, prob: &Problem) -> (Key, Problem) {
        let k = self.key(prob);
        (k, decode_key(k))
    }
}
