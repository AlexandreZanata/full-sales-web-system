-- Module 03-inventory: product images (MIGRATION-SPEC-007)

CREATE TABLE inventory.product_images (
    id          UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id   UUID NOT NULL REFERENCES shared.tenants (id),
    product_id  UUID NOT NULL,
    file_id     UUID NOT NULL,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    is_primary  BOOLEAN NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_product_images_product
    ON inventory.product_images (tenant_id, product_id);

CREATE UNIQUE INDEX uq_product_images_primary
    ON inventory.product_images (tenant_id, product_id)
    WHERE is_primary = true;

ALTER TABLE inventory.product_images ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory.product_images FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON inventory.product_images
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON inventory.product_images
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON inventory.product_images
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

REVOKE ALL ON inventory.product_images FROM app_user;
GRANT SELECT, INSERT, UPDATE ON inventory.product_images TO app_user;
