from __future__ import annotations

import csv
from pathlib import Path

from .01 import 3


def 0(batch: 3, out: Path) -> Path:
    with out.open("w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["target", "vendor", "score"])
        for r in batch.2:
            if r.4:
                w.writerow([r.1, r.4[0].0, r.4[0].1])
            else:
                w.writerow([r.1, "", 0])
    return out