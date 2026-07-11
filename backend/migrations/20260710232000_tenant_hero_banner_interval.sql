-- Hero carousel autoplay interval (seconds) — editable by tenant admin.

ALTER TABLE shared.tenants
    ADD COLUMN IF NOT EXISTS hero_banner_interval_secs INTEGER NOT NULL DEFAULT 7;

ALTER TABLE shared.tenants
    DROP CONSTRAINT IF EXISTS tenants_hero_banner_interval_secs_check;

ALTER TABLE shared.tenants
    ADD CONSTRAINT tenants_hero_banner_interval_secs_check CHECK (
        hero_banner_interval_secs >= 3 AND hero_banner_interval_secs <= 120
    );
