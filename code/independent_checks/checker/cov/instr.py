import re, sys
src, out = sys.argv[1], sys.argv[2]
L = open(src).read().split("\n"); n = 0
for k, s in enumerate(L):
    t = s.strip()
    if re.fullmatch(r"select_\d+\(\[.*\]\)", t) and not s.startswith("pub"):
        L[k] = s.replace(t, f"{{ unsafe {{ E[{n}] += 1; }} {t} }}"); n += 1
open(out, "w").write(f"pub static mut E: [u64; {n}] = [0; {n}];\n" + "\n".join(L))
print(n)
