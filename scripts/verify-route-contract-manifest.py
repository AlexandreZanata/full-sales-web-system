#!/usr/bin/env python3
"""Fail if any Phase 17 inventory ID lacks a T-17-* marker in api-http tests.

Canonical list (committed): scripts/phase17-route-ids.txt
  Format: T-17-NNN METHOD /path
Regenerate: python3 scripts/verify-route-contract-manifest.py --write-manifest

Does not require .local/ on CI — uses the committed file as ID source of truth.
--write-manifest prefers .local ROUTE-INVENTORY when present.
"""

from __future__ import annotations

import argparse
import re
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TESTS = ROOT / "backend" / "crates" / "api-http" / "tests"
IDS_FILE = ROOT / "scripts" / "phase17-route-ids.txt"
INVENTORY = (
    ROOT
    / ".local"
    / "phases"
    / "17-backend-route-contract-coverage"
    / "documentation"
    / "ROUTE-INVENTORY.md"
)
CONTRACT = ROOT / "docs" / "API-CONTRACT.md"

sys.path.insert(0, str(ROOT / "scripts"))
from api_route_inventory.inventory import assign_ids, load_existing_inventory  # noqa: E402
from api_route_inventory.parse import load_contract  # noqa: E402

RANGE = re.compile(r"T-17-(\d+)\.\.(\d+)")
SINGLE = re.compile(r"T-17-(\d+)")
LINE = re.compile(r"^(T-17-\d+)\s+([A-Z]+)\s+(\S+)\s*$")


def expand_markers(text: str) -> set[str]:
    found: set[str] = set()
    for a, b in RANGE.findall(text):
        lo, hi = int(a), int(b)
        if lo > hi:
            lo, hi = hi, lo
        for n in range(lo, hi + 1):
            found.add(f"T-17-{n:03d}")
    for n in SINGLE.findall(text):
        found.add(f"T-17-{int(n):03d}")
    return found


def compute_rows_from_docs() -> list[tuple[str, str, str]]:
    endpoints = load_contract(CONTRACT)
    rows = assign_ids(endpoints, load_existing_inventory(INVENTORY))
    return [(r[0], r[1], r[2]) for r in rows]


def load_committed() -> list[tuple[str, str, str]]:
    if not IDS_FILE.is_file():
        return []
    out: list[tuple[str, str, str]] = []
    for raw in IDS_FILE.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        m = LINE.match(line)
        if not m:
            raise ValueError(f"bad manifest line: {raw!r}")
        out.append((m.group(1), m.group(2), m.group(3)))
    return out


def write_manifest(rows: list[tuple[str, str, str]]) -> None:
    body = [
        "# Phase 17 route contract IDs (committed).",
        "# Format: T-17-NNN METHOD /path",
        "# Regenerate: python3 scripts/verify-route-contract-manifest.py --write-manifest",
        *[f"{tid} {method} {path}" for tid, method, path in rows],
        "",
    ]
    IDS_FILE.write_text("\n".join(body), encoding="utf-8")


def scan_tests(tests_dir: Path) -> set[str]:
    found: set[str] = set()
    for path in tests_dir.rglob("*.rs"):
        found |= expand_markers(path.read_text(encoding="utf-8", errors="ignore"))
    return found


def run_verify(*, write_manifest_flag: bool) -> int:
    contract = set(load_contract(CONTRACT))

    if write_manifest_flag:
        rows = compute_rows_from_docs()
        write_manifest(rows)
        print(f"Wrote {IDS_FILE.relative_to(ROOT)} ({len(rows)} ids)")

    committed = load_committed()
    if not committed:
        print(f"FAIL: missing {IDS_FILE.relative_to(ROOT)} — run with --write-manifest")
        return 1

    committed_eps = {(m, p) for _, m, p in committed}
    if committed_eps != contract:
        print("FAIL: phase17-route-ids.txt endpoints drift vs docs/API-CONTRACT.md")
        for m, p in sorted(contract - committed_eps)[:20]:
            print(f"  missing from manifest: {m} {p}")
        for m, p in sorted(committed_eps - contract)[:20]:
            print(f"  extra in manifest: {m} {p}")
        return 1

    found = scan_tests(TESTS)
    missing = [tid for tid, _, _ in committed if tid not in found]
    print(f"Inventory IDs: {len(committed)}")
    print(f"Markers found: {len(found & {t for t, _, _ in committed})}")
    if missing:
        print("FAIL: inventory IDs with no T-17-* test marker:")
        for tid in missing:
            print(f"  - {tid}")
        return 1
    print("OK: phase17 route-id manifest + test markers")
    return 0


class ExpandMarkersTest(unittest.TestCase):
    def test_range_and_single(self) -> None:
        text = "// T-17-115..117 / T-17-150\n// T-17-001"
        self.assertEqual(
            expand_markers(text),
            {"T-17-001", "T-17-115", "T-17-116", "T-17-117", "T-17-150"},
        )


def main(argv: list[str] | None = None) -> int:
    raw = argv if argv is not None else sys.argv[1:]
    raw = [a for a in raw if a != "--"]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument(
        "--write-manifest",
        action="store_true",
        help="Rewrite scripts/phase17-route-ids.txt from contract/inventory",
    )
    args = parser.parse_args(raw)
    if args.self_test:
        suite = unittest.defaultTestLoader.loadTestsFromTestCase(ExpandMarkersTest)
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        return 0 if result.wasSuccessful() else 1
    return run_verify(write_manifest_flag=args.write_manifest)


if __name__ == "__main__":
    raise SystemExit(main())
