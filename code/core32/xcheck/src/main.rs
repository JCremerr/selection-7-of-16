//! cross-check of code/core32 (u32 sets, 192-bit keys) against the old
//! code/core (u16 sets, u128 keys) on n <= 16. Both crates are linked into this
//! binary (`old`, `new`); every problem is built in the old types and converted.
//!
//! usage:
//!   xcheck canon TRIALS SEED     random problems n <= 16 (random orders, search
//!                                paths, structured): canonical labeling, keys,
//!                                LB info, |C|, reduction, solved test, and
//!                                isomorphism decisions on pairs (relabelled copy =
//!                                isomorphic; perturbed copy = mostly not)
//!   xcheck roots                 LB info and root keys of all (E_n, i), n <= 16
//!   xcheck table PATH EVERY      every EVERY-th exact row of the ground-truth table
//!
//! Checked per problem P (old types) and its conversion P' (new types):
//!   * new canonical labeling == old canonical labeling (same lab array)
//!   * new key: relation bits == old key's bits 0..119, key_n / key_i equal,
//!     Key::fold() == old u128 key (so hashes are equal too)
//!   * lb_info (C, FG, FGdual, KEY) equal; compat_count equal
//!   * reduced() equal (after conversion), is_solved equal, twins equal
//! and per pair (P, Q): old_key(P) == old_key(Q)  <=>  new_key(P') == new_key(Q').
//! Prints one summary line per mode and "XCHECK OK" / "XCHECK FAIL".

use std::io::BufRead;

fn to_new(p: &old::poset::Poset) -> new::poset::Poset {
    let n = p.n();
    let ab: Vec<u32> = (0..n).map(|a| p.above[a] as u32).collect();
    new::poset::Poset::from_above(n, &ab).expect("conversion")
}

fn to_new_prob(q: &old::poset::Problem) -> new::poset::Problem {
    new::poset::Problem::new(to_new(&q.p), q.i as usize)
}

/// Negative control (XCHECK_NEGCTRL=1): the new side gets a perturbed copy of
/// every 1000th problem (one extra relation), so the checks must report errors.
fn negctrl() -> bool {
    std::env::var("XCHECK_NEGCTRL").map(|v| v == "1").unwrap_or(false)
}

struct Ctx {
    oc: old::canon::Canonizer,
    nc: new::canon::Canonizer,
    problems: u64,
    pairs_iso: u64,
    pairs_noniso: u64,
    errors: u64,
}

impl Ctx {
    fn new() -> Self {
        Ctx { oc: old::canon::Canonizer::new(), nc: new::canon::Canonizer::new(), problems: 0, pairs_iso: 0, pairs_noniso: 0, errors: 0 }
    }

    fn err(&mut self, what: &str, q: &old::poset::Problem) {
        self.errors += 1;
        if self.errors <= 20 {
            println!("MISMATCH {}: n={} i={} above={}", what, q.n(), q.i, q.p.above_hex());
        }
    }

    /// Single-problem checks; returns (old key, new key).
    fn check(&mut self, q: &old::poset::Problem) -> (u128, new::canon::Key) {
        self.problems += 1;
        let n = q.n();
        let mut qn = to_new_prob(q);
        if negctrl() && self.problems % 1000 == 0 {
            if let Some((a, b)) = (0..n).flat_map(|a| (0..n).map(move |b| (a, b))).find(|&(a, b)| a != b && !qn.p.comparable(a, b)) {
                qn = qn.with_less(a, b);
            }
        }
        // labeling of the poset itself
        let (oc, olab) = self.oc.canon_poset(&q.p);
        let (nc, nlab) = self.nc.canon_poset(&qn.p);
        if olab[..n] != nlab[..n] {
            self.err("canon_poset labeling", q);
        }
        if nc.low128() != oc || nc.0[0] != 0 {
            self.err("canon_poset certificate", q);
        }
        // problem keys (iso + duality)
        let ok = self.oc.key(q);
        let (nk, ndual, nl) = self.nc.key_lab(&qn);
        let (ok2, odual, ol) = self.oc.key_lab(q);
        assert_eq!(ok, ok2);
        if ndual != odual || nl[..n] != ol[..n] {
            self.err("key_lab (dual choice / labeling)", q);
        }
        let (f, e) = nk.fold();
        if f != ok || e != 0 {
            self.err("key fold", q);
        }
        if nk.low128() != ok & ((1u128 << 120) - 1)
            || new::canon::key_n(nk) != old::canon::key_n(ok)
            || new::canon::key_i(nk) != old::canon::key_i(ok)
        {
            self.err("key fields", q);
        }
        if new::bits::hash_key(nk) != old::bits::hash_key(ok) {
            self.err("hash", q);
        }
        // decode
        let od = old::canon::decode_key(ok);
        let nd = new::canon::decode_key(nk);
        if to_new_prob(&od) != nd {
            self.err("decode_key", q);
        }
        // LB info and counts
        let ol = old::lbbest::lb_info(q);
        let nl = new::lbbest::lb_info(&qn);
        if (ol.c, ol.fg, ol.fgd, ol.key) != (nl.c, nl.fg, nl.fgd, nl.key) || ol.lb() != nl.lb() {
            self.err("lb_info", q);
        }
        if n <= 14 && old::count::compat_count(q) != new::count::compat_count(&qn) {
            self.err("compat_count", q);
        }
        if to_new_prob(&q.reduced()) != qn.reduced() || q.is_solved() != qn.is_solved() || q.is_reduced() != qn.is_reduced() {
            self.err("reduction / solved", q);
        }
        let (ot, nt) = (q.p.twins(), qn.p.twins());
        if (0..n).any(|x| ot[x] as u32 != nt[x]) {
            self.err("twins", q);
        }
        (ok, nk)
    }

