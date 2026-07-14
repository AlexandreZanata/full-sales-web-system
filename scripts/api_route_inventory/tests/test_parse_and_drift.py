"""Contract-first unit tests for route inventory parsers and drift."""

from __future__ import annotations

import unittest
from datetime import date
from pathlib import Path

from api_route_inventory.drift import diff_endpoints
from api_route_inventory.parse import parse_contract, parse_routes_rs

HERE = Path(__file__).resolve().parents[1]
DATA = HERE / "testdata"


class ParseContractTests(unittest.TestCase):
    def test_given_contract_headings_when_parse_then_methods_and_paths(self):
        text = (DATA / "contract_sample.md").read_text(encoding="utf-8")
        got = parse_contract(text)
        self.assertEqual(
            got,
            [
                ("GET", "/health"),
                ("GET", "/v1/users/{id}"),
                ("POST", "/v1/users"),
            ],
        )


class ParseRoutesTests(unittest.TestCase):
    def test_given_route_chains_when_parse_then_expands_methods(self):
        text = (DATA / "routes_sample.rs").read_text(encoding="utf-8")
        got = parse_routes_rs(text)
        self.assertIn(("GET", "/health"), got)
        self.assertIn(("POST", "/v1/users"), got)
        self.assertIn(("GET", "/v1/users"), got)
        self.assertIn(("DELETE", "/v1/categories/{id}"), got)
        self.assertIn(("PATCH", "/v1/categories/{id}"), got)
        self.assertNotIn(("GET", "/v1/ghost"), got)


class DriftTests(unittest.TestCase):
    def test_given_mismatch_when_diff_then_reports_both_sides(self):
        contract = [("GET", "/a"), ("POST", "/b")]
        router = [("GET", "/a"), ("GET", "/c")]
        report = diff_endpoints(contract, router, {"waivers": []})
        self.assertEqual(report.missing_from_router, [("POST", "/b")])
        self.assertEqual(report.undocumented_in_router, [("GET", "/c")])
        self.assertTrue(report.has_failures)

    def test_given_dated_waiver_when_diff_then_suppresses_failure(self):
        contract = [("POST", "/v1/billing/subscription/cancel")]
        router: list[tuple[str, str]] = []
        allowlist = {
            "waivers": [
                {
                    "method": "POST",
                    "path": "/v1/billing/subscription/cancel",
                    "direction": "contract_missing_router",
                    "expires": "2099-01-01",
                }
            ]
        }
        report = diff_endpoints(
            contract, router, allowlist, today=date(2026, 7, 14)
        )
        self.assertEqual(report.missing_from_router, [])
        self.assertEqual(
            report.waived_missing,
            [("POST", "/v1/billing/subscription/cancel")],
        )
        self.assertFalse(report.has_failures)


if __name__ == "__main__":
    unittest.main()
