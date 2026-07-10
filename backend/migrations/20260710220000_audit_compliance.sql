-- Phase 10: audit compliance (actor metadata, platform scope, LGPD exports).

ALTER TABLE audit.events
    ADD COLUMN IF NOT EXISTS actor_type VARCHAR(20) NOT NULL DEFAULT 'User'
        CHECK (actor_type IN ('PlatformAdmin', 'User')),
    ADD COLUMN IF NOT EXISTS ip VARCHAR(45);

ALTER TABLE audit.events
    ALTER COLUMN tenant_id DROP NOT NULL;

CREATE INDEX IF NOT EXISTS idx_audit_events_actor
    ON audit.events (actor_id);

CREATE INDEX IF NOT EXISTS idx_audit_events_action
    ON audit.events (action);

DROP POLICY IF EXISTS tenant_select ON audit.events;
CREATE POLICY tenant_select ON audit.events
    FOR SELECT
    USING (
        tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
        OR current_setting('app.bypass_rls', true) = 'true'
    );

DROP POLICY IF EXISTS tenant_insert ON audit.events;
CREATE POLICY tenant_insert ON audit.events
    FOR INSERT
    WITH CHECK (
        tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
        OR current_setting('app.bypass_rls', true) = 'true'
    );

CREATE TABLE ops.data_export_jobs (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    requested_by    UUID NOT NULL,
    actor_type      VARCHAR(20) NOT NULL CHECK (actor_type IN ('PlatformAdmin', 'User')),
    status          VARCHAR(20) NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'processing', 'completed', 'failed')),
    storage_bucket  VARCHAR(100),
    storage_key     TEXT,
    error_message   TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at    TIMESTAMPTZ
);

CREATE INDEX idx_data_export_jobs_tenant
    ON ops.data_export_jobs (tenant_id, created_at DESC);

GRANT SELECT, INSERT, UPDATE ON ops.data_export_jobs TO app_user;
