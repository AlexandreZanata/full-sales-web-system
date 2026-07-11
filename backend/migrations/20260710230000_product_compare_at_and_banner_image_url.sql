-- Product compare-at pricing (Mercado Livre style) + optional external banner image URLs.

ALTER TABLE inventory.products
    ADD COLUMN IF NOT EXISTS compare_at_price BIGINT NULL;

ALTER TABLE inventory.products
    DROP CONSTRAINT IF EXISTS products_compare_at_price_check;

ALTER TABLE inventory.products
    ADD CONSTRAINT products_compare_at_price_check CHECK (
        compare_at_price IS NULL OR compare_at_price > price_amount
    );

ALTER TABLE portal.banners
    ADD COLUMN IF NOT EXISTS image_url TEXT NULL;

ALTER TABLE portal.banners
    ALTER COLUMN image_file_id DROP NOT NULL;

ALTER TABLE portal.banners
    DROP CONSTRAINT IF EXISTS portal_banners_image_source_check;

ALTER TABLE portal.banners
    ADD CONSTRAINT portal_banners_image_source_check CHECK (
        (image_file_id IS NOT NULL AND (image_url IS NULL OR trim(image_url) = ''))
        OR (
            image_file_id IS NULL
            AND image_url IS NOT NULL
            AND length(trim(image_url)) > 0
        )
    );
