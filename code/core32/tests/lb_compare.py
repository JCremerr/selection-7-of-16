"""Compare the Rust LB_best port (src/lbbest.rs) with the Python reference
bounds_fast (code/analysis/lb_candidates.py): exact equality of C, FG, FGdual
and KEY on every line of a `lbcheck dump` file.

usage: python3 -I tests/lb_compare.py DUMPFILE
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "..", "analysis"))
from lb_candidates import bounds_fast  # noqa: E402


def main():
    path = sys.argv[1]
    rows = bad = 0
    by_n = {}
    with open(path) as f:
        for line in f:
            t = line.split()
            n, i = int(t[0]), int(t[1])
            up = [int(x, 16) for x in t[2].split(",")]
            c, fg, fgd, key = (int(x) for x in t[3:7])
            r = bounds_fast(n, i, up)
            got = (r["C"], r["FG"], r["FGdual"], r["key"])
            if got != (c, fg, fgd, key):
                bad += 1
                if bad <= 10:
                    print("MISMATCH", line.strip(), "python:", got)
            rows += 1
            by_n[n] = by_n.get(n, 0) + 1
    print(f"{rows} problems compared, {bad} mismatches; per n: "
          + " ".join(f"{k}:{v}" for k, v in sorted(by_n.items())))
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
