-- Phase 4: subscription billing tables (ADR-014, ADR-015).

ALTER TABLE billing.plans
    ADD COLUMN price_minor BIGINT NOT NULL DEFAULT 0 CHECK (price_minor >= 0),
    ADD COLUMN price_currency CHAR(3) NOT NULL DEFAULT 'BRL',
    ADD COLUMN billing_interval VARCHAR(20) NOT NULL DEFAULT 'Monthly'
        CHECK (billing_interval IN ('Monthly')),
    ADD COLUMN feature_limits JSONB NOT NULL DEFAULT '{}'::jsonb;

UPDATE billing.plans SET
    price_minor = CASE code
        WHEN 'Starter' THEN 9900
        WHEN 'Pro' THEN 19900
        WHEN 'Enterprise' THEN 49900
        ELSE 0
    END,
    feature_limits = CASE code
        WHEN 'Starter' THEN '{"maxUsers": 5, "customDomain": false}'::jsonb
        WHEN 'Pro' THEN '{"maxUsers": 25, "customDomain": true}'::jsonb
        WHEN 'Enterprise' THEN '{"maxUsers": null, "customDomain": true}'::jsonb
        ELSE '{}'::jsonb
    END;

ALTER TABLE shared.tenants
    ADD COLUMN past_due_at TIMESTAMPTZ,
    ADD COLUMN grace_extended_until TIMESTAMPTZ;

CREATE TABLE billing.subscriptions (
    id                      UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id               UUID NOT NULL REFERENCES shared.tenants (id),
    plan_id                 UUID NOT NULL REFERENCES billing.plans (id),
    asaas_subscription_id   VARCHAR(50),
    status                  VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (
        status IN ('Pending', 'Active', 'PastDue', 'Cancelled', 'Expired')
    ),
    current_period_end      TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_subscriptions_active_tenant
    ON billing.subscriptions (tenant_id)
    WHERE status IN ('Pending', 'Active', 'PastDue');

CREATE INDEX idx_subscriptions_tenant ON billing.subscriptions (tenant_id);
CREATE INDEX idx_subscriptions_asaas ON billing.subscriptions (asaas_subscription_id)
    WHERE asaas_subscription_id IS NOT NULL;

CREATE TABLE billing.invoices (
    id                  UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id           UUID NOT NULL REFERENCES shared.tenants (id),
    subscription_id     UUID NOT NULL REFERENCES billing.subscriptions (id),
    amount_minor        BIGINT NOT NULL CHECK (amount_minor >= 0),
    amount_currency     CHAR(3) NOT NULL DEFAULT 'BRL',
    due_date            TIMESTAMPTZ NOT NULL,
    paid_at             TIMESTAMPTZ,
    status              VARCHAR(20) NOT NULL DEFAULT 'Open' CHECK (
        status IN ('Pending', 'Open', 'Paid', 'Overdue', 'Cancelled', 'Refunded')
    ),
    asaas_payment_id    VARCHAR(50),
    pdf_url             TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_invoices_asaas_payment
    ON billing.invoices (asaas_payment_id)
    WHERE asaas_payment_id IS NOT NULL;

CREATE INDEX idx_invoices_tenant_due ON billing.invoices (tenant_id, due_date DESC);
CREATE INDEX idx_invoices_subscription ON billing.invoices (subscription_id);

ALTER TABLE billing.subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE billing.subscriptions FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON billing.subscriptions
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

ALTER TABLE billing.invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE billing.invoices FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON billing.invoices
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

GRANT SELECT, INSERT, UPDATE ON billing.subscriptions TO app_user;
GRANT SELECT, INSERT, UPDATE ON billing.invoices TO app_user;
