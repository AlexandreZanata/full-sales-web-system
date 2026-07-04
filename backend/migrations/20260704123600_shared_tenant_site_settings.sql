-- Module 00-shared: tenant site branding (Phase 41)

ALTER TABLE shared.tenants
    ADD COLUMN display_name VARCHAR(200),
    ADD COLUMN logo_file_id UUID;

UPDATE shared.tenants
SET display_name = name
WHERE display_name IS NULL;

ALTER TABLE shared.tenants
    ALTER COLUMN display_name SET NOT NULL;

ALTER TABLE shared.tenants
    ADD CONSTRAINT tenants_display_name_check CHECK (char_length(display_name) >= 1);

GRANT SELECT, UPDATE ON shared.tenants TO app_user;
