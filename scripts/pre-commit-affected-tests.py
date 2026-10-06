#!/usr/bin/env python3
"""Run tests only for workspace packages affected by the staged Rust/Cargo files."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def run(command: list[str]) -> int:
    print("+", " ".join(command), flush=True)
    return subprocess.run(command, cwd=ROOT, check=False).returncode


def cargo_metadata() -> dict:
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    return json.loads(result.stdout)


def main(filenames: list[str]) -> int:
    paths = [Path(name) for name in filenames]
    normalized = {path.as_posix() for path in paths}

    # Workspace dependency changes can affect every member.
    if {"Cargo.toml", "Cargo.lock"} & normalized:
        return run(["cargo", "test", "--workspace"])

    metadata = cargo_metadata()
    packages: set[str] = set()
    unmatched_rust_or_manifest = False

    package_roots = [
        (Path(package["manifest_path"]).resolve().parent, package["name"])
        for package in metadata["packages"]
    ]

    for relative in paths:
        candidate = (ROOT / relative).resolve()
        if relative.suffix != ".rs" and relative.name != "Cargo.toml":
            continue

        owner = None
        for package_root, package_name in package_roots:
            try:
                candidate.relative_to(package_root)
            except ValueError:
                continue
            owner = package_name
            break

        if owner is None:
            unmatched_rust_or_manifest = True
        else:
            packages.add(owner)

    # Root-level/integration Rust files can span packages, so fail safe to workspace tests.
    if unmatched_rust_or_manifest:
        return run(["cargo", "test", "--workspace"])

    if not packages:
        print("No affected workspace package tests to run.")
        return 0

    for package in sorted(packages):
        code = run(["cargo", "test", "-p", package])
        if code != 0:
            return code
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
