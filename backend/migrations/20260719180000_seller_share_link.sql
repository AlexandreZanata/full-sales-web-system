-- Phase 19: seller catalog share link fields on seller_profiles

ALTER TABLE identity.seller_profiles
    ADD COLUMN IF NOT EXISTS public_code VARCHAR(32),
    ADD COLUMN IF NOT EXISTS contact_phone VARCHAR(15),
    ADD COLUMN IF NOT EXISTS share_link_active BOOLEAN NOT NULL DEFAULT true;

-- Unique per tenant when code is present (NULLs allowed multiple times in Postgres)
CREATE UNIQUE INDEX IF NOT EXISTS idx_seller_profiles_tenant_public_code
    ON identity.seller_profiles (tenant_id, lower(public_code))
    WHERE public_code IS NOT NULL;

-- Backfill human-ish codes from user name for sellers missing public_code (OD-19-4 A)
WITH candidates AS (
    SELECT
        sp.user_id,
        sp.tenant_id,
        lower(
            regexp_replace(
                regexp_replace(trim(u.name), '[^a-zA-Z0-9]+', '-', 'g'),
                '(^-+|-+$)',
                '',
                'g'
            )
        ) AS base_slug
    FROM identity.seller_profiles sp
    JOIN identity.users u ON u.id = sp.user_id
    WHERE sp.public_code IS NULL
      AND u.role = 'Seller'
),
numbered AS (
    SELECT
        user_id,
        tenant_id,
        CASE
            WHEN base_slug IS NULL OR base_slug = '' THEN 'seller'
            ELSE left(base_slug, 28)
        END AS base_slug,
        row_number() OVER (
            PARTITION BY tenant_id,
                CASE
                    WHEN base_slug IS NULL OR base_slug = '' THEN 'seller'
                    ELSE left(base_slug, 28)
                END
            ORDER BY user_id
        ) AS rn
    FROM candidates
)
UPDATE identity.seller_profiles sp
SET public_code = CASE
    WHEN n.rn = 1 THEN n.base_slug
    ELSE left(n.base_slug, 28) || '-' || n.rn::text
END
FROM numbered n
WHERE sp.user_id = n.user_id
  AND sp.public_code IS NULL;
