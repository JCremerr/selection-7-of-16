"""Dump every comparing function of a verified algorithm file as
   "n r0 above_hex x y W"  (precondition P_f in the function's own orientation,
   0-based rank r0, compared positions x, y, W = worst case of f).
Uses the independent verifier's parser/linker read-only (code/verifier).
usage: python3 -I tests/dump_expert.py ALGO.rs > out.txt
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "..", "verifier"))
import verify_rs_algorithm as V  # noqa: E402


def main():
    text = open(sys.argv[1]).read()
    prog = V.parse(text)
    V.link(prog)
    order = V.topological_order(prog)
    pre = {prog.root.name: [0] * prog.root.arity}
    rows = []
    for f in order:
        P = pre.pop(f.name)
        b = f.body
        if isinstance(b, V.If):
            branches = [(V.add_relation(P, b.x, b.y), b.then), (V.add_relation(P, b.y, b.x), b.els)]
            rows.append((f, P, b.x, b.y))
        else:
            branches = [(P, b)]
        for Q, stmt in branches:
            V.check_stmt(f, Q, stmt, pre)
    cost = {}
    for f in reversed(order):
        b = f.body
        stmts = [b.then, b.els] if isinstance(b, V.If) else [b]
        cost[f.name] = (1 if isinstance(b, V.If) else 0) + max(
            cost[s.callee.name] if isinstance(s, V.Call) else 0 for s in stmts)
    for f, P, x, y in rows:
        print(f.arity, f.doc_i, ",".join("%x" % m for m in P), x, y, cost[f.name])


if __name__ == "__main__":
    main()
