-- Phase 71 — Portal home content (banners, promotions, featured products).

CREATE SCHEMA IF NOT EXISTS portal;

ALTER TABLE inventory.products
    ADD COLUMN IF NOT EXISTS is_featured BOOLEAN NOT NULL DEFAULT false;

CREATE INDEX IF NOT EXISTS idx_products_tenant_featured
    ON inventory.products (tenant_id, is_featured)
    WHERE is_featured = true AND active = true;

CREATE TABLE portal.banners (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    placement       VARCHAR(32) NOT NULL DEFAULT 'hero',
    image_file_id   UUID NOT NULL REFERENCES media.files (id),
    link_url        TEXT,
    alt_text        VARCHAR(200),
    sort_order      INTEGER NOT NULL DEFAULT 0,
    active          BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_portal_banners_tenant_placement
    ON portal.banners (tenant_id, placement, sort_order)
    WHERE active = true;

CREATE TABLE portal.promotions (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    headline        VARCHAR(200) NOT NULL,
    discount_text   VARCHAR(200) NOT NULL,
    background      VARCHAR(16) NOT NULL CHECK (background IN ('yellow', 'green')),
    category_slug   VARCHAR(64),
    link_url        TEXT,
    image_file_id   UUID REFERENCES media.files (id),
    sort_order      INTEGER NOT NULL DEFAULT 0,
    active          BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_portal_promotions_tenant_sort
    ON portal.promotions (tenant_id, sort_order)
    WHERE active = true;

ALTER TABLE portal.banners ENABLE ROW LEVEL SECURITY;
ALTER TABLE portal.banners FORCE ROW LEVEL SECURITY;
ALTER TABLE portal.promotions ENABLE ROW LEVEL SECURITY;
ALTER TABLE portal.promotions FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON portal.banners
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON portal.banners
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON portal.banners
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_delete ON portal.banners
    FOR DELETE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_select ON portal.promotions
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON portal.promotions
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON portal.promotions
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_delete ON portal.promotions
    FOR DELETE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

GRANT USAGE ON SCHEMA portal TO app_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON portal.banners TO app_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON portal.promotions TO app_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA portal GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO app_user;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM pg_constraint con
        JOIN pg_class rel ON rel.oid = con.conrelid
        JOIN pg_namespace nsp ON nsp.oid = rel.relnamespace
        WHERE nsp.nspname = 'media'
          AND rel.relname = 'files'
          AND con.contype = 'c'
          AND pg_get_constraintdef(con.oid) LIKE '%entity_type%'
    ) THEN
        ALTER TABLE media.files DROP CONSTRAINT files_entity_type_check;
    END IF;
END $$;

ALTER TABLE media.files
    ADD CONSTRAINT files_entity_type_check CHECK (
        entity_type IN (
            'Product',
            'User',
            'Commerce',
            'Delivery',
            'Tenant',
            'ProductCategory',
            'PortalBanner',
            'PortalPromotion'
        )
    );
