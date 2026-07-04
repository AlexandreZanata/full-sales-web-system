-- Module 02-commerces: commerce addresses (MIGRATION-SPEC-003)

CREATE TABLE commerces.commerce_addresses (
    id           UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id    UUID NOT NULL REFERENCES shared.tenants (id),
    commerce_id  UUID NOT NULL,
    address_type VARCHAR(20) NOT NULL CHECK (address_type IN ('Billing', 'Delivery')),
    street       VARCHAR(200) NOT NULL CHECK (char_length(street) >= 1),
    number       VARCHAR(20) NOT NULL CHECK (char_length(number) >= 1),
    district     VARCHAR(100),
    city         VARCHAR(100) NOT NULL CHECK (char_length(city) >= 1),
    state        CHAR(2) NOT NULL,
    postal_code  VARCHAR(8) NOT NULL CHECK (postal_code ~ '^[0-9]{8}$'),
    latitude     NUMERIC(10, 7),
    longitude    NUMERIC(10, 7),
    is_primary   BOOLEAN NOT NULL DEFAULT false,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_commerce_addresses_commerce
    ON commerces.commerce_addresses (tenant_id, commerce_id, address_type);

CREATE UNIQUE INDEX uq_commerce_addresses_primary
    ON commerces.commerce_addresses (tenant_id, commerce_id, address_type)
    WHERE is_primary = true;

ALTER TABLE commerces.commerce_addresses ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerces.commerce_addresses FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON commerces.commerce_addresses
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON commerces.commerce_addresses
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON commerces.commerce_addresses
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE TRIGGER set_commerce_addresses_updated_at
    BEFORE UPDATE ON commerces.commerce_addresses
    FOR EACH ROW EXECUTE FUNCTION shared.set_updated_at();

REVOKE ALL ON commerces.commerce_addresses FROM app_user;
GRANT SELECT, INSERT, UPDATE ON commerces.commerce_addresses TO app_user;
