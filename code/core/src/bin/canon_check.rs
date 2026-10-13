//! Exactness tests and throughput benchmark for canonical forms.
//!
//! usage: canon_check exhaustive <nmax>   all naturally labeled posets n <= nmax:
//!                                         #distinct certs == #unlabeled posets (A000112),
//!                                         invariance under random relabeling, decode check,
//!                                         brute-force canonical form agreement (n <= 6/7)
//!        canon_check problems <nmax>     (P,i) keys vs brute force iso+duality classes
//!        canon_check random <trials>     random/structured posets n <= 16: invariance
//!        canon_check bench               canonicalizations/s on search-like problems

use selcore::bits::{bits, low_mask, Rng};
use selcore::canon::{cert_of, decode_cert, make_key, Canonizer};
use selcore::poset::{Poset, Problem, MAXN};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

/// All naturally labeled posets on n elements (a<b implies a<b as integers).
fn natural_posets(n: usize) -> Vec<Poset> {
    let mut out = Vec::new();
    let p = Poset::antichain(1);
    if n == 0 {
        return out;
    }
    rec_nat(&p, 1, n, &mut out);
    out
}

fn rec_nat(p: &Poset, k: usize, n: usize, out: &mut Vec<Poset>) {
    if k == n {
        out.push(*p);
        return;
    }
    // choose below set of new element k: a down-set of p
    for d in 0u32..(1u32 << k) {
        let d = d as u16;
        let mut ok = true;
        for x in bits(d) {
            if p.below[x] & !d != 0 {
                ok = false;
                break;
            }
        }
        if !ok {
            continue;
        }
        let mut q = *p;
        q.n = (k + 1) as u8;
        q.below[k] = d;
        q.above[k] = 0;
        for x in bits(d) {
            q.above[x] |= 1 << k;
        }
        rec_nat(&q, k + 1, n, out);
    }
}

/// Full adjacency matrix encoding (n*n bits) under a permutation.
fn full_code(p: &Poset, perm: &[u8]) -> u64 {
    let n = p.n();
    let mut c = 0u64;
    for x in 0..n {
        for y in bits(p.above[x]) {
            c |= 1u64 << (perm[x] as usize * n + perm[y] as usize);
        }
    }
    c
}

fn all_perms(n: usize) -> Vec<[u8; MAXN]> {
    let mut res = Vec::new();
    let mut a: Vec<u8> = (0..n as u8).collect();
    fn heap(k: usize, a: &mut Vec<u8>, res: &mut Vec<[u8; MAXN]>) {
        if k <= 1 {
            let mut p = [0u8; MAXN];
            for (i, x) in p.iter_mut().enumerate() {
                *x = i as u8;
            }
            p[..a.len()].copy_from_slice(a);
            res.push(p);
            return;
        }
        heap(k - 1, a, res);
        for i in 0..k - 1 {
            if k % 2 == 0 {
                a.swap(i, k - 1);
            } else {
                a.swap(0, k - 1);
            }
            heap(k - 1, a, res);
        }
    }
    heap(n, &mut a, &mut res);
    res
}

/// Brute-force canonical code: min over all permutations.
fn brute_canon(p: &Poset, perms: &[[u8; MAXN]]) -> u64 {
    perms.iter().map(|s| full_code(p, s)).min().unwrap()
}

const A000112: [usize; 10] = [1, 1, 2, 5, 16, 63, 318, 2045, 16999, 183231];

