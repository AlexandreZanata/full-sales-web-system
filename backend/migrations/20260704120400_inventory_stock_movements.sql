-- Module 03-inventory: stock_movements (MIGRATION-SPEC-002-stock-movements) — append-only

CREATE TABLE inventory.stock_movements (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    product_id      UUID NOT NULL,
    responsible_id  UUID NOT NULL,
    movement_type   VARCHAR(20) NOT NULL CHECK (
        movement_type IN ('Inbound', 'SaleOutbound', 'Adjustment', 'Return')
    ),
    quantity        INTEGER NOT NULL CHECK (quantity > 0),
    reference_id    UUID,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_stock_movements_tenant ON inventory.stock_movements (tenant_id);
CREATE INDEX idx_stock_movements_driver_product
    ON inventory.stock_movements (tenant_id, responsible_id, product_id);

ALTER TABLE inventory.stock_movements ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory.stock_movements FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON inventory.stock_movements
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON inventory.stock_movements
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);
