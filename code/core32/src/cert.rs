//! Certificate extraction: turn a df-pn proof into a selection algorithm printed as
//! Rust in the PLAIN dialect accepted by code/verifier/verify_rs_algorithm.py.
//!
//! One function per ORIENTED problem up to isomorphism (iso-only canonical form;
//! the search table is keyed up to duality, but PLAIN has no is_dual flag, so a
//! problem and its dual get separate functions). Parameters = canonical positions.
//!   * n = 1: leaf `a`.
//!   * not reduced: pass-through (no comparison) to ONE round of deletions, exactly
//!     the verifier's rule R2 (drop e if |above(e)| >= n-i+1 -> below the answer,
//!     or |below(e)| >= i -> above the answer). Multi-round reductions become a
//!     chain of pass-through functions (cost 0).
//!   * reduced, n > 1: the proving move {x, y} from the search table (re-searched
//!     if it was evicted), then a call per outcome with knowledge P + x<y.
//! Function depth (worst case) is tracked; if a problem is needed at a smaller
//! budget than its existing function's depth, the function is regenerated at the
//! smaller budget (callers stay valid since depth only decreases). Only functions
//! reachable from the root are printed, callees first, root = select_0 last.

use crate::bits::bits;
use crate::canon::{Canonizer, Key};
use crate::dfpn::Dfpn;
use crate::poset::{Problem, MAXN};
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Clone)]
struct Call {
    f: usize,
    /// args[j] = caller parameter passed as the callee's parameter j
    args: Vec<u8>,
}

#[derive(Clone)]
enum Body {
    Pending,
    Leaf,
    Pass(Call),
    Cmp(u8, u8, Call, Call),
}

struct Func {
    n: u8,
    r: u8,
    depth: u8,
    /// cover relations (u, v) of the canonical poset, positions
    covers: Vec<(u8, u8)>,
    body: Body,
}

pub struct CertWriter<'a> {
    s: &'a mut Dfpn,
    canon: Canonizer,
    ids: HashMap<(Key, u8, u8), usize>,
    funcs: Vec<Func>,
    pub regenerated: usize,
    pub researched: usize,
}

/// Parameter names (core32: 19 names; the first 16 are those of code/core).
const NAMES: [&str; MAXN] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s"];

/// One round of deletions: (alive mask, new i). Elements with >= i below or
/// >= n-i+1 above (alive counts = all, since this is one round on the full set).
fn one_round(q: &Problem) -> (u32, u8) {
    let n = q.n() as u32;
    let i = q.i as u32;
    let mut alive = q.p.all();
    let mut lo = 0u32;
    for x in 0..q.n() {
        if q.p.below[x].count_ones() >= i {
            alive &= !(1 << x);
        } else if q.p.above[x].count_ones() >= n - i + 1 {
            alive &= !(1 << x);
            lo += 1;
        }
    }
    (alive, (i - lo) as u8)
}

impl<'a> CertWriter<'a> {
    pub fn new(s: &'a mut Dfpn) -> Self {
        CertWriter { s, canon: Canonizer::new(), ids: HashMap::new(), funcs: Vec::new(), regenerated: 0, researched: 0 }
    }

    /// Function for the oriented problem q (any labeling) with worst case <= b.
    /// Returns (function id, inv) with inv[j] = element of q that is parameter j.
    fn gen(&mut self, q: &Problem, b: u8) -> Result<(usize, [u8; MAXN]), String> {
        let n = q.n();
        let (cert, lab) = self.canon.canon_poset(&q.p);
        let mut inv = [0u8; MAXN];
        for v in 0..n {
            inv[lab[v] as usize] = v as u8;
        }
        let id = (cert, n as u8, q.i);
        let f = match self.ids.get(&id) {
            Some(&f) => {
                if !matches!(self.funcs[f].body, Body::Pending) && self.funcs[f].depth <= b {
                    return Ok((f, inv));
                }
                if matches!(self.funcs[f].body, Body::Pending) {
                    return Err("cycle in certificate generation (impossible)".into());
                }
                self.regenerated += 1;
                f
            }
            None => {
                let f = self.funcs.len();
                self.funcs.push(Func { n: n as u8, r: q.i - 1, depth: 0, covers: Vec::new(), body: Body::Pending });
                self.ids.insert(id, f);
                f
            }
        };
        let qc = q.relabel(&lab);
        // cover relations of the canonical poset
        let mut covers = Vec::new();
        for u in 0..n {
            for v in bits(qc.p.above[u]) {
                // u < v is a cover iff no w with u < w < v
                if qc.p.above[u] & qc.p.below[v] == 0 {
                    covers.push((u as u8, v as u8));
                }
            }
        }
        self.funcs[f].body = Body::Pending;
        let (body, depth) = if n == 1 {
            (Body::Leaf, 0u8)
        } else if !qc.is_reduced() {
            let (c, d) = self.call_for(&qc, b)?;
            (Body::Pass(c), d)
        } else {
            if qc.is_solved() || b == 0 {
                return Err(format!("unsolvable node at budget {} (n = {})", b, n));
            }
            let (key, _dual, lab2) = self.s.canon.key_lab(&qc);
            let mut inv2 = [0u8; MAXN];
            for v in 0..n {
                inv2[lab2[v] as usize] = v as u8;
            }
            if self.s.proof_move(key, b).is_none() {
                self.researched += 1;
            }
            let (u, v) = self.s.proof_of(key, b).ok_or_else(|| format!("no proof found for a node at budget {} (n = {})", b, n))?;
            let (x, y) = (inv2[u as usize] as usize, inv2[v as usize] as usize);
            if qc.p.comparable(x, y) {
                return Err("proof move compares comparable elements".into());
            }
            let (c1, d1) = self.call_for(&qc.with_less(x, y), b - 1)?;
            let (c2, d2) = self.call_for(&qc.with_less(y, x), b - 1)?;
            (Body::Cmp(x as u8, y as u8, c1, c2), 1 + d1.max(d2))
        };
        if depth > b {
            return Err(format!("generated depth {} > budget {}", depth, b));
        }
        let fu = &mut self.funcs[f];
        fu.body = body;
        fu.depth = depth;
        fu.covers = covers;
        Ok((f, inv))
    }

