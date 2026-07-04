-- Module 08-orders: order items (MIGRATION-SPEC-002)

CREATE TABLE orders.order_items (
    id                      UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id               UUID NOT NULL REFERENCES shared.tenants (id),
    order_id                UUID NOT NULL REFERENCES orders.orders (id),
    product_id              UUID NOT NULL,
    quantity_requested      INTEGER NOT NULL CHECK (quantity_requested > 0),
    quantity_delivered      INTEGER CHECK (quantity_delivered IS NULL OR quantity_delivered >= 0),
    unit_price_amount       BIGINT NOT NULL CHECK (unit_price_amount >= 0),
    unit_price_currency     CHAR(3) NOT NULL,
    line_total_amount       BIGINT NOT NULL CHECK (line_total_amount >= 0),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_order_items_order
    ON orders.order_items (tenant_id, order_id);

ALTER TABLE orders.order_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE orders.order_items FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON orders.order_items
    FOR SELECT
    USING (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND EXISTS (
            SELECT 1
            FROM orders.orders o
            WHERE o.id = order_items.order_id
              AND o.tenant_id = order_items.tenant_id
              AND orders.can_access_order(o.commerce_id, o.created_by_user_id)
        )
    );

CREATE POLICY tenant_insert ON orders.order_items
    FOR INSERT
    WITH CHECK (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND EXISTS (
            SELECT 1
            FROM orders.orders o
            WHERE o.id = order_items.order_id
              AND o.tenant_id = order_items.tenant_id
              AND orders.can_access_order(o.commerce_id, o.created_by_user_id)
        )
    );

CREATE POLICY tenant_update ON orders.order_items
    FOR UPDATE
    USING (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND EXISTS (
            SELECT 1
            FROM orders.orders o
            WHERE o.id = order_items.order_id
              AND o.tenant_id = order_items.tenant_id
              AND orders.can_access_order(o.commerce_id, o.created_by_user_id)
        )
    )
    WITH CHECK (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND EXISTS (
            SELECT 1
            FROM orders.orders o
            WHERE o.id = order_items.order_id
              AND o.tenant_id = order_items.tenant_id
              AND orders.can_access_order(o.commerce_id, o.created_by_user_id)
        )
    );

REVOKE ALL ON orders.order_items FROM app_user;
GRANT SELECT, INSERT, UPDATE ON orders.order_items TO app_user;
