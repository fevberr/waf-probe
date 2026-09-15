from __future__ import annotations

from pathlib import Path


def 0(p: Path) -> bool:
    return p.exists() and p.suffix == ".json"


def 1(p: Path) -> str:
    return p.stem