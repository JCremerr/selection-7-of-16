import random, sys
import os; sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "verifier"))
import verify_rs_algorithm as V
def warshall(rel, m):
    R = [[(rel[u] >> v) & 1 for v in range(m)] for u in range(m)]
    for k in range(m):
        for i in range(m):
            if R[i][k]:
                for j in range(m):
                    if R[k][j]: R[i][j] = 1
    return [sum(R[u][v] << v for v in range(m)) for u in range(m)]
rng = random.Random(5); bad = 0; tests = 0
for t in range(3000):
    m = rng.randint(2, 17)
    perm = list(range(m)); rng.shuffle(perm)       # random linear order; posets = subsets of it
    pos = {e: i for i, e in enumerate(perm)}
    P = [0] * m
    for _ in range(rng.randint(0, 2 * m)):
        u, v = rng.sample(range(m), 2)
        if pos[u] > pos[v]: u, v = v, u
        P[u] |= 1 << v
    P = warshall(P, m)
    inc = [(u, v) for u in range(m) for v in range(m) if u != v and not (P[u] >> v) & 1 and not (P[v] >> u) & 1]
    if inc:
        lo, hi = rng.choice(inc); tests += 1
        A = V.add_relation(P, lo, hi); Q = list(P); Q[lo] |= 1 << hi
        if A != warshall(Q, m) or not V.is_strict_partial_order(A): bad += 1
    k = rng.randint(1, m); args = rng.sample(range(m), k); flip = rng.random() < .5
    R = V.restrict(P, args, flip)
    ref = [sum(1 << l for l in range(k) if ((P[args[l]] >> args[j]) & 1 if flip else (P[args[j]] >> args[l]) & 1)) for j in range(k)]
    if R != ref: bad += 1
print("add_relation tests", tests, "restrict tests 3000", "mismatches", bad)
