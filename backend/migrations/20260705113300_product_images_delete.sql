-- Allow Admin to remove product gallery images (GAP: DELETE returned 500 — missing grant/policy)

CREATE POLICY tenant_delete ON inventory.product_images
    FOR DELETE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

GRANT DELETE ON inventory.product_images TO app_user;
