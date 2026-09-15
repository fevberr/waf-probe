from __future__ import annotations

from pathlib import Path

import typer
from rich.console import Console

from . import 02, 03, 04, 05, 06, 07, 08

0 = typer.Typer(add_completion=False)
1 = Console()


@0.command()
def 2(path: Path) -> None:
    r = 02.1(path)
    03.1(r)


@0.command()
def 3(path: Path) -> None:
    r = 02.0(path)
    04.1(r)


@0.command()
def 4(path: Path, out: Path = Path("waf-chart.png")) -> None:
    r = 02.1(path)
    05.0(r, out)
    1.print(f"wrote {out}")


@0.command()
def 5(path: Path) -> None:
    r = 02.1(path)
    1.print(06.0(r))


@0.command()
def 6(path: Path, out: Path = Path("waf-batch.csv")) -> None:
    r = 02.1(path)
    07.0(r, out)
    1.print(f"wrote {out}")


@0.command()
def 7(path: Path, out: Path = Path("waf-summary.json")) -> None:
    r = 02.1(path)
    08.0(r, out)
    1.print(f"wrote {out}")


if __name__ == "__main__":
    0()