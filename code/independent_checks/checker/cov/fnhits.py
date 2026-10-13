import re, sys
src, out = sys.argv[1], sys.argv[2]
L = open(src).read().split("\n"); names = []
for k, s in enumerate(L):
    m = re.match(r"fn (select_\d+)\(", s)
    if m:
        L[k] = s + f" unsafe {{ E[{len(names)}] += 1; }}"; names.append(m.group(1))
open(out, "w").write(f"pub static mut E: [u64; {len(names)}] = [0; {len(names)}];\n" + "\n".join(L))
open(out + ".names", "w").write("\n".join(names))