fn exhaustive(nmax: usize) {
    let mut cz = Canonizer::new();
    let mut rng = Rng::new(42);
    let mut all_ok = true;
    for n in 1..=nmax {
        let t0 = Instant::now();
        let ps = natural_posets(n);
        let perms = if n <= 7 { all_perms(n) } else { Vec::new() };
        let mut certs: HashSet<u128> = HashSet::new();
        let mut brute_map: HashMap<u64, u128> = HashMap::new();
        let mut cert_map: HashMap<u128, u64> = HashMap::new();
        let mut bad = 0usize;
        for p in &ps {
            let (c, lab) = cz.canon_poset(p);
            certs.insert(c);
            // decode check: relabeling p by lab gives exactly decode(c)
            let q = p.relabel(&lab);
            if q != decode_cert(c, n) || cert_of(p, &lab) != c {
                bad += 1;
            }
            // invariance under 3 random relabelings
            for _ in 0..3 {
                let s = rng.perm(n);
                let (c2, _) = cz.canon_poset(&p.relabel(&s));
                if c2 != c {
                    bad += 1;
                }
            }
            // brute force canonical form agreement (bijection between classes)
            if n <= 6 || (n == 7 && rng.chance(0.02)) {
                let b = brute_canon(p, &perms);
                if let Some(&c0) = brute_map.get(&b) {
                    if c0 != c {
                        bad += 1;
                    }
                } else {
                    brute_map.insert(b, c);
                }
                if let Some(&b0) = cert_map.get(&c) {
                    if b0 != b {
                        bad += 1;
                    }
                } else {
                    cert_map.insert(c, b);
                }
            }
        }
        let expect = A000112[n];
        let ok = certs.len() == expect && bad == 0;
        all_ok &= ok;
        println!(
            "n={} labeled-natural={} distinct-certs={} expected(A000112)={} brute-classes-checked={} errors={} {:.2}s {}",
            n,
            ps.len(),
            certs.len(),
            expect,
            brute_map.len(),
            bad,
            t0.elapsed().as_secs_f64(),
            if ok { "OK" } else { "FAIL" }
        );
    }
    println!("{}", if all_ok { "EXHAUSTIVE: ALL OK" } else { "EXHAUSTIVE: FAILURES" });
}

/// Problems (P,i): keys must induce exactly the iso+duality classes (brute force).
fn problems(nmax: usize) {
    let mut cz = Canonizer::new();
    let mut rng = Rng::new(7);
    let mut all_ok = true;
    for n in 1..=nmax {
        let ps = natural_posets(n);
        let perms = all_perms(n);
        let mut key_to_brute: HashMap<u128, (u64, usize)> = HashMap::new();
        let mut brute_to_key: HashMap<(u64, usize), u128> = HashMap::new();
        let mut bad = 0usize;
        let mut reduced_classes: HashSet<u128> = HashSet::new();
        for p in &ps {
            for i in 1..=n {
                let prob = Problem::new(*p, i);
                let k = cz.key(&prob);
                // invariance: random relabeling and dual
                let s = rng.perm(n);
                if cz.key(&prob.relabel(&s)) != k || cz.key(&prob.dual()) != k || cz.key(&prob.dual().relabel(&s)) != k {
                    bad += 1;
                }
                // decode gives an isomorphic copy of P or its dual with the right i
                let (k2, dual, lab) = cz.key_lab(&prob);
                assert_eq!(k, k2);
                let base = if dual { prob.dual() } else { prob };
                let dk = selcore::canon::decode_key(k);
                if base.p.relabel(&lab) != dk.p || base.i != dk.i {
                    bad += 1;
                }
                // brute-force class of (P,i) up to iso and duality
                let b1 = (brute_canon(p, &perms), i);
                let b2 = (brute_canon(&p.dual(), &perms), n + 1 - i);
                let b = b1.min(b2);
                if let Some(&x) = key_to_brute.get(&k) {
                    if x != b {
                        bad += 1;
                    }
                } else {
                    key_to_brute.insert(k, b);
                }
                if let Some(&x) = brute_to_key.get(&b) {
                    if x != k {
                        bad += 1;
                    }
                } else {
                    brute_to_key.insert(b, k);
                }
                if prob.is_reduced() {
                    reduced_classes.insert(k);
                }
            }
        }
        let ok = bad == 0 && key_to_brute.len() == brute_to_key.len();
        all_ok &= ok;
        println!(
            "n={} classes(P,i) keys={} brute={} reduced-classes={} errors={} {}",
            n,
            key_to_brute.len(),
            brute_to_key.len(),
            reduced_classes.len(),
            bad,
            if ok { "OK" } else { "FAIL" }
        );
    }
    println!("{}", if all_ok { "PROBLEMS: ALL OK" } else { "PROBLEMS: FAILURES" });
}

