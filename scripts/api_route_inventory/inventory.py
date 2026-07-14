"""Domain labels and stable inventory IDs for Phase 17."""

from __future__ import annotations

import re
from pathlib import Path

DOMAIN_RULES: list[tuple[str, str]] = [
    (r"^/health", "Health"),
    (r"^/v1/status$", "Health"),
    (r"^/v1/$", "Health"),
    (r"^/v1/auth/", "Auth"),
    (r"^/v1/users", "Users"),
    (r"^/v1/commerces", "Commerces"),
    (r"^/v1/products", "Catalog"),
    (r"^/v1/categories", "Catalog"),
    (r"^/v1/inventory", "Inventory"),
    (r"^/v1/sales", "Sales"),
    (r"^/v1/public/", "Public"),
    (r"^/v1/portal/", "Portal"),
    (r"^/v1/orders", "OrdersDeliveries"),
    (r"^/v1/deliveries", "OrdersDeliveries"),
    (r"^/v1/media", "Media"),
    (r"^/v1/reports", "Reports"),
    (r"^/v1/audit/", "AuditFraud"),
    (r"^/v1/fraud/", "AuditFraud"),
    (r"^/v1/billing/", "Billing"),
    (r"^/v1/platform/", "Platform"),
    (r"^/v1/settings", "Settings"),
]

CHECK_ROW = re.compile(
    r"^\| (T-17-\d+) \| `([A-Z]+)` \| `([^`]+)` \| ([^|]+) \| "
    r"(\[[ x]\]) \| (\[[ x]\]) \| (\[[ x]\]) \| (\[[ x]\]) \|",
    re.MULTILINE,
)


def domain_for(path: str) -> str:
    for pattern, domain in DOMAIN_RULES:
        if re.search(pattern, path):
            return domain
    return "Other"


def load_existing_inventory(
    path: Path,
) -> dict[tuple[str, str], tuple[str, str, str, str, str]]:
    """Map (method, path) -> (id, happy, authz, errors, journey)."""
    if not path.is_file():
        return {}
    text = path.read_text(encoding="utf-8")
    out: dict[tuple[str, str], tuple[str, str, str, str, str]] = {}
    for m in CHECK_ROW.finditer(text):
        tid, method, route, _domain, happy, authz, errors, journey = m.groups()
        out[(method, route)] = (tid, happy, authz, errors, journey)
    return out


def assign_ids(
    endpoints: list[tuple[str, str]],
    existing: dict[tuple[str, str], tuple[str, str, str, str, str]],
) -> list[tuple[str, str, str, str, str, str, str, str]]:
    """Return rows: id, method, path, domain, happy, authz, errors, journey."""
    used = {meta[0] for meta in existing.values()}
    next_n = 1
    rows: list[tuple[str, str, str, str, str, str, str, str]] = []
    blanks = ("[ ]", "[ ]", "[ ]", "[ ]")

    def next_id() -> str:
        nonlocal next_n
        while True:
            cand = f"T-17-{next_n:03d}"
            next_n += 1
            if cand not in used:
                used.add(cand)
                return cand

    ordered_keys = sorted(
        endpoints, key=lambda ep: existing.get(ep, (f"T-17-{9999}",))[0]
    )
    for method, path in ordered_keys:
        domain = domain_for(path)
        if (method, path) in existing:
            tid, happy, authz, errors, journey = existing[(method, path)]
            rows.append((tid, method, path, domain, happy, authz, errors, journey))
        else:
            rows.append((next_id(), method, path, domain, *blanks))
    rows.sort(key=lambda r: r[0])
    return rows
