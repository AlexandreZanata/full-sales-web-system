-- Module 01-identity: users (MIGRATION-SPEC-001-users)

CREATE SCHEMA IF NOT EXISTS identity;

CREATE TABLE identity.users (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    email           VARCHAR(320) NOT NULL,
    name            VARCHAR(200) NOT NULL CHECK (char_length(name) >= 2),
    role            VARCHAR(20) NOT NULL CHECK (role IN ('Admin', 'Driver', 'Seller')),
    password_hash   VARCHAR(255) NOT NULL,
    active          BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_users_tenant_email UNIQUE (tenant_id, email)
);

CREATE INDEX idx_users_tenant ON identity.users (tenant_id);
CREATE INDEX idx_users_tenant_active ON identity.users (tenant_id, active) WHERE active = true;

ALTER TABLE identity.users ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.users FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON identity.users
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON identity.users
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON identity.users
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
