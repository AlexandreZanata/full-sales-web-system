-- Module 07-media: files (MIGRATION-SPEC-001-files)

CREATE SCHEMA IF NOT EXISTS media;

CREATE TABLE media.files (
    id                  UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id           UUID NOT NULL REFERENCES shared.tenants (id),
    entity_type         VARCHAR(20) NOT NULL CHECK (
        entity_type IN ('Product', 'User', 'Commerce', 'Delivery')
    ),
    entity_id           UUID NOT NULL,
    bucket              VARCHAR(255) NOT NULL,
    object_key          VARCHAR(1024) NOT NULL,
    mime_type           VARCHAR(100) NOT NULL,
    size_bytes          BIGINT NOT NULL CHECK (size_bytes > 0 AND size_bytes <= 5242880),
    sha256              CHAR(64) NOT NULL,
    uploaded_by_user_id UUID NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_media_files_bucket_key UNIQUE (bucket, object_key)
);

CREATE INDEX idx_media_files_tenant_entity
    ON media.files (tenant_id, entity_type, entity_id);

ALTER TABLE media.files ENABLE ROW LEVEL SECURITY;
ALTER TABLE media.files FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON media.files
    FOR SELECT
    USING (tenant_id = current_setting('app.tenant_id', true)::uuid);

CREATE POLICY tenant_insert ON media.files
    FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.tenant_id', true)::uuid);

REVOKE ALL ON media.files FROM app_user;
GRANT SELECT, INSERT ON media.files TO app_user;

GRANT USAGE ON SCHEMA media TO app_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA media GRANT SELECT, INSERT ON TABLES TO app_user;
