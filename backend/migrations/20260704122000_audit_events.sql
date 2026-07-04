-- Module 06-audit: audit events (MIGRATION-SPEC-001-audit-events)

CREATE SCHEMA IF NOT EXISTS audit;

CREATE TABLE audit.events (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    actor_id        UUID NOT NULL,
    action          VARCHAR(100) NOT NULL CHECK (char_length(action) >= 1),
    resource_type   VARCHAR(50) NOT NULL CHECK (char_length(resource_type) >= 1),
    resource_id     UUID NOT NULL,
    metadata        JSONB,
    correlation_id  UUID,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_audit_events_tenant_created
    ON audit.events (tenant_id, created_at DESC);

ALTER TABLE audit.events ENABLE ROW LEVEL SECURITY;
ALTER TABLE audit.events FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON audit.events
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON audit.events
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

REVOKE ALL ON audit.events FROM app_user;
GRANT SELECT, INSERT ON audit.events TO app_user;

CREATE OR REPLACE FUNCTION audit.prevent_audit_event_mutation()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'audit events are append-only';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_audit_event_update_delete
    BEFORE UPDATE OR DELETE ON audit.events
    FOR EACH ROW EXECUTE FUNCTION audit.prevent_audit_event_mutation();

GRANT USAGE ON SCHEMA audit TO app_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA audit GRANT SELECT, INSERT ON TABLES TO app_user;
