-- Phase 3: Asaas payment events + tenant customer id (ADR-014).

ALTER TABLE shared.tenants
    ADD COLUMN asaas_customer_id VARCHAR(50);

CREATE INDEX idx_tenants_asaas_customer
    ON shared.tenants (asaas_customer_id)
    WHERE asaas_customer_id IS NOT NULL;

CREATE TABLE billing.payment_events (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    asaas_event_id  VARCHAR(100) NOT NULL UNIQUE,
    event_type      VARCHAR(80) NOT NULL,
    tenant_id       UUID REFERENCES shared.tenants (id),
    payload         JSONB NOT NULL,
    status          VARCHAR(20) NOT NULL DEFAULT 'received' CHECK (
        status IN ('received', 'processed', 'failed')
    ),
    processed_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_payment_events_tenant ON billing.payment_events (tenant_id)
    WHERE tenant_id IS NOT NULL;
CREATE INDEX idx_payment_events_created ON billing.payment_events (created_at DESC);
CREATE INDEX idx_payment_events_status ON billing.payment_events (status);

CREATE TABLE billing.provisioning_dead_letters (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    error_code      VARCHAR(50) NOT NULL,
    error_message   TEXT NOT NULL,
    payload         JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_provisioning_dead_letters_tenant
    ON billing.provisioning_dead_letters (tenant_id);

GRANT SELECT, INSERT, UPDATE ON billing.payment_events TO app_user;
GRANT SELECT, INSERT ON billing.provisioning_dead_letters TO app_user;
