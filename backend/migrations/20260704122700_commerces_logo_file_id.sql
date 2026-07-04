-- Module 02-commerces: commerce logo (MIGRATION-SPEC-004)

ALTER TABLE commerces.commerces
    ADD COLUMN logo_file_id UUID;
