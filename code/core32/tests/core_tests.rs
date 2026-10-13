//! Cross-checks of the core against definitions (small n).

use selcore::bits::Rng;
use selcore::count::{ceil_log2, compat_count, compat_count_brute};
use selcore::poset::{Poset, Problem, MAXN};
use selcore::solver::{Memo, Searcher};
use selcore::Canonizer;
use std::collections::HashMap;

/// Naive minimax on labeled, unreduced posets (no canonical forms, no reduction).
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

fn random_problem(rng: &mut Rng, nmax: usize) -> Problem {
    let n = 1 + rng.below(nmax);
    let dens = [0.0, 0.1, 0.2, 0.4][rng.below(4)];
    let p = Poset::random(n, dens, rng);
    Problem::new(p, 1 + rng.below(n))
}

#[test]
fn transitive_closure_and_reduction() {
    let mut rng = Rng::new(1);
    for _ in 0..2000 {
        let pr = random_problem(&mut rng, 16);
        assert!(pr.p.is_valid());
        let r = pr.reduced();
        assert!(r.p.is_valid());
        assert!(r.is_reduced());
        // reduction preserves |C| and solvedness
        if pr.n() <= 12 {
            assert_eq!(compat_count(&pr), compat_count(&r));
        }
        assert_eq!(pr.is_solved(), r.n() == 1);
        // duality is an involution
        assert_eq!(pr.dual().dual(), pr);
    }
}

#[test]
fn compat_count_matches_definition() {
    let mut rng = Rng::new(2);
    for _ in 0..3000 {
        let pr = random_problem(&mut rng, 9);
        let c = compat_count(&pr);
        assert_eq!(c, compat_count_brute(&pr));
        assert_eq!(c, compat_count(&pr.dual()));
        assert_eq!(c == 1, pr.is_solved());
    }
    // E_n: n * C(n-1, i-1)
    assert_eq!(compat_count(&Problem::selection(12, 6)), 12 * 462);
    assert_eq!(ceil_log2(1), 0);
    assert_eq!(ceil_log2(2), 1);
    assert_eq!(ceil_log2(5), 3);
    assert_eq!(ceil_log2(8), 3);
}

#[test]
fn canonical_keys_invariant() {
    let mut rng = Rng::new(3);
    let mut cz = Canonizer::new();
    for _ in 0..5000 {
        let pr = random_problem(&mut rng, 16);
        let k = cz.key(&pr);
        let s = rng.perm(pr.n());
        assert_eq!(k, cz.key(&pr.relabel(&s)));
        assert_eq!(k, cz.key(&pr.dual().relabel(&s)));
        let d = selcore::decode_key(k);
        assert!(d.p.is_valid());
        assert_eq!(compat_count(&d), compat_count(&pr));
    }
}

#[test]
fn solver_matches_naive_minimax() {
    let memo = Memo::new(4);
    let mut s = Searcher::new(&memo, 12);
    let mut nm = HashMap::new();
    // all (E_n, i), n <= 6
    let known: [&[u8]; 7] = [&[], &[0], &[1, 1], &[2, 3, 2], &[3, 4, 4, 3], &[4, 6, 6, 6, 4], &[5, 7, 8, 8, 7, 5]];
    for n in 1..=6 {
        for i in 1..=n {
            let pr = Problem::selection(n, i);
            let v = s.value(&pr);
            assert_eq!(v, known[n][i - 1], "V_{}({})", i, n);
            assert_eq!(v, naive(&pr, &mut nm));
        }
    }
    // random problems n <= 6 (labeled, unreduced) vs naive
    let mut rng = Rng::new(4);
    for _ in 0..400 {
        let pr = random_problem(&mut rng, 6);
        assert_eq!(s.value(&pr), naive(&pr, &mut nm), "{:?}", pr);
    }
}

// ---------------------------------------------------------------------------
// core32 additions: n up to 19, 192-bit keys.
// ---------------------------------------------------------------------------

