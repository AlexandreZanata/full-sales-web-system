-- Module 05-reports: active signing key lookup (MIGRATION-SPEC-003-signing-keys-active-index)

CREATE INDEX idx_signing_keys_tenant_active
    ON reports.signing_keys (tenant_id, active)
    WHERE active = true;
