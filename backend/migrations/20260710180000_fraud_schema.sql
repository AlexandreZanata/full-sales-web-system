-- Phase 6: fraud events, blocklist, tenant fraud scores (ADR-016 RLS bypass for PlatformAdmin).

CREATE SCHEMA IF NOT EXISTS fraud;

CREATE TABLE fraud.fraud_events (
    id                  UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id           UUID REFERENCES shared.tenants (id),
    user_id             UUID,
    event_type          VARCHAR(50) NOT NULL,
    severity            VARCHAR(20) NOT NULL CHECK (
        severity IN ('Low', 'Medium', 'High', 'Critical')
    ),
    status              VARCHAR(20) NOT NULL DEFAULT 'Open' CHECK (
        status IN ('Open', 'Reviewed', 'Blocked')
    ),
    resolution          VARCHAR(20) CHECK (
        resolution IS NULL OR resolution IN ('blocked', 'whitelisted', 'dismissed')
    ),
    resolution_note     TEXT,
    metadata            JSONB NOT NULL DEFAULT '{}'::jsonb,
    reviewed_by         UUID,
    reviewed_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_fraud_events_tenant_created
    ON fraud.fraud_events (tenant_id, created_at DESC)
    WHERE tenant_id IS NOT NULL;

CREATE INDEX idx_fraud_events_status_created
    ON fraud.fraud_events (status, created_at DESC);

CREATE TABLE fraud.blocklist_entries (
    id                  UUID PRIMARY KEY DEFAULT uuidv7(),
    email               VARCHAR(255),
    cnpj                VARCHAR(14),
    ip                  VARCHAR(45),
    card_fingerprint    VARCHAR(64),
    reason              TEXT NOT NULL,
    expires_at          TIMESTAMPTZ,
    created_by          UUID NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (
        email IS NOT NULL OR cnpj IS NOT NULL OR ip IS NOT NULL OR card_fingerprint IS NOT NULL
    )
);

CREATE INDEX idx_blocklist_email ON fraud.blocklist_entries (lower(email))
    WHERE email IS NOT NULL;
CREATE INDEX idx_blocklist_cnpj ON fraud.blocklist_entries (cnpj)
    WHERE cnpj IS NOT NULL;
CREATE INDEX idx_blocklist_ip ON fraud.blocklist_entries (ip)
    WHERE ip IS NOT NULL;
CREATE INDEX idx_blocklist_card ON fraud.blocklist_entries (card_fingerprint)
    WHERE card_fingerprint IS NOT NULL;

CREATE TABLE fraud.tenant_fraud_scores (
    tenant_id           UUID PRIMARY KEY REFERENCES shared.tenants (id),
    score               INT NOT NULL DEFAULT 0 CHECK (score >= 0),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE fraud.platform_settings (
    id                  INT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    thresholds          JSONB NOT NULL DEFAULT '{
        "loginFailureMax": 5,
        "loginFailureWindowSecs": 3600,
        "paymentVelocityMax": 20,
        "paymentVelocityWindowSecs": 3600,
        "provisionAlertMax": 10,
        "provisionAlertWindowSecs": 3600,
        "webhookFailureBurstMax": 10,
        "webhookFailureBurstWindowSecs": 300,
        "tenantFraudScoreBlock": 100
    }'::jsonb,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO fraud.platform_settings (id) VALUES (1);

ALTER TABLE fraud.fraud_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE fraud.fraud_events FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON fraud.fraud_events
    FOR SELECT
    USING (
        tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
        OR current_setting('app.bypass_rls', true) = 'true'
    );

GRANT USAGE ON SCHEMA fraud TO app_user;
GRANT SELECT ON fraud.fraud_events TO app_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON fraud.blocklist_entries TO app_user;
GRANT SELECT, INSERT, UPDATE ON fraud.tenant_fraud_scores TO app_user;
GRANT SELECT, UPDATE ON fraud.platform_settings TO app_user;
