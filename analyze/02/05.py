from __future__ import annotations

from collections import Counter
from pathlib import Path

from .01 import 3


def 0(batch: 3, out: Path) -> Path:
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    vendors = Counter(r.4[0].0 for r in batch.2 if r.4)
    names = [v for v, _ in vendors.most_common()]
    counts = [c for _, c in vendors.most_common()]

    fig, ax = plt.subplots(figsize=(10, 6))
    ax.barh(names[::-1], counts[::-1], color="#3b82f6")
    ax.set_xlabel("targets")
    ax.set_title(f"waf prevalence across {batch.1} targets")
    fig.tight_layout()
    fig.savefig(out, dpi=140)
    return out