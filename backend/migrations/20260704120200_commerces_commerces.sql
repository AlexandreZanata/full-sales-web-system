-- Module 02-commerces: commerces (MIGRATION-SPEC-001-commerces)

CREATE SCHEMA IF NOT EXISTS commerces;

CREATE TABLE commerces.commerces (
    id          UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id   UUID NOT NULL REFERENCES shared.tenants (id),
    cnpj        VARCHAR(14) NOT NULL CHECK (cnpj ~ '^[0-9]{14}$'),
    legal_name  VARCHAR(200) NOT NULL CHECK (char_length(legal_name) >= 1),
    trade_name  VARCHAR(200) NOT NULL CHECK (char_length(trade_name) >= 1),
    address     JSONB NOT NULL,
    active      BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_commerces_tenant_cnpj UNIQUE (tenant_id, cnpj)
);

CREATE INDEX idx_commerces_tenant ON commerces.commerces (tenant_id);
CREATE INDEX idx_commerces_tenant_active ON commerces.commerces (tenant_id, active) WHERE active = true;

ALTER TABLE commerces.commerces ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerces.commerces FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON commerces.commerces
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON commerces.commerces
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON commerces.commerces
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
