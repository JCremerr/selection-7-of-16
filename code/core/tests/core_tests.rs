//! Cross-checks of the core against definitions (small n).

use selcore::bits::Rng;
use selcore::count::{ceil_log2, compat_count, compat_count_brute};
use selcore::poset::{Poset, Problem};
use selcore::solver::{Memo, Searcher};
use selcore::Canonizer;
use std::collections::HashMap;

/// Naive minimax on labeled, unreduced posets (no canonical forms, no reduction).
fn naive(prob: &Problem, memo: &mut HashMap<([u16; 16], u8, u8), u8>) -> u8 {
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
