-- Module 03-inventory: products (MIGRATION-SPEC-001-products)

CREATE SCHEMA IF NOT EXISTS inventory;

CREATE TABLE inventory.products (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    sku             VARCHAR(64) NOT NULL,
    name            VARCHAR(200) NOT NULL CHECK (char_length(name) >= 1),
    price_amount    BIGINT NOT NULL CHECK (price_amount >= 0),
    price_currency  CHAR(3) NOT NULL,
    active          BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_products_tenant_sku UNIQUE (tenant_id, sku)
);

CREATE INDEX idx_products_tenant ON inventory.products (tenant_id);

ALTER TABLE inventory.products ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory.products FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON inventory.products
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON inventory.products
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON inventory.products
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
