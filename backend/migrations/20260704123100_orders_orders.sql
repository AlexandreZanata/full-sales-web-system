-- Module 08-orders: orders table (MIGRATION-SPEC-001)

CREATE SCHEMA IF NOT EXISTS orders;

CREATE OR REPLACE FUNCTION orders.can_access_order(
    p_commerce_id UUID,
    p_created_by UUID
) RETURNS BOOLEAN AS $$
DECLARE
    role TEXT := current_setting('app.role', true);
    user_id UUID := NULLIF(current_setting('app.user_id', true), '')::uuid;
    commerce_id UUID := NULLIF(current_setting('app.commerce_id', true), '')::uuid;
BEGIN
    IF role = 'Admin' THEN
        RETURN TRUE;
    END IF;
    IF role = 'CommerceContact'
        AND commerce_id IS NOT NULL
        AND p_commerce_id = commerce_id THEN
        RETURN TRUE;
    END IF;
    IF role = 'Seller'
        AND user_id IS NOT NULL
        AND p_created_by = user_id THEN
        RETURN TRUE;
    END IF;
    RETURN FALSE;
END;
$$ LANGUAGE plpgsql STABLE;

CREATE TABLE orders.orders (
    id                  UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id           UUID NOT NULL REFERENCES shared.tenants (id),
    commerce_id         UUID NOT NULL,
    created_by_user_id  UUID NOT NULL,
    source              VARCHAR(30) NOT NULL CHECK (
        source IN ('CommercePortal', 'SellerVisit')
    ),
    status              VARCHAR(30) NOT NULL DEFAULT 'Draft' CHECK (
        status IN (
            'Draft', 'PendingApproval', 'Approved', 'Rejected',
            'Picking', 'InTransit', 'Delivered', 'PartiallyDelivered', 'Cancelled'
        )
    ),
    delivery_address_id UUID NOT NULL,
    notes               TEXT,
    total_amount        BIGINT NOT NULL DEFAULT 0 CHECK (total_amount >= 0),
    total_currency      CHAR(3) NOT NULL DEFAULT 'BRL',
    rejection_reason    TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    cancelled_at        TIMESTAMPTZ
);

CREATE INDEX idx_orders_tenant_commerce_created
    ON orders.orders (tenant_id, commerce_id, created_at DESC);

CREATE INDEX idx_orders_tenant_status_created
    ON orders.orders (tenant_id, status, created_at DESC);

ALTER TABLE orders.orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE orders.orders FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON orders.orders
    FOR SELECT
    USING (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND orders.can_access_order(commerce_id, created_by_user_id)
    );

CREATE POLICY tenant_insert ON orders.orders
    FOR INSERT
    WITH CHECK (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND orders.can_access_order(commerce_id, created_by_user_id)
    );

CREATE POLICY tenant_update ON orders.orders
    FOR UPDATE
    USING (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND orders.can_access_order(commerce_id, created_by_user_id)
    )
    WITH CHECK (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND orders.can_access_order(commerce_id, created_by_user_id)
    );

CREATE TRIGGER set_orders_updated_at
    BEFORE UPDATE ON orders.orders
    FOR EACH ROW EXECUTE FUNCTION shared.set_updated_at();

REVOKE ALL ON SCHEMA orders FROM app_user;
GRANT USAGE ON SCHEMA orders TO app_user;
REVOKE ALL ON orders.orders FROM app_user;
GRANT SELECT, INSERT, UPDATE ON orders.orders TO app_user;
