-- Module 03-inventory: stock_balances (MIGRATION-SPEC-003-stock-balances, ADR-001, ADR-005)

CREATE TABLE inventory.stock_balances (
    tenant_id   UUID NOT NULL REFERENCES shared.tenants (id),
    driver_id   UUID NOT NULL,
    product_id  UUID NOT NULL,
    quantity    INTEGER NOT NULL CHECK (quantity >= 0),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (tenant_id, driver_id, product_id)
);

CREATE INDEX idx_stock_balances_tenant ON inventory.stock_balances (tenant_id);

ALTER TABLE inventory.stock_balances ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory.stock_balances FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON inventory.stock_balances
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON inventory.stock_balances
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON inventory.stock_balances
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
