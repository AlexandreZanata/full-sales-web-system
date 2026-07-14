"""Classify existing api-http tests against inventory endpoints."""

from __future__ import annotations

import re
from collections import Counter
from pathlib import Path

ClassName = str  # none | smoke-only | partial | full


def _needles(path: str) -> list[str]:
    """Search strings likely present in tests for this route."""
    out = [path]
    # brace-free form used in format!(...) templates
    out.append(re.sub(r"\{[^}]+\}", "{id}", path))
    static = [p for p in path.split("/") if p and not p.startswith("{")]
    if len(static) >= 2:
        out.append("/".join(static[-3:]))
    return list(dict.fromkeys(out))


def classify_endpoint(method: str, path: str, files: dict[Path, str]) -> ClassName:
    matched: list[Path] = []
    needles = _needles(path)
    for fpath, text in files.items():
        if any(n and n in text for n in needles):
            # Prefer method+path co-occurrence when method appears nearby
            if method in text or f'"{method}"' in text or f"('{method}'" in text:
                matched.append(fpath)
            elif path in text or needles[0] in text:
                matched.append(fpath)
    if not matched:
        return "none"
    names = {p.name for p in matched}
    if names <= {"route_smoke.rs"}:
        return "smoke-only"
    # Full only when explicit Phase-17 inventory marker present
    for p in matched:
        if re.search(rf"//\s*T-17-\d+", files[p]):
            return "full"
    return "partial"


def load_test_files(tests_dir: Path) -> dict[Path, str]:
    return {p: p.read_text(encoding="utf-8") for p in tests_dir.rglob("*.rs")}


def classify_all(
    endpoints: list[tuple[str, str]], tests_dir: Path
) -> dict[tuple[str, str], ClassName]:
    files = load_test_files(tests_dir)
    return {ep: classify_endpoint(ep[0], ep[1], files) for ep in endpoints}


def summarize(classes: dict[tuple[str, str], ClassName]) -> Counter:
    return Counter(classes.values())
