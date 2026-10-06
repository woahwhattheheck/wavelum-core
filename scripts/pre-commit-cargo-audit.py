#!/usr/bin/env python3
"""Run cargo-audit as an informational pre-commit check."""

from __future__ import annotations

import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    if shutil.which("cargo-audit") is None:
        print(
            "cargo-audit is not installed; skipping this informational check. "
            "Install it with: cargo install cargo-audit --locked"
        )
        return 0

    result = subprocess.run(["cargo", "audit"], cwd=ROOT, check=False)
    if result.returncode != 0:
        print(
            "cargo audit reported advisory findings. "
            "This hook is informational and does not block the commit."
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
