-- Module 05-reports: reports (MIGRATION-SPEC-002-reports) — immutable after insert

CREATE TABLE reports.reports (
    id                  UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id           UUID NOT NULL REFERENCES shared.tenants (id),
    report_type         VARCHAR(30) NOT NULL CHECK (
        report_type IN ('DailyDriver', 'CommercePeriod', 'Consolidated')
    ),
    period_start        TIMESTAMPTZ NOT NULL,
    period_end          TIMESTAMPTZ NOT NULL CHECK (period_end >= period_start),
    filters             JSONB,
    canonical_payload   TEXT NOT NULL,
    signature           BYTEA NOT NULL CHECK (octet_length(signature) = 64),
    public_key_id       VARCHAR(64) NOT NULL REFERENCES reports.signing_keys (public_key_id),
    generated_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reports_tenant ON reports.reports (tenant_id);
CREATE INDEX idx_reports_tenant_period ON reports.reports (tenant_id, period_start, period_end);

ALTER TABLE reports.reports ENABLE ROW LEVEL SECURITY;
ALTER TABLE reports.reports FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON reports.reports
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON reports.reports
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
