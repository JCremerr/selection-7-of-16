//! Tests of the df-pn module, the certificate writer and the LB_best port.

use selcore::bits::Rng;
use selcore::cert::CertWriter;
use selcore::count::{ceil_log2, compat_count_brute};
use selcore::dfpn::{root_key, Config, Dfpn, Heur};
use selcore::lbbest::lb_info;
use selcore::poset::{Poset, Problem};
use selcore::solver::{Memo, Searcher};

const KNOWN: [&[u8]; 9] = [&[], &[0], &[1], &[2, 3], &[3, 4], &[4, 6, 6], &[5, 7, 8], &[6, 8, 10, 10], &[7, 9, 11, 12]];

fn small_cfg(heur: Heur, and_weak: bool, and_add: u64) -> Config {
    Config { tt_bits: 8, mc_bits: 4, mc_max_moves: 2000, lbc_bits: 8, heur, and_weak, and_add, progress_secs: 1e9, ..Config::default() }
}

#[test]
fn lb_best_valid_and_consistent() {
    let mut rng = Rng::new(7);
    let memo = Memo::new(4);
    let mut s = Searcher::new(&memo, 12);
    for _ in 0..400 {
        let n = 1 + rng.below(8);
        let i = 1 + rng.below(n);
        let dens = [0.0, 0.1, 0.25][rng.below(3)];
        let prob = Problem::new(Poset::random(n, dens, &mut rng), i);
        let li = lb_info(&prob);
        assert_eq!(li.c, compat_count_brute(&prob));
        assert!(li.fg >= li.c && li.fgd >= li.c);
        assert!(li.lb() >= ceil_log2(li.c));
        let v = s.value(&prob) as u32;
        assert!(li.lb() <= v, "LB_best {} > V {}", li.lb(), v);
        // invariant under reduction and duality
        assert_eq!(lb_info(&prob.dual()).phistar(), li.phistar());
        assert_eq!(lb_info(&prob.reduced()).c, li.c);
    }
}

#[test]
fn dfpn_proves_at_v_and_disproves_below_with_tiny_tables() {
    let heurs = [
        (Heur::Unit, false, 1),
        (Heur::Cs { sigma: 0.2, scale: 10.0, shift: 1.0 }, true, 1),
        (Heur::Cs { sigma: 0.2, scale: 10.0, shift: 1.0 }, true, 0),
        (Heur::Slack { mu_a: 0.55, mu_n0: 7.2, sigma: 0.2, scale: 10.0, size_pow: 0.0 }, true, 1),
    ];
    for &(h, w, add) in &heurs {
        for n in 2..=8 {
            for (j, &v) in KNOWN[n].iter().enumerate() {
                let i = j + 1;
                let mut d = Dfpn::new(small_cfg(h, w, add));
                let key = root_key(&mut d.canon, n, i);
                let r = d.solve(key, v);
                assert_eq!(r.pn, 0, "({},{}) not proved at V={} ({:?})", n, i, v, h);
                assert!(r.hi <= v);
                // certificate extraction must succeed with worst case <= v
                let (src, worst, nf, _, _) = CertWriter::new(&mut d).write(n, i, v).expect("certificate");
                assert!(worst <= v && nf >= 1 && src.contains("fn select_0("));
                let mut d2 = Dfpn::new(small_cfg(h, w, add));
                let key = root_key(&mut d2.canon, n, i);
                let r2 = d2.solve(key, v - 1);
                assert_eq!(r2.dn, 0, "({},{}) not disproved at V-1={} ({:?})", n, i, v - 1, h);
                assert!(r2.lo >= v);
            }
        }
    }
}
