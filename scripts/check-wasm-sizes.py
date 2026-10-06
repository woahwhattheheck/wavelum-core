#!/usr/bin/env python3
import argparse
import json
from pathlib import Path


def collect_sizes(directory):
    if not directory or not directory.exists():
        return {}
    return {path.stem: path.stat().st_size for path in sorted(directory.glob("*.wasm"))}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", required=True, type=Path)
    parser.add_argument("--wasm-dir", required=True, type=Path)
    parser.add_argument("--baseline-dir", type=Path)
    parser.add_argument("--summary", required=True, type=Path)
    parser.add_argument("--json", required=True, type=Path)
    args = parser.parse_args()

    config = json.loads(args.config.read_text())
    default_kb = int(config["default_kb"])
    budgets = {name: int(limit) for name, limit in config["contracts"].items()}
    current = collect_sizes(args.wasm_dir)
    baseline = collect_sizes(args.baseline_dir)

    if not current:
        raise SystemExit(f"No WASM binaries found in {args.wasm_dir}")

    failures = []
    rows = []
    payload = {}

    for name, size_bytes in sorted(current.items()):
        budget_kb = budgets.get(name, default_kb)
        limit_bytes = budget_kb * 1000
        passed = size_bytes < limit_bytes
        if not passed:
            failures.append(f"{name}: {size_bytes} bytes >= {limit_bytes} byte budget")

        base_bytes = baseline.get(name)
        delta_bytes = None if base_bytes is None else size_bytes - base_bytes
        delta = "new" if delta_bytes is None else f"{delta_bytes:+d} B"
        rows.append(
            f"| {name} | {size_bytes / 1000:.1f} | < {budget_kb} | {delta} | {'PASS' if passed else 'FAIL'} |"
        )
        payload[name] = {
            "bytes": size_bytes,
            "kb": round(size_bytes / 1000, 3),
            "budget_kb_exclusive": budget_kb,
            "baseline_bytes": base_bytes,
            "delta_bytes": delta_bytes,
            "status": "pass" if passed else "fail",
        }

    missing = sorted(set(budgets) - set(current))
    if missing:
        failures.append("Expected budgeted contract WASM missing: " + ", ".join(missing))

    summary = "\n".join([
        "## WASM binary size budgets",
        "",
        "| Contract | Size (KB) | Budget | Change vs base | Status |",
        "|---|---:|---:|---:|---|",
        *rows,
        "",
        f"Default budget for contracts not explicitly listed: < {default_kb} KB.",
    ])
    if failures:
        summary += "\n\n### Budget violations\n" + "\n".join(f"- {item}" for item in failures)

    args.summary.parent.mkdir(parents=True, exist_ok=True)
    args.json.parent.mkdir(parents=True, exist_ok=True)
    args.summary.write_text(summary + "\n")
    args.json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(summary)

    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
