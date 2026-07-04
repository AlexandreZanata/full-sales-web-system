-- Module 05-reports: signing_keys (MIGRATION-SPEC-001-signing-keys, ADR-004)

CREATE SCHEMA IF NOT EXISTS reports;

CREATE TABLE reports.signing_keys (
    public_key_id   VARCHAR(64) PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    public_key      BYTEA NOT NULL CHECK (octet_length(public_key) = 32),
    active          BOOLEAN NOT NULL DEFAULT true,
    valid_from      TIMESTAMPTZ NOT NULL,
    valid_until     TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_signing_keys_tenant ON reports.signing_keys (tenant_id);

ALTER TABLE reports.signing_keys ENABLE ROW LEVEL SECURITY;
ALTER TABLE reports.signing_keys FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON reports.signing_keys
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON reports.signing_keys
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON reports.signing_keys
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
