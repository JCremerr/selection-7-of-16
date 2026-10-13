//! df-pn solver for "is there a selection algorithm for (n, i) with <= k comparisons?"
//!
//! usage: dfpn N I K [options]        (I is 1-based: the I-th smallest)
//! Defaults = the recommended config F1: --heur=cs --sigma=0.2 --shift=1 --eps=1,
//! weak AND sums (pn = max + 1). Add --and-add=0 for config F0.
//!   --tt-bits=B      TT = 2^B buckets x 4 entries x 40 bytes      (default 22 = 640 MB)
//!   --mc-bits=B      move-cache slots                             (default 16)
//!   --mc-moves=M     max move records in the move cache (48 B)    (default 4000000)
//!   --lbc-bits=B     LB cache slots (48 B)                        (default 21)
//!   --eps=E          1+eps trick                                  (default 1)
//!   --heur=unit|slack|real|model|cs, --shift=, --mu-a=, --mu-n0=, --mu-c=, --sigma=, --scale=, --size-pow=
//!   --and-sum        pn(AND) = pn1 + pn2 (default: weak, max + and-add, --and-add=1);  --logp=S  leaf numbers -S ln p, -S ln(1-p);
//!     --or-weak   dn(OR) = max + #alive - 1
//!   --order-nolb     static order without max-LB as primary key (pure cs first)
//!   --lb-early       early-terminated LB_best for LB-dead children (no measured gain)
//!   --order-old      static move order (max lb, max h, sum h) instead of (max lb, max log2|C|, max/min log2 Phi*)
//!   --seed=S --jitter=J  tie-break order / relative noise on leaf numbers (portfolios)
//!   --oracle=PATH    exact table (data/groundtruth_table.tsv) as endgame oracle
//!   --oracle-max-n=M only rows with n <= M                        (default 12)
//!   --time=SECS      time limit;  --nodes=N  expansion limit;  --progress=SECS
//!   --out=FILE.rs    write the certificate here if proved
//! Prints one "RESULT ..." line at the end.

use selcore::canon::key_n;
use selcore::cert::CertWriter;
use selcore::dfpn::{load_oracle, rss_mb, root_key, Config, Dfpn, Heur};
use std::rc::Rc;

fn opt<T: std::str::FromStr>(args: &[String], name: &str, def: T) -> T {
    let pre = format!("--{}=", name);
    args.iter().find_map(|a| a.strip_prefix(&pre)).map(|s| s.parse().ok().unwrap_or_else(|| panic!("bad value for --{}", name))).unwrap_or(def)
}