/// Structured posets with large automorphism groups.
fn structured(n: usize, kind: usize, rng: &mut Rng) -> Poset {
    let mut rels = Vec::new();
    match kind {
        0 => {} // antichain
        1 => {
            // disjoint 2-chains (+ isolated)
            for a in (0..n - 1).step_by(2) {
                rels.push((a, a + 1));
            }
        }
        2 => {
            // standard example S_k: a_i < b_j iff i != j
            let k = n / 2;
            for x in 0..k {
                for y in 0..k {
                    if x != y {
                        rels.push((x, k + y));
                    }
                }
            }
        }
        3 => {
            // crown / cycle-like fence of length n (even)
            let k = n / 2;
            for x in 0..k {
                rels.push((x, k + x));
                rels.push((x, k + (x + 1) % k));
            }
        }
        4 => {
            // disjoint 3-chains
            for a in (0..n.saturating_sub(2)).step_by(3) {
                rels.push((a, a + 1));
                rels.push((a + 1, a + 2));
            }
        }
        5 => {
            // complete bipartite K_{k,n-k}
            let k = n / 2;
            for x in 0..k {
                for y in k..n {
                    rels.push((x, y));
                }
            }
        }
        6 => {
            // spiders: one root below legs of length 2
            for l in (1..n.saturating_sub(1)).step_by(2) {
                rels.push((0, l));
                rels.push((l, l + 1));
            }
        }
        7 => {
            // product of 2 chains / grid a x b
            let a = 4.min(n);
            let b = n / a;
            for x in 0..a {
                for y in 0..b {
                    let v = x * b + y;
                    if x + 1 < a {
                        rels.push((v, v + b));
                    }
                    if y + 1 < b {
                        rels.push((v, v + 1));
                    }
                }
            }
        }
        8 => {
            // boolean lattice B_4 (n >= 16) or B_3 + extras
            let d = if n >= 16 { 4 } else if n >= 8 { 3 } else { 2 };
            for x in 0..(1usize << d) {
                for y in 0..(1usize << d) {
                    if x != y && x & y == x {
                        rels.push((x, y));
                    }
                }
            }
        }
        _ => {
            // disjoint copies of a random small poset
            let m = 3 + rng.below(3);
            let base = Poset::random(m, 0.5, rng);
            let mut off = 0;
            while off + m <= n {
                for x in 0..m {
                    for y in bits(base.above[x]) {
                        rels.push((off + x, off + y));
                    }
                }
                off += m;
            }
        }
    }
    let p = Poset::from_relations(n, &rels);
    let s = rng.perm(n);
    p.relabel(&s)
}

fn random(trials: usize) {
    let mut cz = Canonizer::new();
    let mut rng = Rng::new(2024);
    let mut bad = 0;
    let mut count = 0;
    let t0 = Instant::now();
    for t in 0..trials {
        let n = 1 + rng.below(16);
        let p = if t % 3 == 0 {
            let kind = rng.below(10);
            structured(n.max(4), kind, &mut rng)
        } else {
            let dens = [0.05, 0.1, 0.2, 0.3, 0.5, 0.8][rng.below(6)];
            Poset::random(n, dens, &mut rng)
        };
        let n = p.n();
        assert!(p.is_valid());
        let i = 1 + rng.below(n);
        let prob = Problem::new(p, i);
        let k = cz.key(&prob);
        for _ in 0..4 {
            let s = rng.perm(n);
            count += 1;
            if cz.key(&prob.relabel(&s)) != k {
                bad += 1;
            }
            if cz.key(&prob.dual().relabel(&s)) != k {
                bad += 1;
            }
        }
        let (c, lab) = cz.canon_poset(&p);
        if p.relabel(&lab) != decode_cert(c, n) {
            bad += 1;
        }
        let _ = make_key(c, n, 1);
    }
    println!(
        "random/structured: {} posets, {} relabel checks, errors={} ({:.2}s) stats={:?} {}",
        trials,
        count,
        bad,
        t0.elapsed().as_secs_f64(),
        cz.stats,
        if bad == 0 { "OK" } else { "FAIL" }
    );
}

