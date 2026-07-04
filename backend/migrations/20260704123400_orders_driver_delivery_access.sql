-- Phase 12: allow assigned Driver to update order + items during delivery confirm

CREATE POLICY driver_delivery_update ON orders.orders
    FOR UPDATE
    USING (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND current_setting('app.role', true) = 'Driver'
        AND EXISTS (
            SELECT 1
            FROM deliveries.deliveries d
            WHERE d.order_id = orders.orders.id
              AND d.tenant_id = orders.orders.tenant_id
              AND d.driver_id = NULLIF(current_setting('app.user_id', true), '')::uuid
              AND d.status = 'InTransit'
        )
    )
    WITH CHECK (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND current_setting('app.role', true) = 'Driver'
        AND EXISTS (
            SELECT 1
            FROM deliveries.deliveries d
            WHERE d.order_id = orders.orders.id
              AND d.tenant_id = orders.orders.tenant_id
              AND d.driver_id = NULLIF(current_setting('app.user_id', true), '')::uuid
        )
    );

CREATE POLICY driver_delivery_items_update ON orders.order_items
    FOR UPDATE
    USING (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND current_setting('app.role', true) = 'Driver'
        AND EXISTS (
            SELECT 1
            FROM deliveries.deliveries d
            JOIN orders.orders o ON o.id = d.order_id
            WHERE d.order_id = order_items.order_id
              AND d.tenant_id = order_items.tenant_id
              AND d.driver_id = NULLIF(current_setting('app.user_id', true), '')::uuid
              AND d.status = 'InTransit'
        )
    )
    WITH CHECK (
        tenant_id = current_setting('app.tenant_id', true)::uuid
        AND current_setting('app.role', true) = 'Driver'
        AND EXISTS (
            SELECT 1
            FROM deliveries.deliveries d
            WHERE d.order_id = order_items.order_id
              AND d.tenant_id = order_items.tenant_id
              AND d.driver_id = NULLIF(current_setting('app.user_id', true), '')::uuid
        )
    );
