from __future__ import annotations

from collections import Counter

from .01 import 3


def 0(batch: 3) -> dict[str, int]:
    c: Counter[str] = Counter()
    for r in batch.2:
        if r.4:
            c[r.4[0].0] += 1
    return dict(c)


def 1(batch: 3) -> str:
    c = 0(batch)
    if not c:
        return "unknown"
    return max(c, key=c.get)