/// Problems as seen by the search: random comparison sequences from E_n, reduced.
fn search_like(n: usize, i: usize, steps: usize, rng: &mut Rng) -> Problem {
    let mut prob = Problem::selection(n, i);
    for _ in 0..steps {
        let m = prob.n();
        if m == 1 {
            break;
        }
        let mut pairs = Vec::new();
        for a in 0..m {
            for b in a + 1..m {
                if !prob.p.comparable(a, b) {
                    pairs.push((a, b));
                }
            }
        }
        let (a, b) = pairs[rng.below(pairs.len())];
        prob = if rng.chance(0.5) { prob.child(a, b) } else { prob.child(b, a) };
    }
    prob
}

fn bench() {
    let mut rng = Rng::new(99);
    for &n in &[10usize, 12, 14, 16] {
        let mut probs = Vec::new();
        while probs.len() < 20000 {
            let i = 1 + rng.below(n / 2) + if n > 3 { 1 } else { 0 };
            let i = i.min(n);
            let steps = rng.below(2 * n);
            let pr = search_like(n, i, steps, &mut rng);
            let s = rng.perm(pr.n());
            probs.push(pr.relabel(&s));
        }
        let avg_n: f64 = probs.iter().map(|p| p.n() as f64).sum::<f64>() / probs.len() as f64;
        let mut cz = Canonizer::new();
        let t0 = Instant::now();
        let mut acc = 0u128;
        let reps = 5;
        for _ in 0..reps {
            for pr in &probs {
                acc ^= cz.key(pr);
            }
        }
        let dt = t0.elapsed().as_secs_f64();
        let calls = (reps * probs.len()) as f64;
        // also: child generation + reduce + key (what the solver does per child)
        let t1 = Instant::now();
        let mut kids = 0u64;
        for pr in probs.iter().take(4000) {
            let m = pr.n();
            for a in 0..m {
                for b in 0..m {
                    if a != b && !pr.p.comparable(a, b) {
                        acc ^= cz.key(&pr.child(a, b));
                        kids += 1;
                    }
                }
            }
        }
        let dt1 = t1.elapsed().as_secs_f64();
        println!(
            "bench n0={:2}: avg reduced n={:.1}  key(P,i): {:.0}/s ({:.2} us)  child+reduce+key: {:.0}/s  [nodes/call {:.1}, leaves/call {:.2}] {:x}",
            n,
            avg_n,
            calls / dt,
            1e6 * dt / calls,
            kids as f64 / dt1,
            cz.stats.nodes as f64 / cz.stats.calls as f64,
            cz.stats.leaves as f64 / cz.stats.calls as f64,
            acc & 0xf
        );
    }
    // worst-ish cases: symmetric structures
    let mut cz = Canonizer::new();
    for kind in 0..10 {
        let p = structured(16, kind, &mut rng);
        let t0 = Instant::now();
        let reps = 2000;
        let before = cz.stats;
        for _ in 0..reps {
            cz.canon_poset(&p);
        }
        let dt = t0.elapsed().as_secs_f64();
        println!(
            "structured kind {} n=16: {:.1} us/call, leaves/call {:.1}, autos/call {:.1}",
            kind,
            1e6 * dt / reps as f64,
            (cz.stats.leaves - before.leaves) as f64 / reps as f64,
            (cz.stats.autos - before.autos) as f64 / reps as f64
        );
    }
    let _ = low_mask(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("all");
    let num = |k: usize, d: usize| args.get(k).and_then(|s| s.parse().ok()).unwrap_or(d);
    match cmd {
        "exhaustive" => exhaustive(num(2, 7)),
        "problems" => problems(num(2, 6)),
        "random" => random(num(2, 20000)),
        "bench" => bench(),
        _ => {
            exhaustive(7);
            problems(6);
            random(20000);
            bench();
        }
    }
}
