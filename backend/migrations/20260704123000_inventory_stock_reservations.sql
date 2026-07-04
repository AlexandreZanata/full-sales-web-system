-- Module 03-inventory: stock reservations (MIGRATION-SPEC-008, ADR-010)

CREATE TABLE inventory.stock_reservations (
    id                UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id         UUID NOT NULL REFERENCES shared.tenants (id),
    order_id          UUID NOT NULL,
    order_item_id     UUID NOT NULL,
    product_id        UUID NOT NULL,
    driver_id         UUID,
    quantity_reserved INTEGER NOT NULL CHECK (quantity_reserved > 0),
    status            VARCHAR(20) NOT NULL DEFAULT 'Active'
        CHECK (status IN ('Active', 'Released', 'Consumed')),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    released_at       TIMESTAMPTZ,
    consumed_at       TIMESTAMPTZ
);

CREATE INDEX idx_stock_reservations_active
    ON inventory.stock_reservations (tenant_id, product_id, status)
    WHERE status = 'Active';

CREATE INDEX idx_stock_reservations_order
    ON inventory.stock_reservations (tenant_id, order_id);

ALTER TABLE inventory.stock_reservations ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory.stock_reservations FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON inventory.stock_reservations
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON inventory.stock_reservations
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON inventory.stock_reservations
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

REVOKE ALL ON inventory.stock_reservations FROM app_user;
GRANT SELECT, INSERT, UPDATE ON inventory.stock_reservations TO app_user;
