"""Parse API-CONTRACT.md headings and routes.rs `.route(...)` registrations."""

from __future__ import annotations

import re
from pathlib import Path

CONTRACT_HEADING = re.compile(
    r"^### `(GET|POST|PUT|PATCH|DELETE) (/[^`]+)`\s*$",
    re.MULTILINE,
)
ROUTE_CALL = re.compile(r"\.route\s*\(")
METHOD_CALL = re.compile(r"(?:axum::routing::)?(get|post|put|patch|delete)\s*\(")


def parse_contract(text: str) -> list[tuple[str, str]]:
    """Return unique (METHOD, path) from contract `###` headings."""
    found = CONTRACT_HEADING.findall(text)
    return sorted(set(found))


def _strip_line_comments(src: str) -> str:
    out: list[str] = []
    for line in src.splitlines():
        in_str = False
        buf: list[str] = []
        i = 0
        while i < len(line):
            c = line[i]
            if c == '"' and (i == 0 or line[i - 1] != "\\"):
                in_str = not in_str
                buf.append(c)
            elif not in_str and c == "/" and i + 1 < len(line) and line[i + 1] == "/":
                break
            else:
                buf.append(c)
            i += 1
        out.append("".join(buf))
    return "\n".join(out)


def _matching_close(src: str, start: int) -> int:
    depth = 1
    j = start
    while j < len(src) and depth:
        if src[j] == "(":
            depth += 1
        elif src[j] == ")":
            depth -= 1
        j += 1
    return j


def _methods_in_handler_expr(expr: str) -> list[str]:
    methods: list[str] = []
    k = 0
    while k < len(expr):
        m = METHOD_CALL.match(expr, k)
        if m and (k == 0 or expr[k - 1] in " .\n\t"):
            methods.append(m.group(1).upper())
            k = _matching_close(expr, m.end())
            continue
        k += 1
    return methods


def parse_routes_rs(text: str) -> list[tuple[str, str]]:
    """Return unique (METHOD, path) from Axum `.route("path", ...)` chains."""
    src = _strip_line_comments(text)
    endpoints: list[tuple[str, str]] = []
    i = 0
    while True:
        m = ROUTE_CALL.search(src, i)
        if not m:
            break
        start = m.end()
        end = _matching_close(src, start)
        args = src[start : end - 1]
        i = end
        pm = re.match(r'\s*"([^"]+)"\s*,\s*(.*)$', args, re.S)
        if not pm:
            continue
        path, rest = pm.group(1), pm.group(2)
        for method in _methods_in_handler_expr(rest):
            endpoints.append((method, path))
    return sorted(set(endpoints))


def load_contract(path: Path) -> list[tuple[str, str]]:
    return parse_contract(path.read_text(encoding="utf-8"))


def load_routes(path: Path) -> list[tuple[str, str]]:
    return parse_routes_rs(path.read_text(encoding="utf-8"))