    /// Pair check: isomorphism decisions must agree.
    fn pair(&mut self, a: &old::poset::Problem, b: &old::poset::Problem, expect_iso: Option<bool>) {
        let (oa, na) = self.check(a);
        let (ob, nb) = self.check(b);
        let oeq = oa == ob;
        let neq = na == nb;
        if oeq != neq {
            self.err("isomorphism decision differs", a);
        }
        if let Some(e) = expect_iso {
            if e && !oeq {
                self.err("relabelled copy not isomorphic", a);
            }
        }
        if oeq {
            self.pairs_iso += 1;
        } else {
            self.pairs_noniso += 1;
        }
    }
}

fn random_problem(rng: &mut old::bits::Rng) -> old::poset::Problem {
    use old::poset::{Poset, Problem};
    let n = 1 + rng.below(16);
    let i = 1 + rng.below(n);
    match rng.below(4) {
        0 => {
            let dens = [0.0, 0.05, 0.1, 0.2, 0.3, 0.5, 0.8][rng.below(7)];
            Problem::new(Poset::random(n, dens, rng), i)
        }
        1 => {
            // structured: disjoint chains / tournaments-like blocks of random sizes
            let mut rels = Vec::new();
            let mut off = 0;
            while off < n {
                let m = (1 + rng.below(4)).min(n - off);
                for x in off..off + m - 1 {
                    rels.push((x, x + 1));
                }
                off += m;
            }
            let p = Poset::from_relations(n, &rels);
            let s = rng.perm(n);
            Problem::new(p.relabel(&s), i)
        }
        _ => {
            // search path from the antichain (what the solver sees), maybe reduced
            let mut prob = Problem::selection(n, i);
            let steps = rng.below(2 * n + 1);
            for _ in 0..steps {
                let m = prob.n();
                if m < 2 {
                    break;
                }
                let a = rng.below(m);
                let b = rng.below(m);
                if a == b || prob.p.comparable(a, b) {
                    continue;
                }
                prob = prob.with_less(a, b);
                if rng.below(2) == 0 {
                    prob = prob.reduced();
                }
            }
            prob
        }
    }
}

/// A perturbed copy: add one relation between an incomparable pair (or, if none,
/// change i). Usually not isomorphic; the decision is compared, not assumed.
fn perturb(q: &old::poset::Problem, rng: &mut old::bits::Rng) -> old::poset::Problem {
    let n = q.n();
    let mut pairs = Vec::new();
    for a in 0..n {
        for b in 0..n {
            if a != b && !q.p.comparable(a, b) {
                pairs.push((a, b));
            }
        }
    }
    if pairs.is_empty() {
        let i = 1 + rng.below(n);
        return old::poset::Problem::new(q.p, i);
    }
    let (a, b) = pairs[rng.below(pairs.len())];
    q.with_less(a, b)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("");
    let mut cx = Ctx::new();
    match mode {
        "canon" => {
            let trials: usize = args[2].parse().unwrap();
            let seed: u64 = args[3].parse().unwrap();
            let mut rng = old::bits::Rng::new(seed);
            for _ in 0..trials {
                let q = random_problem(&mut rng);
                let s = rng.perm(q.n());
                let r = q.relabel(&s);
                cx.pair(&q, &r, Some(true));
                let r2 = if rng.below(2) == 0 { r.dual() } else { r };
                cx.pair(&q, &r2, Some(true));
                let pq = perturb(&q, &mut rng);
                cx.pair(&q, &pq, None);
                // two independent random problems of the same (n, i) (rarely iso)
                let q2 = random_problem(&mut rng);
                cx.pair(&q, &q2, None);
            }
        }
        "roots" => {
            for n in 1..=16 {
                for i in 1..=n {
                    let q = old::poset::Problem::selection(n, i);
                    cx.check(&q);
                    cx.check(&q.reduced());
                }
            }
        }
        "table" => {
            let every: usize = args[3].parse().unwrap();
            let f = std::io::BufReader::new(std::fs::File::open(&args[2]).unwrap());
            for (idx, line) in f.lines().enumerate() {
                if idx % every != 0 {
                    continue;
                }
                let line = line.unwrap();
                let t: Vec<&str> = line.split('\t').collect();
                if t.len() < 7 || t[6] != "exact" {
                    continue;
                }
                let n: usize = t[0].parse().unwrap();
                let i: usize = t[1].parse().unwrap();
                let v: u32 = t[2].parse().unwrap();
                let c: u64 = t[4].parse().unwrap();
                let op = old::poset::Poset::from_above_hex(t[3]).unwrap();
                let np = new::poset::Poset::from_above_hex(t[3]).unwrap();
                assert_eq!(op.n(), n);
                let q = old::poset::Problem::new(op, i);
                if to_new(&op) != np {
                    cx.err("hex parse", &q);
                }
                cx.check(&q);
                let nl = new::lbbest::lb_info(&to_new_prob(&q));
                if nl.c != c || nl.lb() > v {
                    cx.err("table |C| / LB_best <= V", &q);
                }
            }
        }
        _ => {
            eprintln!("usage: xcheck canon TRIALS SEED | roots | table PATH EVERY");
            std::process::exit(2);
        }
    }
    println!(
        "xcheck {}: problems={} pairs_iso={} pairs_noniso={} errors={}",
        mode, cx.problems, cx.pairs_iso, cx.pairs_noniso, cx.errors
    );
    println!("{}", if cx.errors == 0 { "XCHECK OK" } else { "XCHECK FAIL" });
    std::process::exit(if cx.errors == 0 { 0 } else { 1 });
}
