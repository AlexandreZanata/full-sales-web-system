-- Draft order replace deletes and re-inserts line items (PUT /v1/portal/orders/{id}).
-- Without DELETE grant + RLS policy, replace_draft_order fails under app_user.

CREATE POLICY tenant_delete ON orders.order_items
    FOR DELETE
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

GRANT DELETE ON orders.order_items TO app_user;
