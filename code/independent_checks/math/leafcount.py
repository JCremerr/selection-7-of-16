import re, sys
from functools import lru_cache
src = open(sys.argv[1]).read()
funcs = {}
for m in re.finditer(r'\nfn (select_\d+)\(\[[^\]]*\]: \[usize; (\d+)\]\) -> usize \{\n(.*?)\n\}\n', src, re.S):
    name, n, body = m.group(1), int(m.group(2)), m.group(3)
    funcs[name] = (n, body)
print("functions parsed:", len(funcs))
kinds = {}
for name,(n,body) in funcs.items():
    b = body.strip()
    calls = re.findall(r'(select_\d+)\(', b)
    ncmp = len(re.findall(r'\bif\b', b))
    if b.startswith('let'):
        assert ncmp == 1 and len(calls) == 1, (name, b)
        kinds[name] = ('let', calls)
    elif b.startswith('if'):
        assert ncmp == 1 and len(calls) == 2, (name, b)
        kinds[name] = ('if', calls)
    else:
        assert ncmp == 0 and len(calls) == 0 and n == 1, (name, b)
        kinds[name] = ('leaf', [])
sys.setrecursionlimit(10000)
@lru_cache(None)
def L(f):
    k, c = kinds[f]
    if k == 'leaf': return 1
    if k == 'let': return 2*L(c[0])
    return L(c[0]) + L(c[1])
@lru_cache(None)
def D(f):
    k, c = kinds[f]
    if k == 'leaf': return 0
    return 1 + max(D(x) for x in c)
from collections import Counter
@lru_cache(None)
def prof(f):
    k, c = kinds[f]
    if k == 'leaf': return ((0,1),)
    out = Counter()
    for x in (c if k=='if' else c*2):
        for d,cnt in prof(x): out[d+1]+=cnt
    return tuple(sorted(out.items()))
print("leaves", L('select_0'), "< 2^28 =", 2**28, L('select_0') < 2**28, "max depth", D('select_0'))
print("depth profile", prof('select_0'))
