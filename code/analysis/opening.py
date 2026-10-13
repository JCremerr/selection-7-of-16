"""Print the opening of a selection algorithm (generated Rust, either format).

Follows the path where every comparison answers "first operand smaller" for
DEPTH comparisons and prints, after each comparison, the shape of the active
poset (elements still passed down): component sizes and Hasse edges.
Reuses the independent verifier's parser (code/verifier).
Usage: python3 -I opening.py FILE.rs [DEPTH]
"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "verifier"))
import verify_rs_algorithm as V

def closure_add(above, a, b):
    # global strict order as dict elem -> set of elements above
    ups = {b} | above[b]
    downs = {x for x in above if a in above[x]} | {a}
    for x in downs:
        above[x] |= ups

def shape(active, above):
    act = set(active)
    # components of the comparability graph restricted to active
    adj = {x: {y for y in act if y != x and (y in above[x] or x in above[y])} for x in act}
    seen, comps = set(), []
    for x in act:
        if x in seen: continue
        st, c = [x], 0
        seen.add(x)
        while st:
            u = st.pop(); c += 1
            for v in adj[u]:
                if v not in seen: seen.add(v); st.append(v)
        comps.append(c)
    hasse = sum(1 for x in act for y in act if y in above[x] and
                not any(z in act and z in above[x] and y in above[z] for z in act))
    return sorted(comps, reverse=True), hasse

def main():
    text = open(sys.argv[1]).read()
    depth = int(sys.argv[2]) if len(sys.argv) > 2 else 16
    prog = V.parse(text); V.link(prog)
    f, args, flip = prog.root, list(range(prog.root.arity)), prog.root_dual
    n = len(args)
    above = {x: set() for x in range(n)}
    step = 0
    while step < depth:
        body = f.body
        if isinstance(body, V.Leaf):
            print("leaf reached"); return
        if isinstance(body, V.If):
            x, y = args[body.x], args[body.y]
            lo, hi = (y, x) if flip else (x, y)     # THEN branch: x <_d y
            closure_add(above, lo, hi)
            step += 1
            nxt = body.then
            comps, hasse = shape(args, above)
            print(f"{step:2d}: compare {x:2d}-{y:2d}  active={len(args):2d} "
                  f"(rank {f.doc_i})  components={comps}  hasse_edges={hasse}")
        else:
            nxt = body
        if isinstance(nxt, V.Leaf):
            print("leaf reached"); return
        args = [args[p] for p in nxt.args]
        flip = flip ^ nxt.flip
        f = nxt.callee

main()
