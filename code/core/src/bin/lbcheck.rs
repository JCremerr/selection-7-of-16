//! Checks of the LB_best port (src/lbbest.rs).
//!
//! usage:
//!   lbcheck dump <count> <seed>     random problems, one per line:
//!                                   "n i above_hex C FG FGdual KEY"
//!                                   (compare with tests/lb_compare.py)
//!   lbcheck table <table.tsv>       ground-truth table (data/): assert
//!                                   LB_best <= V and |C| == table column, print the
//!                                   gap V - LB_best histogram per n.

use selcore::bits::Rng;
use selcore::count::{ceil_log2, compat_count};
use selcore::lbbest::lb_info;
use selcore::poset::{Poset, Problem};
use std::io::{BufRead, BufWriter, Write};

/// Random problem: either a random closed order, or a random "search path" (random
/// comparisons with random outcomes from the antichain, optionally reduced).
fn random_problem(rng: &mut Rng) -> Problem {
    let n = 1 + rng.below(16);
    let i = 1 + rng.below(n);
    match rng.below(3) {
        0 => {
            let dens = [0.0, 0.05, 0.1, 0.2, 0.4][rng.below(5)];
            Problem::new(Poset::random(n, dens, rng), i)
        }
        _ => {
            let mut prob = Problem::selection(n, i);
            let steps = rng.below(n * 2 + 1);
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("");
    match mode {
        "dump" => {
            let count: usize = args[2].parse().unwrap();
            let seed: u64 = args[3].parse().unwrap();
            let mut rng = Rng::new(seed);
            let out = std::io::stdout();
            let mut w = BufWriter::new(out.lock());
            for _ in 0..count {
                let prob = random_problem(&mut rng);
                let li = lb_info(&prob);
                // internal cross-check against the existing |C| counter
                assert_eq!(li.c, compat_count(&prob));
                writeln!(w, "{} {} {} {} {} {} {}", prob.n(), prob.i, prob.p.above_hex(), li.c, li.fg, li.fgd, li.key).unwrap();
            }
        }
        "table" => {
            let f = std::fs::File::open(&args[2]).unwrap();
            let r = std::io::BufReader::new(f);
            // hist[n][gap]
            let mut hist = vec![vec![0u64; 16]; 17];
            let mut hist_base = vec![vec![0u64; 16]; 17];
            let mut rows = 0u64;
            for line in r.lines() {
                let line = line.unwrap();
                let t: Vec<&str> = line.split('\t').collect();
                if t.len() < 7 || t[6] != "exact" {
                    continue;
                }
                let n: usize = t[0].parse().unwrap();
                let i: usize = t[1].parse().unwrap();
                let v: u32 = t[2].parse().unwrap();
                let p = Poset::from_above_hex(t[3]).expect("bad poset");
                let c: u64 = t[4].parse().unwrap();
                assert_eq!(p.n(), n);
                let prob = Problem::new(p, i);
                let li = lb_info(&prob);
                assert_eq!(li.c, c, "|C| mismatch on {}", line);
                let lb = li.lb();
                assert!(lb <= v, "LB_best {} > V {} on {}", lb, v, line);
                assert!(lb >= ceil_log2(c), "LB_best below baseline on {}", line);
                // duality: the bound must be the same on the dual problem
                let ld = lb_info(&prob.dual());
                assert_eq!(ld.c, li.c);
                assert_eq!(ld.key, li.key);
                assert_eq!(ld.phistar(), li.phistar(), "Phi* not dual-invariant on {}", line);
                hist[n][(v - lb) as usize] += 1;
                hist_base[n][(v - ceil_log2(c).max(prob.p.components() as u32 - 1)) as usize] += 1;
                rows += 1;
            }
            println!("# {} exact rows checked: LB_best <= V everywhere, |C| equal, dual-invariant", rows);
            println!("# gap V - LB_best per n (count by gap 0,1,2,..), then mean");
            for n in 1..=16 {
                let tot: u64 = hist[n].iter().sum();
                if tot == 0 {
                    continue;
                }
                let mean: f64 = hist[n].iter().enumerate().map(|(g, &c)| g as f64 * c as f64).sum::<f64>() / tot as f64;
                let meanb: f64 = hist_base[n].iter().enumerate().map(|(g, &c)| g as f64 * c as f64).sum::<f64>() / tot as f64;
                let last = hist[n].iter().rposition(|&c| c > 0).unwrap();
                let s: Vec<String> = hist[n][..=last].iter().map(|c| c.to_string()).collect();
                println!("n={:2} rows={:8} gap: {}  mean {:.3} (static_lb mean {:.3})", n, tot, s.join(" "), mean, meanb);
            }
        }
        "gapfit" => {
            // per n: mean/sd of V - lb, V - log2Phi*, V - max(log2Phi*, KEY), and the
            // empirical P(V <= lb + s) for s = 0..4
            let f = std::fs::File::open(&args[2]).unwrap();
            let r = std::io::BufReader::new(f);
            let mut acc = vec![[0f64; 7]; 17];
            let mut cum = vec![[0u64; 6]; 17];
            for line in r.lines() {
                let line = line.unwrap();
                let t: Vec<&str> = line.split('\t').collect();
                if t.len() < 7 || t[6] != "exact" {
                    continue;
                }
                let n: usize = t[0].parse().unwrap();
                let i: usize = t[1].parse().unwrap();
                let v: f64 = t[2].parse().unwrap();
                let prob = Problem::new(Poset::from_above_hex(t[3]).unwrap(), i);
                let li = lb_info(&prob);
                let lb = li.lb() as f64;
                let x = li.log2_phi() as f64;
                let y = x.max(li.key as f64);
                let a = &mut acc[n];
                a[0] += 1.0;
                a[1] += v - lb;
                a[2] += (v - lb) * (v - lb);
                a[3] += v - x;
                a[4] += (v - x) * (v - x);
                a[5] += v - y;
                a[6] += (v - y) * (v - y);
                let g = (v - lb) as usize;
                for s in g.min(5)..6 {
                    cum[n][s] += 1;
                }
            }
            for n in 2..=16 {
                let a = acc[n];
                if a[0] == 0.0 {
                    continue;
                }
                let m = |s: f64, q: f64| (s / a[0], (q / a[0] - (s / a[0]).powi(2)).max(0.0).sqrt());
                let (m1, s1) = m(a[1], a[2]);
                let (m2, s2) = m(a[3], a[4]);
                let (m3, s3) = m(a[5], a[6]);
                let p: Vec<String> = cum[n].iter().map(|&c| format!("{:.4}", c as f64 / a[0])).collect();
                println!("n={:2} rows={:8} V-lb {:.3}+-{:.3}  V-log2phi {:.3}+-{:.3}  V-max(log2phi,key) {:.3}+-{:.3}  P(gap<=s) {}", n, a[0], m1, s1, m2, s2, m3, s3, p.join(" "));
            }
        }
        "regress" => {
            // least squares of g = V - log2Phi* on features; prints coefficients,
            // residual sd overall and per n
            let f = std::fs::File::open(&args[2]).unwrap();
            let r = std::io::BufReader::new(f);
            // optional feature subset (indices into the 9 features + 9 = log2C, 10 = n*log2C/16, 11 = lb)
            let all_names = ["1", "n", "x", "rel_frac", "comps", "key-x", "minmax", "x/n", "log2C-x", "log2C", "n*log2C/16", "lb"];
            let subset: Vec<usize> = args.get(3).map(|s| s.split(',').map(|x| x.parse().unwrap()).collect()).unwrap_or((0..9).collect());
            let names: Vec<&str> = subset.iter().map(|&j| all_names[j]).collect();
            const F: usize = 12;
            let nf = subset.len();
            let mut rows: Vec<([f64; F], f64, usize)> = Vec::new();
            for line in r.lines() {
                let line = line.unwrap();
                let t: Vec<&str> = line.split('\t').collect();
                if t.len() < 7 || t[6] != "exact" {
                    continue;
                }
                let n: usize = t[0].parse().unwrap();
                if n < 6 {
                    continue;
                }
                let i: usize = t[1].parse().unwrap();
                let v: f64 = t[2].parse().unwrap();
                let prob = Problem::new(Poset::from_above_hex(t[3]).unwrap(), i);
                let ff = selcore::dfpn::features(&prob);
                let li = lb_info(&prob);
                let l2c = (li.c as f64).log2();
                let full = [ff[0], ff[1], ff[2], ff[3], ff[4], ff[5], ff[6], ff[7], ff[8], l2c, n as f64 * l2c / 16.0, li.lb() as f64];
                let mut xx = [0f64; F];
                for (j, &q) in subset.iter().enumerate() {
                    xx[j] = full[q];
                }
                // target: V itself if x (index 2) is not a feature... keep V - x for compatibility only when 2 in subset
                let target = if subset.contains(&2) { v - li.log2_phi() as f64 } else { v };
                rows.push((xx, target, n));
            }
            let mut a = [[0f64; F]; F];
            let mut bv = [0f64; F];
            for (x, g, _) in &rows {
                for p in 0..F {
                    bv[p] += x[p] * g;
                    for q in 0..F {
                        a[p][q] += x[p] * x[q];
                    }
                }
            }
            // ridge + Gaussian elimination
            for p in 0..F {
                a[p][p] += 1e-6;
            }
            let _ = nf;
            for c in 0..F {
                let piv = (c..F).max_by(|&x, &y| a[x][c].abs().partial_cmp(&a[y][c].abs()).unwrap()).unwrap();
                a.swap(c, piv);
                bv.swap(c, piv);
                for r2 in 0..F {
                    if r2 != c {
                        let fct = a[r2][c] / a[c][c];
                        for q in 0..F {
                            a[r2][q] -= fct * a[c][q];
                        }
                        bv[r2] -= fct * bv[c];
                    }
                }
            }
            let w: Vec<f64> = (0..F).map(|p| bv[p] / a[p][p]).collect();
            for p in 0..nf {
                println!("coef {:10} {:+.5}", names[p], w[p]);
            }
            let mut per = vec![(0f64, 0f64, 0f64); 17];
            for (x, g, n) in &rows {
                let pred: f64 = (0..F).map(|p| w[p] * x[p]).sum();
                let e = g - pred;
                per[*n].0 += 1.0;
                per[*n].1 += e;
                per[*n].2 += e * e;
            }
            for n in 6..=16 {
                let (c, s1, s2) = per[n];
                if c > 0.0 {
                    println!("n={:2} rows={:8} resid mean {:+.3} sd {:.3}", n, c, s1 / c, (s2 / c - (s1 / c).powi(2)).sqrt());
                }
            }
            println!("COEFS {}", w[..nf].iter().map(|x| format!("{:.6}", x)).collect::<Vec<_>>().join(","));
        }
        "oracle" => {
            // spot-check of the ground-truth table as a df-pn oracle: canonical form
            // agreement, then exact V recomputed for `count` random rows
            let count: usize = args[3].parse().unwrap();
            let maxn: usize = args.get(4).map(|x| x.parse().unwrap()).unwrap_or(12);
            let (map, rows, noncanon) = selcore::dfpn::load_oracle(&args[2], 12).unwrap();
            println!("# loaded {} rows, {} keys, {} rows not stored in our canonical form", rows, map.len(), noncanon);
            let mut keys: Vec<(selcore::Key, u8)> = map.iter().filter(|(k, _)| selcore::canon::key_n(**k) <= maxn).map(|(k, v)| (*k, *v)).collect();
            keys.sort();
            let mut rng = Rng::new(12345 + maxn as u64);
            let memo = selcore::solver::Memo::new(6);
            let mut s = selcore::solver::Searcher::new(&memo, 18);
            let mut bad = 0;
            let mut by_n = [0usize; 17];
            let t0 = std::time::Instant::now();
            for c in 0..count {
                let (k, v) = keys[rng.below(keys.len())];
                let prob = selcore::canon::decode_key(k);
                let w = s.value(&prob);
                by_n[prob.n()] += 1;
                if w != v {
                    bad += 1;
                    println!("MISMATCH key {:032x} table {} exact {}", k, v, w);
                }
                if (c + 1) % 50 == 0 {
                    println!("# {} rows done, {} mismatches, {:.1}s", c + 1, bad, t0.elapsed().as_secs_f64());
                }
            }
            println!("# {} random rows recomputed with the exact solver: {} mismatches ({:.1}s); per n: {:?}", count, bad, t0.elapsed().as_secs_f64(), &by_n[2..13]);
        }
        _ => {
            eprintln!("usage: lbcheck dump <count> <seed> | lbcheck table <table.tsv>");
            std::process::exit(2);
        }
    }
}