/// Diagnostic: for every comparing function of an optimal algorithm (dump from
/// tests/dump_expert.py: "n r0 above_hex x y W"), rank of the expert move among
/// our moves (static order, and heuristic leaf pn at budget W-1).
fn expert(path: &str, cfg: Config) {
    use selcore::poset::{Poset, Problem};
    let mut s = Dfpn::new(cfg);
    let text = std::fs::read_to_string(path).unwrap();
    // stats per (n bucket)
    let mut rows = 0usize;
    let mut top1_static = 0usize;
    let mut top1_pn = 0usize;
    let mut top1_pn_tie = 0usize;
    let mut sum_rank_static = 0usize;
    let mut sum_rank_pn = 0usize;
    let mut not_found = 0usize;
    let mut by_slack: Vec<(usize, usize, usize, usize)> = vec![(0, 0, 0, 0); 12];
    let mut hist_pn = vec![0usize; 8];
    let mut alt_rank = [0usize; 14];
    let mut alt_top1 = [0usize; 14];
    let mut alt_pos1 = [0usize; 14];
    for line in text.lines() {
        let t: Vec<&str> = line.split_whitespace().collect();
        let n: usize = t[0].parse().unwrap();
        let r0: usize = t[1].parse().unwrap();
        let ab: Vec<u16> = t[2].split(',').map(|x| u16::from_str_radix(x, 16).unwrap()).collect();
        let x: usize = t[3].parse().unwrap();
        let y: usize = t[4].parse().unwrap();
        let w: u8 = t[5].parse().unwrap();
        if n < 9 {
            continue;
        }
        let prob = Problem::new(Poset::from_above(n, &ab).unwrap(), r0 + 1);
        let red = prob.reduced();
        let key = s.canon.key(&red);
        let mut k1 = s.canon.key(&prob.child(x, y));
        let mut k2 = s.canon.key(&prob.child(y, x));
        if k1 > k2 {
            std::mem::swap(&mut k1, &mut k2);
        }
        let (lb, _) = s.static_info(key);
        let (moves, _) = s.moves(key, w);
        let bc = w - 1;
        let alive: Vec<_> = moves.iter().filter(|m| m.lb1 <= bc && m.lb2 <= bc).collect();
        let pos = alive.iter().position(|m| m.k1 == k1 && m.k2 == k2);
        rows += 1;
        let pos = match pos {
            Some(p) => p,
            None => {
                not_found += 1;
                continue;
            }
        };
        let pnv: Vec<u64> = alive
            .iter()
            .map(|m| {
                let a = if key_n(m.k1) == 1 { 0 } else { s.init_numbers(m.k1, key_n(m.k1), m.lb1, m.h1, bc).0 };
                let b = if key_n(m.k2) == 1 { 0 } else { s.init_numbers(m.k2, key_n(m.k2), m.lb2, m.h2, bc).0 };
                s.and_pn(a, b)
            })
            .collect();
        if let Ok(fp) = std::env::var("EXPERT_FEATURES") {
            // one line per alive move: node id, expert flag, n, b, child stats
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new().create(true).append(true).open(fp).unwrap();
            for (j, m) in alive.iter().enumerate() {
                let st = |k: selcore::Key| -> (f64, f64, f64, f64) {
                    if key_n(k) == 1 {
                        return (0.0, 0.0, 0.0, 1.0);
                    }
                    let p = selcore::canon::decode_key(k);
                    let li = selcore::lbbest::lb_info(&p);
                    ((li.c as f64).log2(), li.log2_phi() as f64, li.lb() as f64, p.n() as f64)
                };
                let a = st(m.k1);
                let b = st(m.k2);
                writeln!(f, "{} {} {} {} {:.4} {:.4} {:.4} {:.4} {} {} {} {}", rows, (j == pos) as u8, n, w, a.0, b.0, a.1, b.1, a.2, b.2, a.3, b.3).unwrap();
            }
        }
        let e = pnv[pos];
        let better = pnv.iter().filter(|&&v| v < e).count();
        let equal = pnv.iter().filter(|&&v| v == e).count();
        let rank_pn = better + pnv[..pos].iter().filter(|&&v| v == e).count();
        // alternative orderings: rank of expert under several keys (ties -> expert after equals)
        {
            use selcore::canon::decode_key;
            use selcore::lbbest::lb_info;
            let feats: Vec<[f64; 14]> = alive
                .iter()
                .map(|m| {
                    let f = |k: selcore::Key| -> (f64, f64, f64, f64, f64) {
                        if key_n(k) == 1 {
                            return (0.0, 0.0, 0.0, 0.0, 0.0);
                        }
                        let p = decode_key(k);
                        let li = lb_info(&p);
                        (li.lb() as f64, li.log2_phi() as f64, (li.c as f64).log2(), selcore::dfpn::predict_v(&p) as f64, p.n() as f64)
                    };
                    let a = f(m.k1);
                    let b = f(m.k2);
                    [
                        a.0.max(b.0) * 100.0 + a.1.max(b.1),          // 0 current static (approx)
                        a.1.max(b.1),                                  // 1 max log2Phi*
                        a.2.max(b.2),                                  // 2 max log2|C| (cs)
                        a.3.max(b.3),                                  // 3 max predV
                        a.0.max(b.0) * 100.0 + a.3.max(b.3),          // 4 max lb, then max predV
                        (a.1.exp2() + b.1.exp2()).log2() + 0.0 * a.1, // 5 log2(Phi1+Phi2)
                        a.0.max(b.0) * 100.0 + a.2.max(b.2),          // 6 max lb then cs
                        a.3.max(b.3) * 10.0 + a.1.max(b.1) * 0.1,     // 7 predV then phi
                        a.2.max(b.2) * 1000.0 + a.2.min(b.2),          // 8 maxC then minC (smaller)
                        a.2.max(b.2) * 1000.0 - a.2.min(b.2),          // 9 maxC then larger minC
                        a.2.max(b.2) * 1000.0 + a.1.max(b.1),          // 10 maxC then max phi
                        a.2.max(b.2) * 1000.0 + a.1.max(b.1) * 10.0 + a.1.min(b.1) * 0.01, // 11 maxC, maxphi, minphi
                        a.2.max(b.2) * 1000.0 - a.4.max(b.4) - a.4.min(b.4),  // 12 maxC then larger n (fewer eliminations)
                        a.2.max(b.2) * 1000.0 + a.4.max(b.4) + a.4.min(b.4),  // 13 maxC then smaller n
                    ]
                })
                .collect();
            for o in 0..14 {
                let e = feats[pos][o];
                let r = feats.iter().filter(|f| f[o] < e - 1e-9).count();
                let eq = feats.iter().filter(|f| (f[o] - e).abs() <= 1e-9).count();
                alt_rank[o] += r;
                alt_top1[o] += (r == 0) as usize;
                alt_pos1[o] += (r == 0 && eq == 1) as usize;
            }
        }
        top1_static += (pos == 0) as usize;
        top1_pn += (rank_pn == 0) as usize;
        top1_pn_tie += (better == 0) as usize;
        sum_rank_static += pos;
        sum_rank_pn += rank_pn;
        hist_pn[match better { 0 => 0, 1 => 1, 2 => 2, 3..=4 => 3, 5..=8 => 4, 9..=16 => 5, 17..=32 => 6, _ => 7 }] += 1;
        let sl = ((w - lb) as usize).min(11);
        by_slack[sl].0 += 1;
        by_slack[sl].1 += (rank_pn == 0) as usize;
        by_slack[sl].2 += better;
        by_slack[sl].3 += alive.len();
        let _ = equal;
    }
    let f = |a: usize| a as f64 / rows.max(1) as f64;
    println!("# {}: {} nodes (n >= 9), expert move not among alive moves: {}", path, rows, not_found);
    println!("# static order: top1 {:.3}, mean rank {:.2}", f(top1_static), sum_rank_static as f64 / rows as f64);
    let names = ["maxlb,maxphi", "maxphi", "maxC(cs)", "maxpredV", "maxlb,maxpredV", "log2(phi1+phi2)", "maxlb,maxC", "predV,phi", "maxC,minC", "maxC,-minC", "maxC,maxphi", "maxC,maxphi,minphi", "maxC,-n", "maxC,+n"];
    for o in 0..14 {
        println!("# order {:18}: top1(ties ok) {:.3}  unique-top1 {:.3}  mean #strictly-better {:.2}", names[o], f(alt_top1[o]), f(alt_pos1[o]), alt_rank[o] as f64 / rows as f64);
    }
    println!("# heuristic pn: top1 {:.3} (no strictly better: {:.3}), mean rank {:.2}", f(top1_pn), f(top1_pn_tie), sum_rank_pn as f64 / rows as f64);
    println!("# #moves with strictly smaller pn than the expert: 0:{} 1:{} 2:{} 3-4:{} 5-8:{} 9-16:{} 17-32:{} >32:{}", hist_pn[0], hist_pn[1], hist_pn[2], hist_pn[3], hist_pn[4], hist_pn[5], hist_pn[6], hist_pn[7]);
    for (sl, v) in by_slack.iter().enumerate() {
        if v.0 > 0 {
            println!("#   slack W-LB={:2}: nodes {:5}  top1(pn) {:.3}  mean #better {:.2}  mean #alive {:.1}", sl, v.0, v.1 as f64 / v.0 as f64, v.2 as f64 / v.0 as f64, v.3 as f64 / v.0 as f64);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let expert_mode = args.get(1).map(|a| a == "expert").unwrap_or(false);
    if args.len() < 4 && !expert_mode {
        eprintln!("usage: dfpn N I K [options]  |  dfpn expert DUMPFILE [options]  (see source header)");
        std::process::exit(2);
    }
    let flags = if expert_mode { &args[3..] } else { &args[4..] };
    let has = |f: &str| flags.iter().any(|a| a == f);

    let mut cfg = Config::default();
    cfg.tt_bits = opt(flags, "tt-bits", 22);
    cfg.mc_bits = opt(flags, "mc-bits", 16);
    cfg.mc_max_moves = opt(flags, "mc-moves", 4_000_000);
    cfg.lbc_bits = opt(flags, "lbc-bits", 21);
    cfg.eps = opt(flags, "eps", 1.0);
    // weak AND sum is the default (--and-weak accepted for old command lines)
    cfg.and_weak = !has("--and-sum");
    cfg.or_weak = has("--or-weak");
    cfg.and_add = opt(flags, "and-add", 1);
    cfg.logp = opt(flags, "logp", 0.0);
    cfg.seed = opt(flags, "seed", 0);
    cfg.order_cs = !has("--order-old");
    cfg.lb_early = has("--lb-early");
    cfg.order_lb_first = !has("--order-nolb");
    cfg.jitter = opt(flags, "jitter", 0.0);
    cfg.time_limit = opt(flags, "time", f64::INFINITY);
    cfg.node_limit = opt(flags, "nodes", u64::MAX);
    cfg.progress_secs = opt(flags, "progress", 30.0);
    let heur: String = opt(flags, "heur", "cs".to_string());
    cfg.heur = match heur.as_str() {
        "unit" => Heur::Unit,
        "slack" => Heur::Slack {
            mu_a: opt(flags, "mu-a", 0.25),
            mu_n0: opt(flags, "mu-n0", 7.2),
            sigma: opt(flags, "sigma", 0.33),
            scale: opt(flags, "scale", 10.0),
            size_pow: opt(flags, "size-pow", 0.0),
        },
        "model" => Heur::Model { sigma: opt(flags, "sigma", 0.35), scale: opt(flags, "scale", 10.0), shift: opt(flags, "shift", 0.0) },
        "cs" => Heur::Cs { sigma: opt(flags, "sigma", 0.2), scale: opt(flags, "scale", 10.0), shift: opt(flags, "shift", 1.0) },
        "real" => Heur::Real {
            mu_c: opt(flags, "mu-c", 1.7),
            mu_a: opt(flags, "mu-a", 0.24),
            sigma: opt(flags, "sigma", 0.45),
            scale: opt(flags, "scale", 10.0),
        },
        _ => panic!("unknown heuristic {}", heur),
    };
    if expert_mode {
        cfg.mc_bits = 4;
        expert(&args[2], cfg);
        return;
    }
    let n: usize = args[1].parse().unwrap();
    let i: usize = args[2].parse().unwrap();
    let k: u8 = args[3].parse().unwrap();
    assert!(n >= 1 && n <= 16 && i >= 1 && i <= n);
    let oracle_path: String = opt(flags, "oracle", String::new());
    let out: String = opt(flags, "out", String::new());
    println!("# dfpn n={} i={} k={} cfg={:?}", n, i, k, Config { oracle: None, ..cfg.clone() });

    let t0 = std::time::Instant::now();
    if !oracle_path.is_empty() {
        let maxn: usize = opt(flags, "oracle-max-n", 12);
        let (map, rows, noncanon) = load_oracle(&oracle_path, maxn).expect("cannot read oracle table");
        println!("# oracle: {} rows (n <= {}), {} keys, {} rows not in our canonical form, {:.1}s, rss {} MB", rows, maxn, map.len(), noncanon, t0.elapsed().as_secs_f64(), rss_mb());
        cfg.oracle = Some(Rc::new(map));
    }

    let mut s = Dfpn::new(cfg);
    println!("# tables: {} MB allocated", s.table_bytes() >> 20);
    let key = root_key(&mut s.canon, n, i);
    let (lb, h) = s.static_info(key);
    println!("# root key {:032x} n={} LB_best={} log2Phi*={:.3}", key, key_n(key), lb, h);

    let v = s.solve(key, k);
    let t = s.elapsed();
    s.progress_line();
    let status = if v.pn == 0 {
        "PROVED"
    } else if v.dn == 0 {
        "DISPROVED"
    } else {
        "UNKNOWN"
    };
    let st = s.st;
    println!(
        "RESULT n={} i={} k={} {} time={:.2}s expansions={} movegens={} canon={} lbc={} tt_used={} tt_repl={} rss={}MB lo={} hi={}",
        n, i, k, status, t, st.expansions, st.movegens, st.canon_calls, st.lb_computed, st.tt_used, st.tt_replaced, rss_mb(), v.lo, v.hi
    );
    println!("# expansions by depth: {}", s.depth_profile());
    if v.pn != 0 {
        let (used, pr, dis) = s.tt_census();
        println!("# TT entries: {} used, {} with a proof (ub), {} with a refutation (lb)", used, pr, dis);
        print!("{}", s.pv_report(key, k, 12));
    }
    if v.pn == 0 && !out.is_empty() {
        let t1 = std::time::Instant::now();
        s.cfg.time_limit = f64::INFINITY;
        s.cfg.node_limit = u64::MAX;
        let w = CertWriter::new(&mut s);
        match w.write(n, i, k) {
            Ok((src, worst, nf, ncmp, researched)) => {
                std::fs::write(&out, src).expect("cannot write certificate");
                println!(
                    "CERT {} worst={} functions={} comparing={} researched={} time={:.2}s",
                    out,
                    worst,
                    nf,
                    ncmp,
                    researched,
                    t1.elapsed().as_secs_f64()
                );
            }
            Err(e) => {
                println!("CERT-ERROR {}", e);
                std::process::exit(3);
            }
        }
    }
}
