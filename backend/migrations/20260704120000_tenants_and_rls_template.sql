-- Platform owner tenants (ADR-002). RLS applies to tenant-scoped tables, not this root table.

CREATE SCHEMA IF NOT EXISTS shared;

CREATE TABLE shared.tenants (
    id          UUID PRIMARY KEY DEFAULT uuidv7(),
    name        VARCHAR(200) NOT NULL,
    active      BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_tenants_active ON shared.tenants (active) WHERE active = true;

-- RLS template: placeholder table demonstrating tenant isolation (ARCHITECTURE.md).
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

CREATE POLICY tenant_isolation ON shared.tenant_scoped_placeholder
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);
