-- Phase 43: structured product categories (replaces free-text products.category)

CREATE TABLE inventory.product_categories (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id       UUID NOT NULL REFERENCES shared.tenants (id),
    name            VARCHAR(200) NOT NULL CHECK (char_length(name) >= 1),
    slug            VARCHAR(120) NOT NULL,
    description     TEXT,
    sort_order      INTEGER NOT NULL DEFAULT 0,
    active          BOOLEAN NOT NULL DEFAULT true,
    image_file_id   UUID REFERENCES media.files (id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_product_categories_tenant_slug UNIQUE (tenant_id, slug)
);

CREATE INDEX idx_product_categories_tenant ON inventory.product_categories (tenant_id);
CREATE INDEX idx_product_categories_tenant_sort ON inventory.product_categories (tenant_id, sort_order);

ALTER TABLE inventory.product_categories ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory.product_categories FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON inventory.product_categories
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON inventory.product_categories
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_update ON inventory.product_categories
    FOR UPDATE
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

ALTER TABLE inventory.products
    ADD COLUMN category_id UUID REFERENCES inventory.product_categories (id);

CREATE INDEX idx_products_category_id ON inventory.products (category_id);

-- Backfill categories from legacy free-text column (distinct per tenant).
INSERT INTO inventory.product_categories (id, tenant_id, name, slug, sort_order, active)
SELECT
    uuidv7(),
    tenant_id,
    trimmed,
    slug,
    (ROW_NUMBER() OVER (PARTITION BY tenant_id ORDER BY trimmed) - 1)::integer,
    true
FROM (
    SELECT DISTINCT
        p.tenant_id,
        trim(p.category) AS trimmed,
        lower(
            regexp_replace(
                regexp_replace(trim(p.category), '[^a-zA-Z0-9]+', '-', 'g'),
                '(^-|-$)',
                '',
                'g'
            )
        ) AS slug
    FROM inventory.products p
    WHERE p.category IS NOT NULL
      AND trim(p.category) <> ''
) AS distinct_categories
WHERE slug <> '';

UPDATE inventory.products p
SET category_id = pc.id
FROM inventory.product_categories pc
WHERE p.tenant_id = pc.tenant_id
  AND p.category IS NOT NULL
  AND trim(p.category) <> ''
  AND pc.slug = lower(
        regexp_replace(
            regexp_replace(trim(p.category), '[^a-zA-Z0-9]+', '-', 'g'),
            '(^-|-$)',
            '',
            'g'
        )
    );

-- Allow category image uploads via media.files
DO $$
DECLARE
    constraint_name text;
BEGIN
    FOR constraint_name IN
        SELECT con.conname
        FROM pg_constraint con
        JOIN pg_class rel ON rel.oid = con.conrelid
        JOIN pg_namespace nsp ON nsp.oid = rel.relnamespace
        WHERE nsp.nspname = 'media'
          AND rel.relname = 'files'
          AND con.contype = 'c'
          AND pg_get_constraintdef(con.oid) LIKE '%entity_type%'
    LOOP
        EXECUTE format('ALTER TABLE media.files DROP CONSTRAINT %I', constraint_name);
    END LOOP;
END $$;

ALTER TABLE media.files
    ADD CONSTRAINT files_entity_type_check CHECK (
        entity_type IN ('Product', 'User', 'Commerce', 'Delivery', 'Tenant', 'ProductCategory')
    );

GRANT SELECT, INSERT, UPDATE ON inventory.product_categories TO app_user;
