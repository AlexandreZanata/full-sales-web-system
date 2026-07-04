-- Allow Tenant entity type for site logo uploads (Phase 41)

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
        entity_type IN ('Product', 'User', 'Commerce', 'Delivery', 'Tenant')
    );
