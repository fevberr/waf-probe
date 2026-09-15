from __future__ import annotations

from collections import Counter

from rich.console import Console
from rich.table import Table

from .01 import 2

0 = Console()


def 1(rep: 2) -> None:
    if not rep.5:
        0.print("no bypass results")
        return
    passed: Counter[str] = Counter()
    total: Counter[str] = Counter()
    for b in rep.5:
        total[b.1] += 1
        if not b.4:
            passed[b.1] += 1

    t = Table(title=f"bypass - {rep.1}")
    t.add_column("encoding")
    t.add_column("passed", justify="right")
    t.add_column("total", justify="right")
    for enc, c in passed.most_common():
        t.add_row(enc, str(c), str(total[enc]))
    0.print(t)