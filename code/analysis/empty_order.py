"""At the empty order (no comparisons known): pairs (n, i), n <= 16,
i <= ceil(n/2), where ceil(log2 Phi*) beats the one-sided bound
max(ceil log2 FG, ceil log2 FGdual), and the value of both for (16, 8).

  python3 -I empty_order.py
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lb_candidates as L  # noqa: E402

pairs = []
for n in range(1, 17):
    for i in range(1, (n + 1) // 2 + 1):
        up = [0] * n
        one = max(L._ceil_log2(L.phi_fg(n, i, up)),
                  L._ceil_log2(L.phi_fg(n, n + 1 - i, L.reverse(n, up))))
        phi = L.lb_fgsum(n, i, up)
        if phi > one:
            pairs.append((n, i, phi - one))
        if (n, i) == (16, 8):
            print(f"(16, 8): ceil log2 Phi* = {phi}, one-sided bound = {one}")
print(f"{len(pairs)} pairs (n, i, improvement):", pairs)