#[test]
fn n19_closure_reduction_and_canonical_invariance() {
    let mut rng = Rng::new(31);
    let mut cz = Canonizer::new();
    for t in 0..3000 {
        let pr = random_problem(&mut rng, MAXN);
        assert!(pr.p.is_valid());
        let r = pr.reduced();
        assert!(r.p.is_valid() && r.is_reduced());
        assert_eq!(pr.is_solved(), r.n() == 1);
        assert_eq!(pr.dual().dual(), pr);
        let k = cz.key(&pr);
        assert_eq!(selcore::canon::key_n(k), pr.n());
        let s = rng.perm(pr.n());
        assert_eq!(k, cz.key(&pr.relabel(&s)));
        assert_eq!(k, cz.key(&pr.dual().relabel(&s)));
        let d = selcore::decode_key(k);
        assert!(d.p.is_valid());
        assert_eq!(d.p.num_relations(), pr.p.num_relations());
        if t % 10 == 0 {
            assert_eq!(compat_count(&d), compat_count(&pr));
        }
        // the key really determines the class: a canonical problem is its own key
        assert_eq!(cz.key(&d), k);
    }
}

#[test]
fn key_layout() {
    use selcore::canon::{cert_of, decode_cert, key_i, key_n, make_key, Key};
    // n / i fields round-trip and never overlap the relation bits (n = 19: bits 0..170)
    let chain = Poset::chain(MAXN);
    let mut id = [0u8; MAXN];
    for (k, x) in id.iter_mut().enumerate() {
        *x = k as u8;
    }
    let c = cert_of(&chain, &id);
    assert_eq!(c.0[0] >> 43, 0, "relation bits above 170");
    assert_eq!(decode_cert(c, MAXN), chain);
    for n in 1..=MAXN {
        for i in 1..=n {
            let k = make_key(c, n, i);
            assert_eq!((key_n(k), key_i(k)), (n, i));
        }
    }
    // order of keys = (i, n, certificate), as in code/core
    let a = make_key(Key::ZERO, 19, 1);
    let b = make_key(c, 3, 2);
    assert!(a < b);
    let mut c1 = Key::ZERO;
    c1.set_bit(170);
    assert!(make_key(Key::ZERO, 18, 2) < make_key(c1, 18, 2));
    assert!(make_key(c1, 17, 2) < make_key(Key::ZERO, 18, 2));
    // fold: for n <= 16 the old u128 key cert | (n-1) << 120 | (i-1) << 124
    let mut rng = Rng::new(5);
    let mut cz = Canonizer::new();
    for _ in 0..2000 {
        let pr = random_problem(&mut rng, 16);
        let k = cz.key(&pr);
        let (f, e) = k.fold();
        assert_eq!(e, 0);
        let old = k.low128() | ((key_n(k) as u128 - 1) << 120) | ((key_i(k) as u128 - 1) << 124);
        assert_eq!(f, old);
        assert_eq!(k.0[0] & ((1u64 << 43) - 1), 0);
    }
    // get_bits across word boundaries
    let mut k = Key::ZERO;
    for p in [60u32, 63, 64, 65, 127, 128, 129, 170] {
        k.set_bit(p);
    }
    assert_eq!(k.get_bits(58, 10), (1 << 2) | (1 << 5) | (1 << 6) | (1 << 7));
    assert_eq!(k.get_bits(126, 5), 0b01110);
    assert_eq!(k.get_bits(160, 11), 1 << 10);
}

#[test]
fn compat_count_n17_to_19() {
    // E_n: n * C(n-1, i-1)
    assert_eq!(compat_count(&Problem::selection(19, 10)), 19 * 48620);
    assert_eq!(compat_count(&Problem::selection(17, 9)), 17 * 12870);
    let mut rng = Rng::new(9);
    for n in 17..=19 {
        for _ in 0..2 {
            let p = Poset::random(n, 0.15, &mut rng);
            let pr = Problem::new(p, 1 + rng.below(n));
            assert_eq!(compat_count(&pr), compat_count_brute(&pr));
            assert_eq!(compat_count(&pr), compat_count(&pr.dual()));
        }
    }
}
