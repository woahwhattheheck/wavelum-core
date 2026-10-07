#!/usr/bin/env python3
"""Run bounded Stellar testnet initialization and claim-path smoke probes."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import time
from typing import Sequence


RESOURCE_FEE_PATTERN = re.compile(r"min_resource_fee:\s*([0-9_]+)")


def parse_command(value: str, label: str, account: str) -> list[str]:
    try:
        parsed = json.loads(value)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"{label} must be valid JSON: {exc}") from exc

    if not isinstance(parsed, list) or not all(isinstance(item, str) for item in parsed):
        raise SystemExit(f"{label} must be a JSON array of strings")

    return [account if item == "__SOURCE_ACCOUNT__" else item for item in parsed]


def command(
    *,
    contract_id: str,
    source: str,
    network: str,
    contract_args: Sequence[str],
    send: str,
    cost: bool,
) -> list[str]:
    args = [
        "stellar",
        "contract",
        "invoke",
        "--contract-id",
        contract_id,
        "--source-account",
        source,
        "--network",
        network,
        "--send",
        send,
    ]
    if cost:
        args.append("--cost")
    return [*args, "--", *contract_args]


def simulated_resource_fee(
    *,
    contract_id: str,
    source: str,
    network: str,
    contract_args: Sequence[str],
    label: str,
) -> int:
    result = subprocess.run(
        command(
            contract_id=contract_id,
            source=source,
            network=network,
            contract_args=contract_args,
            send="no",
            cost=True,
        ),
        check=True,
        capture_output=True,
        text=True,
    )
    combined = "\n".join((result.stdout, result.stderr))
    match = RESOURCE_FEE_PATTERN.search(combined)
    if match is None:
        raise SystemExit(f"{label}: Stellar CLI cost output did not report min_resource_fee")

    fee = int(match.group(1).replace("_", ""))
    print(f"{label}: simulated minimum resource fee={fee} stroops")
    return fee


def invoke(
    *,
    contract_id: str,
    source: str,
    network: str,
    max_resource_fee: int,
    contract_args: Sequence[str],
    send: str,
    label: str,
) -> None:
    if not contract_args:
        print(f"{label}: skipped (no contract arguments configured)")
        return

    simulated_fee = simulated_resource_fee(
        contract_id=contract_id,
        source=source,
        network=network,
        contract_args=contract_args,
        label=label,
    )
    if simulated_fee > max_resource_fee:
        raise SystemExit(
            f"{label}: simulated minimum resource fee {simulated_fee} exceeds "
            f"budget {max_resource_fee} stroops"
        )

    print(
        f"{label}: {contract_args[0]} "
        f"(send={send}, simulated fee={simulated_fee}, budget={max_resource_fee} stroops)"
    )
    if send == "no":
        return

    subprocess.run(
        command(
            contract_id=contract_id,
            source=source,
            network=network,
            contract_args=contract_args,
            send=send,
            cost=False,
        ),
        check=True,
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--contract-id", required=True)
    parser.add_argument("--source", required=True)
    parser.add_argument("--account", required=True)
    parser.add_argument("--network", default="testnet")
    parser.add_argument("--max-resource-fee", required=True, type=int)
    parser.add_argument("--init-json", default="[]")
    parser.add_argument("--claim-json", default="[]")
    parser.add_argument("--claim-send", choices=("yes", "no"), default="no")
    parser.add_argument("--claim-wait-seconds", type=int, default=0)
    args = parser.parse_args()

    if args.max_resource_fee <= 0:
        raise SystemExit("--max-resource-fee must be positive")
    if args.claim_wait_seconds < 0:
        raise SystemExit("--claim-wait-seconds must not be negative")

    init_args = parse_command(args.init_json, "--init-json", args.account)
    claim_args = parse_command(args.claim_json, "--claim-json", args.account)

    invoke(
        contract_id=args.contract_id,
        source=args.source,
        network=args.network,
        max_resource_fee=args.max_resource_fee,
        contract_args=init_args,
        send="yes",
        label="initialization probe",
    )
    if claim_args and args.claim_wait_seconds:
        print(
            f"claim-path probe: waiting {args.claim_wait_seconds}s "
            "for the testnet ledger to advance"
        )
        time.sleep(args.claim_wait_seconds)

    invoke(
        contract_id=args.contract_id,
        source=args.source,
        network=args.network,
        max_resource_fee=args.max_resource_fee,
        contract_args=claim_args,
        send=args.claim_send,
        label="claim-path probe",
    )


if __name__ == "__main__":
    main()
