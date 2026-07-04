-- Module 01-identity: driver profiles (MIGRATION-SPEC-003)

CREATE TABLE identity.driver_profiles (
    user_id             UUID PRIMARY KEY REFERENCES identity.users (id),
    tenant_id           UUID NOT NULL REFERENCES shared.tenants (id),
    cnh_number          VARCHAR(20) NOT NULL,
    cnh_category        VARCHAR(5) NOT NULL,
    cnh_photo_file_id   UUID REFERENCES media.files (id),
    vehicle_plate       VARCHAR(10) NOT NULL,
    vehicle_model       VARCHAR(100) NOT NULL,
    vehicle_capacity_kg NUMERIC(10, 2),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_driver_profiles_tenant ON identity.driver_profiles (tenant_id);

ALTER TABLE identity.driver_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.driver_profiles FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON identity.driver_profiles
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON identity.driver_profiles
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON identity.driver_profiles
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE TRIGGER set_driver_profiles_updated_at
    BEFORE UPDATE ON identity.driver_profiles
    FOR EACH ROW EXECUTE FUNCTION shared.set_updated_at();

REVOKE ALL ON identity.driver_profiles FROM app_user;
GRANT SELECT, INSERT, UPDATE ON identity.driver_profiles TO app_user;
