#!/usr/bin/env python3
"""Run bounded Stellar testnet initialization and claim-path smoke probes."""

from __future__ import annotations

import argparse
import json
import subprocess
from typing import Sequence


def parse_command(value: str, label: str, account: str) -> list[str]:
    try:
        parsed = json.loads(value)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"{label} must be valid JSON: {exc}") from exc

    if not isinstance(parsed, list) or not all(isinstance(item, str) for item in parsed):
        raise SystemExit(f"{label} must be a JSON array of strings")

    return [account if item == "__SOURCE_ACCOUNT__" else item for item in parsed]


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

    print(f"{label}: {contract_args[0]} (send={send}, resource-fee ceiling={max_resource_fee} stroops)")
    subprocess.run(
        [
            "stellar",
            "contract",
            "invoke",
            "--contract-id",
            contract_id,
            "--source-account",
            source,
            "--network",
            network,
            "--resource-fee",
            str(max_resource_fee),
            "--send",
            send,
            "--cost",
            "--",
            *contract_args,
        ],
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
    args = parser.parse_args()

    if args.max_resource_fee <= 0:
        raise SystemExit("--max-resource-fee must be positive")

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
