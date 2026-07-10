-- Phase 2: tenant lifecycle columns + billing plans stub (ADR-015).

CREATE SCHEMA IF NOT EXISTS billing;

CREATE TABLE billing.plans (
    id          UUID PRIMARY KEY,
    code        VARCHAR(32) NOT NULL UNIQUE,
    name        VARCHAR(100) NOT NULL,
    active      BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO billing.plans (id, code, name) VALUES
    ('01900002-0001-7000-8000-000000000001', 'Starter', 'Starter'),
    ('01900002-0001-7000-8000-000000000002', 'Pro', 'Pro'),
    ('01900002-0001-7000-8000-000000000003', 'Enterprise', 'Enterprise');

ALTER TABLE shared.tenants
    ADD COLUMN status VARCHAR(20) NOT NULL DEFAULT 'Active'
        CHECK (status IN (
            'Provisioning', 'Trial', 'Active', 'PastDue',
            'Suspended', 'Offboarding', 'Deleted'
        )),
    ADD COLUMN legal_name VARCHAR(200),
    ADD COLUMN plan_id UUID REFERENCES billing.plans (id),
    ADD COLUMN trial_ends_at TIMESTAMPTZ,
    ADD COLUMN suspended_at TIMESTAMPTZ,
    ADD COLUMN suspended_reason VARCHAR(500),
    ADD COLUMN offboarding_scheduled_at TIMESTAMPTZ,
    ADD COLUMN settings JSONB NOT NULL DEFAULT '{}'::jsonb;

UPDATE shared.tenants
SET legal_name = name,
    status = CASE WHEN active THEN 'Active' ELSE 'Suspended' END
WHERE legal_name IS NULL;

ALTER TABLE shared.tenants
    ALTER COLUMN legal_name SET NOT NULL;

CREATE INDEX idx_tenants_status ON shared.tenants (status);
CREATE INDEX idx_tenants_plan_id ON shared.tenants (plan_id) WHERE plan_id IS NOT NULL;

GRANT USAGE ON SCHEMA billing TO app_user;
GRANT SELECT ON billing.plans TO app_user;
GRANT SELECT, INSERT, UPDATE ON shared.tenants TO app_user;
