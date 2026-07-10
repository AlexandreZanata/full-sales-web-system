-- Phase 5: tenant payment settings and encrypted Asaas credentials (ADR-018).

UPDATE billing.plans
SET feature_limits = feature_limits || '{"onlinePayments": false}'::jsonb
WHERE code = 'Starter';

UPDATE billing.plans
SET feature_limits = feature_limits || '{"onlinePayments": true}'::jsonb
WHERE code IN ('Pro', 'Enterprise');

CREATE TABLE billing.tenant_payment_settings (
    tenant_id       UUID PRIMARY KEY REFERENCES shared.tenants (id),
    enabled         BOOLEAN NOT NULL DEFAULT false,
    method_pix      BOOLEAN NOT NULL DEFAULT true,
    method_credit   BOOLEAN NOT NULL DEFAULT true,
    method_boleto   BOOLEAN NOT NULL DEFAULT false,
    auto_capture    BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE billing.tenant_asaas_credentials (
    tenant_id       UUID PRIMARY KEY REFERENCES shared.tenants (id),
    ciphertext      BYTEA NOT NULL,
    nonce           BYTEA NOT NULL,
    key_version     SMALLINT NOT NULL DEFAULT 1 CHECK (key_version >= 1),
    api_key_last4   VARCHAR(4) NOT NULL,
    connected_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE billing.tenant_payment_settings ENABLE ROW LEVEL SECURITY;
ALTER TABLE billing.tenant_payment_settings FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON billing.tenant_payment_settings
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON billing.tenant_payment_settings
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON billing.tenant_payment_settings
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

GRANT SELECT, INSERT, UPDATE ON billing.tenant_payment_settings TO app_user;

ALTER TABLE orders.orders
    ADD COLUMN asaas_payment_id VARCHAR(50);

CREATE UNIQUE INDEX idx_orders_asaas_payment
    ON orders.orders (asaas_payment_id)
    WHERE asaas_payment_id IS NOT NULL;

ALTER TABLE orders.orders DROP CONSTRAINT orders_status_check;

ALTER TABLE orders.orders ADD CONSTRAINT orders_status_check CHECK (
    status IN (
        'Draft', 'AwaitingPayment', 'Paid', 'PendingApproval', 'Approved', 'Rejected',
        'Picking', 'InTransit', 'Delivered', 'PartiallyDelivered', 'Cancelled'
    )
);
