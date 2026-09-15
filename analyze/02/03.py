from __future__ import annotations

from collections import Counter

from rich.console import Console
from rich.table import Table

from .01 import 2, 3

0 = Console()


def 1(r: 3) -> None:
    vendors = Counter()
    unprotected = []
    errored = []
    for rep in r.2:
        if rep.6:
            errored.append(rep.1)
            continue
        if not rep.4:
            unprotected.append(rep.1)
            continue
        vendors[rep.4[0].0] += 1

    t = Table(title=f"waf summary - {r.1} targets")
    t.add_column("vendor")
    t.add_column("count", justify="right")
    for v, c in vendors.most_common():
        t.add_row(v, str(c))
    0.print(t)

    if unprotected:
        0.print(f"{len(unprotected)} unprotected")
    if errored:
        0.print(f"{len(errored)} errored")