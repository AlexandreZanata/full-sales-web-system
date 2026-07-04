-- Module 00-shared: tenants (MIGRATION-SPEC-001-tenants, ADR-002)

CREATE SCHEMA IF NOT EXISTS shared;

CREATE TABLE shared.tenants (
    id          UUID PRIMARY KEY DEFAULT uuidv7(),
    name        VARCHAR(200) NOT NULL CHECK (char_length(name) >= 1),
    active      BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_tenants_active ON shared.tenants (active) WHERE active = true;

-- RLS template placeholder (Phase 1 foundation) — superseded by module tables in later migrations.
CREATE TABLE shared.tenant_scoped_placeholder (
    id          UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id   UUID NOT NULL REFERENCES shared.tenants (id),
    label       TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_tenant_scoped_placeholder_tenant
    ON shared.tenant_scoped_placeholder (tenant_id);

ALTER TABLE shared.tenant_scoped_placeholder ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared.tenant_scoped_placeholder FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON shared.tenant_scoped_placeholder
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON shared.tenant_scoped_placeholder
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON shared.tenant_scoped_placeholder
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