    /// Call from a branch whose knowledge is q (caller's parameter labels).
    fn call_for(&mut self, q: &Problem, b: u8) -> Result<(Call, u8), String> {
        let (alive, i2) = one_round(q);
        if alive == q.p.all() {
            let (g, ginv) = self.gen(q, b)?;
            let m = q.n();
            let args = (0..m).map(|j| ginv[j]).collect();
            Ok((Call { f: g, args }, self.funcs[g].depth))
        } else {
            let list: Vec<u8> = bits(alive).map(|x| x as u8).collect();
            let child = Problem { p: q.p.induced(alive), i: i2 };
            let (g, ginv) = self.gen(&child, b)?;
            let args = (0..list.len()).map(|j| list[ginv[j] as usize]).collect();
            Ok((Call { f: g, args }, self.funcs[g].depth))
        }
    }

    /// Build the algorithm for (E_n, i) with worst case <= k and print it.
    /// Returns (source, worst case, #functions).
    pub fn write(mut self, n: usize, i: usize, k: u8) -> Result<(String, u8, usize, usize, usize), String> {
        let root = Problem::selection(n, i);
        let (f0, _) = self.gen(&root, k)?;
        // post-order over reachable functions; root last
        let mut name: HashMap<usize, usize> = HashMap::new();
        let mut order: Vec<usize> = Vec::new();
        let mut stack: Vec<(usize, bool)> = vec![(f0, false)];
        let mut seen = vec![false; self.funcs.len()];
        while let Some((f, expanded)) = stack.pop() {
            if expanded {
                order.push(f);
                continue;
            }
            if seen[f] {
                continue;
            }
            seen[f] = true;
            stack.push((f, true));
            match &self.funcs[f].body {
                Body::Pass(c) => stack.push((c.f, false)),
                Body::Cmp(_, _, c1, c2) => {
                    stack.push((c2.f, false));
                    stack.push((c1.f, false));
                }
                _ => {}
            }
        }
        // root = select_0, others numbered in print order
        name.insert(f0, 0);
        let mut next = 1;
        for &f in &order {
            if f != f0 {
                name.insert(f, next);
                next += 1;
            }
        }
        let mut s = String::new();
        writeln!(s, "#[allow(unused)]\npub const N: usize = {};\n#[allow(unused)]\npub const I: usize = {};\n", n, i - 1).unwrap();
        writeln!(s, "#[allow(unused)]\npub fn run(input: [usize; {}]) -> usize {{\n    select_0(input)\n}}\n", n).unwrap();
        let call_str = |c: &Call, name: &HashMap<usize, usize>| -> String {
            let a: Vec<&str> = c.args.iter().map(|&x| NAMES[x as usize]).collect();
            format!("select_{}([{}])", name[&c.f], a.join(", "))
        };
        for &f in &order {
            let fu = &self.funcs[f];
            let m = fu.n as usize;
            let mut doc = format!("/// n = {}", m);
            if m > 1 {
                write!(doc, ", i = {}", fu.r).unwrap();
            }
            for &(u, v) in &fu.covers {
                write!(doc, ", {} < {}", NAMES[u as usize], NAMES[v as usize]).unwrap();
            }
            writeln!(s, "{}", doc).unwrap();
            let params: Vec<&str> = (0..m).map(|j| NAMES[j]).collect();
            writeln!(s, "fn select_{}([{}]: [usize; {}]) -> usize {{", name[&f], params.join(", "), m).unwrap();
            match &fu.body {
                Body::Leaf => writeln!(s, "    a").unwrap(),
                Body::Pass(c) => writeln!(s, "    {}", call_str(c, &name)).unwrap(),
                Body::Cmp(x, y, c1, c2) => {
                    writeln!(s, "    if {} < {} {{", NAMES[*x as usize], NAMES[*y as usize]).unwrap();
                    writeln!(s, "        {}", call_str(c1, &name)).unwrap();
                    writeln!(s, "    }} else {{").unwrap();
                    writeln!(s, "        {}", call_str(c2, &name)).unwrap();
                    writeln!(s, "    }}").unwrap();
                }
                Body::Pending => return Err("pending function in output".into()),
            }
            writeln!(s, "}}").unwrap();
        }
        let worst = self.funcs[f0].depth;
        let ncmp = order.iter().filter(|&&f| matches!(self.funcs[f].body, Body::Cmp(..))).count();
        Ok((s, worst, order.len(), ncmp, self.researched))
    }
}
