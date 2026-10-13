// Second, independent sanity check: compile the generated algorithm with rustc
// (real Rust semantics) and run it on random permutations of 0..N.
// The result must be the element of 0-based rank I, i.e. the value I.
// Usage: ./build_and_run.sh ALGO_FILE TRIALS  (sets ALGO_FILE and calls rustc)
#![allow(dead_code)]
mod algo {
    include!(env!("ALGO_FILE"));
}
fn main() {
    let trials: u64 = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap_or(10_000_000);
    let mut s: u64 = std::env::args().nth(2).map(|s| s.parse().unwrap()).unwrap_or(0x9E3779B97F4A7C15);
    let mut next = || { s ^= s << 13; s ^= s >> 7; s ^= s << 17; s };
    const N: usize = algo::N;
    let mut a = [0usize; N];
    for t in 0..trials {
        for (k, x) in a.iter_mut().enumerate() { *x = k; }
        for k in (1..N).rev() { let j = (next() % (k as u64 + 1)) as usize; a.swap(k, j); }
        let r = algo::run(a);
        if r != algo::I { println!("FAIL trial {t}: input {:?} returned {r}, expected {}", a, algo::I); std::process::exit(1); }
    }
    println!("OK: {trials} random permutations, N = {N}, I(0-based) = {}", algo::I);
}
