//! Exact V_i(n) for all n <= N, plus a ground-truth table of every reduced problem
//! expanded during the computation (all made exact in a second pass).
//!
//! usage: exact <N> [threads=1] [outdir] [--no-table]
//!
//! Parallelism: lazy-SMP style. All threads search the same root with a shared memo
//! (proven bounds only), with randomized move-order ties near the root for helpers;
//! the first thread to finish an iteration raises the abort flag for that iteration.
//! The second pass (exactify) distributes inexact memo entries over threads.

use selcore::canon::{decode_key, key_i, key_n, Key};
use selcore::count::{ceil_log2, compat_count};
use selcore::poset::Problem;
use selcore::solver::{Memo, SearchStats, Searcher};
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

const KNOWN: [&[u8]; 13] = [
    &[],
    &[0],
    &[1],
    &[2, 3],
    &[3, 4],
    &[4, 6, 6],
    &[5, 7, 8],
    &[6, 8, 10, 10],
    &[7, 9, 11, 12],
    &[8, 11, 12, 14, 14],
    &[9, 12, 14, 15, 16],
    &[10, 13, 15, 17, 18, 18],
    &[11, 14, 17, 18, 19, 20],
];

const LB_CACHE_BITS: u32 = 20;

/// Exact value of one canonical key using `threads` threads (lazy SMP).
fn exact_parallel(memo: &Memo, key: Key, threads: usize, stats: &Mutex<SearchStats>, seed: u64) -> u8 {
    if threads <= 1 {
        let mut s = Searcher::new(memo, LB_CACHE_BITS);
        let v = s.exact(key).unwrap();
        stats.lock().unwrap().add(&s.stats);
        return v;
    }
    loop {
        let mut s0 = Searcher::new(memo, 4);
        let b = s0.bounds(key);
        if b.lb == b.ub {
            return b.lb;
        }
        let k = b.lb;
        let abort = AtomicBool::new(false);
        std::thread::scope(|sc| {
            for t in 0..threads {
                let abort = &abort;
                sc.spawn(move || {
                    let jitter = if t == 0 { 0 } else { 6 };
                    let mut s = Searcher::new(memo, LB_CACHE_BITS).with_abort(abort, seed ^ (t as u64 * 0x9E37), jitter);
                    let prob = decode_key(key);
                    s.search(&prob, key, k, 0);
                    abort.store(true, Ordering::Relaxed);
                    stats.lock().unwrap().add(&s.stats);
                });
            }
        });
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nmax: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10);
    let threads: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
    let outdir = args.get(3).cloned().unwrap_or_else(|| "../../data/groundtruth".to_string());
    let no_table = args.iter().any(|a| a == "--no-table");
    let max_memo: usize = args.iter().find_map(|a| a.strip_prefix("--max-memo=")).and_then(|s| s.parse().ok()).unwrap_or(60_000_000);
    let p2_limit: f64 = args.iter().find_map(|a| a.strip_prefix("--p2-limit=")).and_then(|s| s.parse().ok()).unwrap_or(1e9);
    assert!(nmax <= 16);

    let memo = Memo::new(10);
    let stats = Mutex::new(SearchStats::default());
    let t_all = Instant::now();
    let mut all_ok = true;
    let mut log = String::new();
    let emit = |s: String, log: &mut String| {
        println!("{}", s);
        log.push_str(&s);
        log.push('\n');
    };
    emit(format!("# exact V_i(n), n <= {}, threads = {}", nmax, threads), &mut log);
    for n in 1..=nmax {
        let mut row = Vec::new();
        let t_n = Instant::now();
        for i in 1..=(n + 1) / 2 {
            let t0 = Instant::now();
            let key = {
                let mut s = Searcher::new(&memo, 4);
                s.key_of(&Problem::selection(n, i))
            };
            let v = if key_n(key) == 1 { 0 } else { exact_parallel(&memo, key, threads, &stats, (n * 100 + i) as u64) };
            let known = KNOWN.get(n).and_then(|r| r.get(i - 1)).copied();
            let ok = known.map_or(true, |kv| kv == v);
            all_ok &= ok;
            let st = *stats.lock().unwrap();
            emit(
                format!(
                    "V_{}({}) = {:2}   known {:>4}  {}  {:8.2}s  memo={} expansions={} children={}",
                    i,
                    n,
                    v,
                    known.map_or("?".to_string(), |k| k.to_string()),
                    if known.is_none() { "new?" } else if ok { "OK" } else { "MISMATCH" },
                    t0.elapsed().as_secs_f64(),
                    memo.len(),
                    st.expansions,
                    st.children
                ),
                &mut log,
            );
            row.push(v);
        }
        emit(format!("n={:2}: {:?}  ({:.2}s)", n, row, t_n.elapsed().as_secs_f64()), &mut log);
    }
    let t_phase1 = t_all.elapsed().as_secs_f64();
    emit(format!("# phase 1 done: {:.1}s, memo entries {}, all known values {}", t_phase1, memo.len(), if all_ok { "REPRODUCED" } else { "NOT reproduced" }), &mut log);
    if no_table {
        return;
    }

    // ---- phase 2: make every stored (expanded) node exact ----
    let t2 = Instant::now();
    let phase1: std::collections::HashSet<Key> = memo.entries().into_iter().map(|(k, _)| k).collect();
    let n_phase1 = phase1.len();
    let mut todo = memo.inexact_keys();
    {
        let mut h: std::collections::BTreeMap<usize, (usize, usize, usize)> = Default::default();
        for (k, b) in memo.entries() {
            let e: &mut (usize, usize, usize) = h.entry(key_n(k)).or_default();
            e.0 += 1;
            if b.lb != b.ub {
                e.1 += 1;
                if b.ub == selcore::solver::INF {
                    e.2 += 1;
                }
            }
        }
        for (n, (tot, inex, inf)) in h {
            emit(format!("#   n={:2}: stored {:8}  inexact {:8} (no ub: {:8})", n, tot, inex, inf), &mut log);
        }
    }
    todo.sort_by_key(|&k| {
        let b = memo.get(k).unwrap();
        (key_n(k), b.lb, k)
    });
    emit(format!("# phase 2: {} of {} stored nodes inexact; resolving (limit {}s)", todo.len(), n_phase1, p2_limit), &mut log);
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let stop = AtomicBool::new(false);
    std::thread::scope(|sc| {
        for t in 0..threads.max(1) {
            let (next, done, stop, todo, memo, stats) = (&next, &done, &stop, &todo, &memo, &stats);
            sc.spawn(move || {
                let mut s = Searcher::new(memo, LB_CACHE_BITS).with_abort(stop, 77 + t as u64, 0);
                loop {
                    let j = next.fetch_add(1, Ordering::Relaxed);
                    if j >= todo.len() || stop.load(Ordering::Relaxed) {
                        break;
                    }
                    if s.exact(todo[j]).is_none() {
                        break;
                    }
                    done.fetch_add(1, Ordering::Relaxed);
                }
                stats.lock().unwrap().add(&s.stats);
            });
        }
        // monitor
        let mut last = Instant::now();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(200));
            let d = done.load(Ordering::Relaxed);
            if next.load(Ordering::Relaxed) >= todo.len() + threads.max(1) || d >= todo.len() {
                break;
            }
            if t2.elapsed().as_secs_f64() > p2_limit || memo.len() > max_memo {
                stop.store(true, Ordering::Relaxed);
                break;
            }
            if last.elapsed().as_secs() >= 60 {
                last = Instant::now();
                let j = next.load(Ordering::Relaxed).min(todo.len().saturating_sub(1));
                println!("#   phase 2 progress: {}/{} done, at n={}, memo {} entries, {:.0}s", d, todo.len(), key_n(todo[j]), memo.len(), t2.elapsed().as_secs_f64());
            }
        }
    });
    let mut n_entries = 0usize;
    let mut n_exact = 0usize;
    memo.for_each(|_, b| {
        n_entries += 1;
        n_exact += (b.lb == b.ub) as usize;
    });
    let p1_exact = phase1.iter().filter(|k| memo.get(**k).map_or(false, |b| b.lb == b.ub)).count();
    emit(
        format!(
            "# phase 2 done: {:.1}s{}; memo now {} entries, {} exact; phase-1 nodes {} of which exact {}",
            t2.elapsed().as_secs_f64(),
            if stop.load(Ordering::Relaxed) { " (TIME/MEMORY LIMIT HIT: remaining nodes flagged lb)" } else { "" },
            n_entries,
            n_exact,
            n_phase1,
            p1_exact
        ),
        &mut log,
    );

    // ---- phase 3: write table ----
    std::fs::create_dir_all(&outdir).unwrap();
    let path = format!("{}/table.tsv", outdir);
    let mut f = std::io::BufWriter::new(std::fs::File::create(&path).unwrap());
    let mut rows: Vec<(usize, usize, u8, Key, bool, bool)> =
        memo.entries_where(|k, b| b.lb == b.ub || phase1.contains(&k)) // phase-2-only nodes only if exact
            .iter()
            .map(|(k, b)| (key_n(*k), key_i(*k), b.lb, *k, b.lb == b.ub, phase1.contains(k)))
            .collect();
    // add the trivial solved problem n=1 for completeness
    rows.push((1, 1, 0, Key::ZERO, true, true));
    rows.sort();
    let mut gap_hist: std::collections::BTreeMap<(usize, i32), usize> = Default::default();
    let mut gap_by_v: std::collections::BTreeMap<(u8, i32), usize> = Default::default();
    let mut per_n: std::collections::BTreeMap<usize, usize> = Default::default();
    for &(n, i, v, k, exact, p1) in &rows {
        let prob = decode_key(k);
        let c = compat_count(&prob);
        let l = ceil_log2(c);
        writeln!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", n, i, v, prob.p.above_hex(), c, l, if exact { "exact" } else { "lb" }, if p1 { 1 } else { 2 }).unwrap();
        if !exact {
            continue;
        }
        *gap_hist.entry((n, v as i32 - l as i32)).or_default() += 1;
        *gap_by_v.entry((v, v as i32 - l as i32)).or_default() += 1;
        *per_n.entry(n).or_default() += 1;
    }
    f.flush().unwrap();
    emit(format!("# wrote {} rows to {}", rows.len(), path), &mut log);
    emit("# distinct reduced problems per n (rows):".to_string(), &mut log);
    for (n, c) in &per_n {
        emit(format!("n={:2}: {}", n, c), &mut log);
    }
    emit("# gap V - ceil(log2|C|) histogram by n: n: gap:count ...".to_string(), &mut log);
    for n in per_n.keys() {
        let s: Vec<String> = gap_hist.iter().filter(|((m, _), _)| m == n).map(|((_, g), c)| format!("{}:{}", g, c)).collect();
        emit(format!("n={:2}: {}", n, s.join(" ")), &mut log);
    }
    emit("# gap histogram by V: V: gap:count ...".to_string(), &mut log);
    let vs: std::collections::BTreeSet<u8> = gap_by_v.keys().map(|(v, _)| *v).collect();
    for v in vs {
        let s: Vec<String> = gap_by_v.iter().filter(|((w, _), _)| *w == v).map(|((_, g), c)| format!("{}:{}", g, c)).collect();
        emit(format!("V={:2}: {}", v, s.join(" ")), &mut log);
    }
    let st = *stats.lock().unwrap();
    emit(format!("# total {:.1}s; stats {:?}", t_all.elapsed().as_secs_f64(), st), &mut log);
    std::fs::write(format!("{}/run_log.txt", outdir), log).unwrap();
}
