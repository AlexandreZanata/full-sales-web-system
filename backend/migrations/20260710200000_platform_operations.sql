-- Phase 8: platform operations (maintenance windows, last login tracking).

ALTER TABLE identity.users
    ADD COLUMN IF NOT EXISTS last_login_at TIMESTAMPTZ;

CREATE SCHEMA IF NOT EXISTS ops;

CREATE TABLE ops.maintenance_windows (
    id          UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id   UUID REFERENCES shared.tenants (id),
    message     TEXT NOT NULL CHECK (char_length(message) >= 3),
    starts_at   TIMESTAMPTZ NOT NULL,
    ends_at     TIMESTAMPTZ NOT NULL CHECK (ends_at > starts_at),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_maintenance_active_global
    ON ops.maintenance_windows (starts_at, ends_at)
    WHERE tenant_id IS NULL;

CREATE INDEX idx_maintenance_active_tenant
    ON ops.maintenance_windows (tenant_id, starts_at, ends_at)
    WHERE tenant_id IS NOT NULL;

ALTER TABLE identity.users ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.users FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_update ON identity.users;
CREATE POLICY tenant_update ON identity.users
    FOR UPDATE
    USING (
        tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
        OR current_setting('app.bypass_rls', true) = 'true'
    )
    WITH CHECK (
        tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
        OR current_setting('app.bypass_rls', true) = 'true'
    );

GRANT USAGE ON SCHEMA ops TO app_user;
GRANT SELECT, INSERT ON ops.maintenance_windows TO app_user;
