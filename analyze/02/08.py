from __future__ import annotations

import json
from pathlib import Path

from .01 import 3


def 0(batch: 3, out: Path) -> Path:
    data = {
        "version": batch.0,
        "count": batch.1,
        "vendors": {},
    }
    for r in batch.2:
        if r.4:
            v = r.4[0].0
            data["vendors"].setdefault(v, 0)
            data["vendors"][v] += 1
    out.write_text(json.dumps(data, indent=2))
    return out