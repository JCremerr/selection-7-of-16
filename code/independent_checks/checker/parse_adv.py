import sys
import os; sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "verifier"))
import verify_rs_algorithm as V
base_head = "pub const N: usize = 2;\npub const I: usize = 0;\npub fn run(input: [usize; 2]) -> usize {\n    select_0(input)\n}\n/// n = 1\nfn select_1([a]: [usize; 1]) -> usize {\n    a\n}\n"
cases = {
 "ok min of 2": "/// n = 2, i = 0\nfn select_0([a, b]: [usize; 2]) -> usize {\n    if a < b {\n        select_1([a])\n    } else {\n        select_1([b])\n    }\n}\n",
 "CR inside line": "/// n = 2, i = 0\nfn select_0([a, b]: [usize; 2]) -> usize {\n    if a < b {\r        select_1([b])\n    } else {\n        select_1([b])\n    }\n}\n",
 "block comment hides then": "/// n = 2, i = 0\nfn select_0([a, b]: [usize; 2]) -> usize {\n    if a < b { /*\n        select_1([b])\n    } else { */\n        select_1([a])\n    }\n}\n",
 "let fresh name in tuple": "/// n = 2, i = 0\nfn select_0([a, b]: [usize; 2]) -> usize {\n    let (x, y) = if a < b {\n        (a, x)\n    } else {\n        (b, a)\n    };\n    select_1([x])\n}\n",
 "let shadow then leaf": "/// n = 2, i = 0\nfn select_0([a, b]: [usize; 2]) -> usize {\n    let (a, b) = if a < b {\n        (b, a)\n    } else {\n        (a, b)\n    };\n    b\n}\n",
 "let shadow wrong leaf": "/// n = 2, i = 0\nfn select_0([a, b]: [usize; 2]) -> usize {\n    let (a, b) = if a < b {\n        (b, a)\n    } else {\n        (a, b)\n    };\n    a\n}\n",
 "doc 4 slashes": "//// n = 2, i = 0\nfn select_0([a, b]: [usize; 2]) -> usize {\n    if a < b {\n        select_1([a])\n    } else {\n        select_1([b])\n    }\n}\n",
 "const I wrong": "/// n = 2, i = 1\nfn select_0([a, b]: [usize; 2]) -> usize {\n    if a < b {\n        select_1([b])\n    } else {\n        select_1([a])\n    }\n}\n",
 "call with trailing comma": "/// n = 2, i = 0\nfn select_0([a, b]: [usize; 2]) -> usize {\n    if a < b {\n        select_1([a,])\n    } else {\n        select_1([b])\n    }\n}\n",
 "le operator": "/// n = 2, i = 0\nfn select_0([a, b]: [usize; 2]) -> usize {\n    if a <= b {\n        select_1([a])\n    } else {\n        select_1([b])\n    }\n}\n",
}
for name, body in cases.items():
    r = V.verify_text(base_head + body)
    print(f"{name:28s} {'ACCEPT' if r['accepted'] else 'REJECT'} {r.get('worst_case')} {(r['error'] or '')[:90]}")
