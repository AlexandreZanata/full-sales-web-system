-- Module 01-identity: seller profiles (MIGRATION-SPEC-004)

CREATE TABLE identity.seller_profiles (
    user_id                 UUID PRIMARY KEY REFERENCES identity.users (id),
    tenant_id               UUID NOT NULL REFERENCES shared.tenants (id),
    operating_region        VARCHAR(200),
    monthly_target_amount   BIGINT CHECK (
        monthly_target_amount IS NULL OR monthly_target_amount >= 0
    ),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_seller_profiles_tenant ON identity.seller_profiles (tenant_id);

ALTER TABLE identity.seller_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.seller_profiles FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON identity.seller_profiles
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON identity.seller_profiles
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON identity.seller_profiles
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE TRIGGER set_seller_profiles_updated_at
    BEFORE UPDATE ON identity.seller_profiles
    FOR EACH ROW EXECUTE FUNCTION shared.set_updated_at();

REVOKE ALL ON identity.seller_profiles FROM app_user;
GRANT SELECT, INSERT, UPDATE ON identity.seller_profiles TO app_user;
