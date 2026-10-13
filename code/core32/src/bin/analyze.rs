//! Gap analysis of a ground-truth table (data/groundtruth_table.tsv).
//!
//! usage: analyze <table.tsv> [--naive=K] [--bellman]
//! Also re-verifies every row: the poset parses, is transitively closed, reduced,
//! canonical (its key re-canonicalizes to itself) and |C| / ceil(log2|C|) match.
//! --naive=K : recompute V for all exact rows with n <= K by a naive minimax on
//!             labeled, unreduced posets (no canonical forms, no reduction).
//! --bellman : local consistency of the whole table: for every exact row and every
//!             comparison whose two (reduced, canonical) children are exact rows,
//!             V <= 1 + max(V children); count rows with a witness move achieving equality.

use selcore::canon::{make_key, Canonizer};
use selcore::count::{ceil_log2, compat_count};
use selcore::poset::{Poset, Problem, MAXN};
use std::collections::{BTreeMap, HashMap};
use std::io::BufRead;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().expect("usage: analyze <table.tsv> [--naive=K] [--bellman]");
    let naive_k: usize = args.iter().find_map(|a| a.strip_prefix("--naive=")).and_then(|s| s.parse().ok()).unwrap_or(0);
    let bellman = args.iter().any(|a| a == "--bellman");
    let mut table: HashMap<selcore::Key, u8> = HashMap::new();
    let mut naive_memo: HashMap<([u32; MAXN], u8, u8), u8> = HashMap::new();
    let (mut naive_checked, mut naive_bad) = (0usize, 0usize);
    let f = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
    let mut cz = Canonizer::new();
    let mut rows = 0usize;
    let mut bad = 0usize;
    let mut per_n: BTreeMap<usize, (usize, usize, usize)> = BTreeMap::new(); // rows, exact, phase1
    let mut gap_n: BTreeMap<(usize, i64), usize> = BTreeMap::new();
    let mut gap_v: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    let mut by_lb: BTreeMap<i64, (usize, f64, i64, i64)> = BTreeMap::new(); // count, sumV, minV, maxV
    let (mut sx, mut sy, mut sxx, mut sxy, mut cnt) = (0f64, 0f64, 0f64, 0f64, 0f64);
    let mut ratio_n: BTreeMap<usize, (f64, usize)> = BTreeMap::new();
    for line in f.lines() {
        let line = line.unwrap();
        let c: Vec<&str> = line.split('\t').collect();
        rows += 1;
        let n: usize = c[0].parse().unwrap();
        let i: usize = c[1].parse().unwrap();
        let v: i64 = c[2].parse().unwrap();
        let p = Poset::from_above_hex(c[3]);
        let cc: u64 = c[4].parse().unwrap();
        let lb: i64 = c[5].parse().unwrap();
        let exact = c.get(6).map_or(true, |s| *s == "exact");
        let phase1 = c.get(7).map_or(true, |s| *s == "1");
        let e = per_n.entry(n).or_default();
        e.0 += 1;
        e.1 += exact as usize;
        e.2 += phase1 as usize;
        // re-verify the row
        match p {
            None => bad += 1,
            Some(p) => {
                let prob = Problem::new(p, i);
                let canon_ok = prob.is_reduced() && cz.key(&prob) == make_key(selcore::canon::cert_of(&p, &ident()), n, i);
                if p.n() != n || !canon_ok || compat_count(&prob) != cc || ceil_log2(cc) as i64 != lb {
                    bad += 1;
                }
            }
        }
        if !exact {
            continue;
        }
        if let Some(p) = Poset::from_above_hex(c[3]) {
            let prob = Problem::new(p, i);
            if bellman {
                table.insert(cz.key(&prob), v as u8);
            }
            if n <= naive_k {
                naive_checked += 1;
                if naive(&prob, &mut naive_memo) as i64 != v {
                    naive_bad += 1;
                }
            }
        }
        let g = v - lb;
        *gap_n.entry((n, g)).or_default() += 1;
        *gap_v.entry((v, g)).or_default() += 1;
        let b = by_lb.entry(lb).or_insert((0, 0.0, i64::MAX, i64::MIN));
        b.0 += 1;
        b.1 += v as f64;
        b.2 = b.2.min(v);
        b.3 = b.3.max(v);
        if lb > 0 {
            let r = ratio_n.entry(n).or_insert((0.0, 0));
            r.0 += v as f64 / lb as f64;
            r.1 += 1;
        }
        let (x, y) = (lb as f64, v as f64);
        sx += x;
        sy += y;
        sxx += x * x;
        sxy += x * y;
        cnt += 1.0;
    }
    println!("table {}: {} rows, re-verification errors: {}", path, rows, bad);
    if naive_k > 0 {
        println!("naive minimax check (n <= {}): {} rows checked, {} mismatches", naive_k, naive_checked, naive_bad);
    }
    if bellman {
        let (mut viol, mut witnessed, mut moves_checked) = (0usize, 0usize, 0usize);
        for (&k, &v) in &table {
            if v == 0 {
                witnessed += 1;
                continue;
            }
            let prob = selcore::decode_key(k);
            let m = prob.n();
            let mut wit = false;
            for a in 0..m {
                for b in a + 1..m {
                    if prob.p.comparable(a, b) {
                        continue;
                    }
                    let k1 = cz.key(&prob.child(a, b));
                    let k2 = cz.key(&prob.child(b, a));
                    let v1 = if selcore::canon::key_n(k1) == 1 { Some(0) } else { table.get(&k1).copied() };
                    let v2 = if selcore::canon::key_n(k2) == 1 { Some(0) } else { table.get(&k2).copied() };
                    if let (Some(v1), Some(v2)) = (v1, v2) {
                        moves_checked += 1;
                        let w = 1 + v1.max(v2);
                        if v > w {
                            viol += 1;
                        }
                        if v == w {
                            wit = true;
                        }
                    }
                }
            }
            witnessed += wit as usize;
        }
        println!(
            "bellman check: {} exact rows, {} moves with both children in table, violations (V > 1+max children) = {}, rows with an optimal witness move in table = {}",
            table.len(),
            moves_checked,
            viol,
            witnessed
        );
    }
    println!("\nper n: rows / exact / expanded-in-phase-1");
    for (n, (r, e, p1)) in &per_n {
        println!("  n={:2}: {:9} {:9} {:9}", n, r, e, p1);
    }
    println!("\ngap g = V - ceil(log2|C|) by n (exact rows): g:count  [mean g]");
    for n in per_n.keys() {
        let items: Vec<(i64, usize)> = gap_n.iter().filter(|((m, _), _)| m == n).map(|((_, g), c)| (*g, *c)).collect();
        let tot: usize = items.iter().map(|x| x.1).sum();
        let mean = items.iter().map(|(g, c)| *g as f64 * *c as f64).sum::<f64>() / tot.max(1) as f64;
        let s: Vec<String> = items.iter().map(|(g, c)| format!("{}:{}", g, c)).collect();
        println!("  n={:2}: {}  [{:.2}]", n, s.join(" "), mean);
    }
    println!("\ngap by V (exact rows): g:count  [mean g]");
    let vs: std::collections::BTreeSet<i64> = gap_v.keys().map(|(v, _)| *v).collect();
    for v in vs {
        let items: Vec<(i64, usize)> = gap_v.iter().filter(|((w, _), _)| *w == v).map(|((_, g), c)| (*g, *c)).collect();
        let tot: usize = items.iter().map(|x| x.1).sum();
        let mean = items.iter().map(|(g, c)| *g as f64 * *c as f64).sum::<f64>() / tot.max(1) as f64;
        let s: Vec<String> = items.iter().map(|(g, c)| format!("{}:{}", g, c)).collect();
        println!("  V={:2}: {}  [{:.2}]", v, s.join(" "), mean);
    }
    println!("\nV given LB = ceil(log2|C|): count, mean V, min V, max V, mean V/LB");
    for (lb, (c, s, mn, mx)) in &by_lb {
        println!("  LB={:2}: {:9}  meanV {:6.2}  min {:2}  max {:2}  V/LB {:.3}", lb, c, s / *c as f64, mn, mx, if *lb > 0 { s / *c as f64 / *lb as f64 } else { f64::NAN });
    }
    let a = (cnt * sxy - sx * sy) / (cnt * sxx - sx * sx);
    let b = (sy - a * sx) / cnt;
    println!("\nleast squares over exact rows: V ~= {:.3} * LB + {:.3}   (n = {})", a, b, cnt);
    println!("mean V/LB by n (LB > 0):");
    for (n, (s, c)) in &ratio_n {
        println!("  n={:2}: {:.3}", n, s / *c as f64);
    }
}

fn ident() -> [u8; MAXN] {
    let mut l = [0u8; MAXN];
    for (k, x) in l.iter_mut().enumerate() {
        *x = k as u8;
    }
    l
}

/// Naive minimax on labeled, unreduced posets (independent of canon/reduction).
fn naive(prob: &Problem, memo: &mut HashMap<([u32; MAXN], u8, u8), u8>) -> u8 {
    if prob.is_solved() {
        return 0;
    }
    let key = (prob.p.above, prob.i, prob.p.n);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let n = prob.n();
    let mut best = u8::MAX;
    for a in 0..n {
        for b in a + 1..n {
            if prob.p.comparable(a, b) {
                continue;
            }
            let v1 = naive(&prob.with_less(a, b), memo);
            if v1 + 1 >= best {
                continue;
            }
            let v2 = naive(&prob.with_less(b, a), memo);
            best = best.min(1 + v1.max(v2));
        }
    }
    memo.insert(key, best);
    best
}
