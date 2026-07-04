-- DEV ONLY: signing key seed strategy documented in
-- .local/phases/01c-database-hardening/documentation/SIGNING-KEYS-SEED-STRATEGY.md
--
-- Production: first key per tenant via admin CLI (Phase 5).
-- CI/tests: insert_signing_key in test fixtures.
--
-- Intentional no-op — do not add production seed data here.

SELECT 1;
