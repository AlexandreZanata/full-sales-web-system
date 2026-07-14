#!/usr/bin/env python3
"""Verify docs/API-CONTRACT.md ↔ routes.rs drift (Phase 17A).

Usage (repo root):
  python3 scripts/verify-api-route-inventory.py
  python3 scripts/verify-api-route-inventory.py --write-docs
  python3 scripts/verify-api-route-inventory.py --self-test
"""

from __future__ import annotations

import argparse
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(Path(__file__).resolve().parent))

from api_route_inventory.classify import classify_all, summarize  # noqa: E402
from api_route_inventory.drift import diff_endpoints, load_allowlist  # noqa: E402
from api_route_inventory.generate import write_docs  # noqa: E402
from api_route_inventory.inventory import assign_ids, load_existing_inventory  # noqa: E402
from api_route_inventory.parse import load_contract, load_routes  # noqa: E402

CONTRACT = ROOT / "docs" / "API-CONTRACT.md"
ROUTES = ROOT / "backend" / "crates" / "api-http" / "src" / "routes.rs"
ALLOWLIST = ROOT / "scripts" / "api-route-inventory-allowlist.json"
TESTS = ROOT / "backend" / "crates" / "api-http" / "tests"
PHASE_DOCS = (
    ROOT
    / ".local"
    / "phases"
    / "17-backend-route-contract-coverage"
    / "documentation"
)
INVENTORY_MD = PHASE_DOCS / "ROUTE-INVENTORY.md"
GAP_MD = PHASE_DOCS / "GAP-BASELINE.md"


def run_verify(*, write_docs_flag: bool) -> int:
    contract = load_contract(CONTRACT)
    router = load_routes(ROUTES)
    allowlist = load_allowlist(ALLOWLIST)
    report = diff_endpoints(contract, router, allowlist)

    print(f"Contract endpoints: {len(contract)}")
    print(f"Router endpoints:   {len(router)}")
    if report.waived_missing:
        print(f"Waived (contract→router): {len(report.waived_missing)}")
        for m, p in report.waived_missing:
            print(f"  WAIVE missing router: {m} {p}")
    if report.waived_undocumented:
        print(f"Waived (router→contract): {len(report.waived_undocumented)}")
        for m, p in report.waived_undocumented:
            print(f"  WAIVE undocumented: {m} {p}")

    if report.missing_from_router:
        print("FAIL: in contract but not in routes.rs:")
        for m, p in report.missing_from_router:
            print(f"  - {m} {p}")
    if report.undocumented_in_router:
        print("FAIL: in routes.rs but not in API-CONTRACT.md:")
        for m, p in report.undocumented_in_router:
            print(f"  - {m} {p}")

    classes = classify_all(contract, TESTS)
    counts = summarize(classes)
    print(
        "Gap baseline:",
        ", ".join(f"{k}={counts.get(k, 0)}" for k in ("full", "partial", "smoke-only", "none")),
    )

    if write_docs_flag:
        existing = load_existing_inventory(INVENTORY_MD)
        rows = assign_ids(contract, existing)
        write_docs(INVENTORY_MD, GAP_MD, rows, contract, TESTS)
        print(f"Wrote {INVENTORY_MD.relative_to(ROOT)}")
        print(f"Wrote {GAP_MD.relative_to(ROOT)}")

    if report.has_failures:
        return 1
    print("OK: API route inventory drift check passed")
    return 0


def run_self_test() -> int:
    loader = unittest.TestLoader()
    suite = loader.discover(
        str(Path(__file__).resolve().parent / "api_route_inventory" / "tests"),
        pattern="test_*.py",
    )
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    return 0 if result.wasSuccessful() else 1


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--write-docs",
        action="store_true",
        help="Regenerate .local Phase 17 ROUTE-INVENTORY + GAP-BASELINE",
    )
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="Run parser/drift unit tests",
    )
    args = parser.parse_args(argv)
    if args.self_test:
        return run_self_test()
    return run_verify(write_docs_flag=args.write_docs)


if __name__ == "__main__":
    raise SystemExit(main())
