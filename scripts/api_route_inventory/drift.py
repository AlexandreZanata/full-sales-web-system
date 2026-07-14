"""Diff contract ↔ router and apply dated allowlist waivers."""

from __future__ import annotations

import json
from dataclasses import dataclass
from datetime import date
from pathlib import Path


@dataclass(frozen=True)
class DriftReport:
    missing_from_router: list[tuple[str, str]]
    undocumented_in_router: list[tuple[str, str]]
    waived_missing: list[tuple[str, str]]
    waived_undocumented: list[tuple[str, str]]

    @property
    def has_failures(self) -> bool:
        return bool(self.missing_from_router or self.undocumented_in_router)


def load_allowlist(path: Path | None) -> dict:
    if path is None or not path.is_file():
        return {"waivers": []}
    return json.loads(path.read_text(encoding="utf-8"))


def _active_waivers(allowlist: dict, today: date) -> list[dict]:
    active = []
    for w in allowlist.get("waivers", []):
        expires = date.fromisoformat(w["expires"])
        if expires < today:
            raise SystemExit(
                f"Expired waiver {w['method']} {w['path']} (expires {w['expires']})"
            )
        active.append(w)
    return active


def diff_endpoints(
    contract: list[tuple[str, str]],
    router: list[tuple[str, str]],
    allowlist: dict,
    today: date | None = None,
) -> DriftReport:
    today = today or date.today()
    waivers = _active_waivers(allowlist, today)
    waive_missing = {
        (w["method"], w["path"])
        for w in waivers
        if w.get("direction") == "contract_missing_router"
    }
    waive_extra = {
        (w["method"], w["path"])
        for w in waivers
        if w.get("direction") == "router_missing_contract"
    }
    cs, rs = set(contract), set(router)
    raw_missing = sorted(cs - rs)
    raw_extra = sorted(rs - cs)
    waived_m = [ep for ep in raw_missing if ep in waive_missing]
    waived_e = [ep for ep in raw_extra if ep in waive_extra]
    return DriftReport(
        missing_from_router=[ep for ep in raw_missing if ep not in waive_missing],
        undocumented_in_router=[ep for ep in raw_extra if ep not in waive_extra],
        waived_missing=waived_m,
        waived_undocumented=waived_e,
    )
