-- Module 09-deliveries: deliveries table (MIGRATION-SPEC-001, DE-004)

CREATE SCHEMA IF NOT EXISTS deliveries;

CREATE OR REPLACE FUNCTION deliveries.can_access_delivery(p_driver_id UUID) RETURNS BOOLEAN AS $$
DECLARE
    role TEXT := current_setting('app.role', true);
    user_id UUID := NULLIF(current_setting('app.user_id', true), '')::uuid;
BEGIN
    IF role = 'Admin' THEN
        RETURN TRUE;
    END IF;
    IF role = 'Driver' AND user_id IS NOT NULL AND p_driver_id = user_id THEN
        RETURN TRUE;
    END IF;
    RETURN FALSE;
END;
$$ LANGUAGE plpgsql STABLE;

CREATE TABLE deliveries.deliveries (
    id                  UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id           UUID NOT NULL REFERENCES shared.tenants (id),
    order_id            UUID NOT NULL UNIQUE,
    driver_id           UUID NOT NULL,
    status              VARCHAR(20) NOT NULL DEFAULT 'Waiting' CHECK (
        status IN ('Waiting', 'InTransit', 'Delivered', 'Failed')
    ),
    proof_file_id       UUID,
    delivery_latitude   NUMERIC(10, 7),
    delivery_longitude  NUMERIC(10, 7),
    received_by_name    VARCHAR(200),
    delivered_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_deliveries_driver_status
    ON deliveries.deliveries (tenant_id, driver_id, status);

ALTER TABLE deliveries.deliveries ENABLE ROW LEVEL SECURITY;
ALTER TABLE deliveries.deliveries FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON deliveries.deliveries
    FOR SELECT
    USING (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND deliveries.can_access_delivery(driver_id)
    );

CREATE POLICY tenant_insert ON deliveries.deliveries
    FOR INSERT
    WITH CHECK (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND deliveries.can_access_delivery(driver_id)
    );

CREATE POLICY tenant_update ON deliveries.deliveries
    FOR UPDATE
    USING (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND deliveries.can_access_delivery(driver_id)
    )
    WITH CHECK (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND deliveries.can_access_delivery(driver_id)
    );

CREATE TRIGGER set_deliveries_updated_at
    BEFORE UPDATE ON deliveries.deliveries
    FOR EACH ROW EXECUTE FUNCTION shared.set_updated_at();

REVOKE ALL ON SCHEMA deliveries FROM app_user;
GRANT USAGE ON SCHEMA deliveries TO app_user;
REVOKE ALL ON deliveries.deliveries FROM app_user;
GRANT SELECT, INSERT, UPDATE ON deliveries.deliveries TO app_user;
