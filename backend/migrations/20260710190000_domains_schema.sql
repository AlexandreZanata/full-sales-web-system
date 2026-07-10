-- Phase 7: custom tenant domains (ADR-017).

CREATE SCHEMA IF NOT EXISTS domains;

CREATE TABLE domains.tenant_domains (
    id                  UUID PRIMARY KEY DEFAULT uuidv7(),
    tenant_id           UUID NOT NULL REFERENCES shared.tenants (id),
    hostname            VARCHAR(255) NOT NULL,
    status              VARCHAR(20) NOT NULL CHECK (
        status IN ('Pending', 'Verifying', 'Verified', 'Active', 'Failed', 'Detached')
    ),
    verification_token  VARCHAR(64) NOT NULL,
    verified_at         TIMESTAMPTZ,
    is_primary          BOOLEAN NOT NULL DEFAULT false,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (hostname)
);

CREATE INDEX idx_tenant_domains_tenant ON domains.tenant_domains (tenant_id);
CREATE INDEX idx_tenant_domains_status ON domains.tenant_domains (status);
CREATE UNIQUE INDEX idx_tenant_domains_one_primary
    ON domains.tenant_domains (tenant_id)
    WHERE is_primary = true AND status = 'Active';

CREATE TABLE domains.domain_verification_challenges (
    id                  UUID PRIMARY KEY DEFAULT uuidv7(),
    domain_id           UUID NOT NULL REFERENCES domains.tenant_domains (id) ON DELETE CASCADE,
    token               VARCHAR(64) NOT NULL,
    expires_at          TIMESTAMPTZ NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_domain_challenges_domain ON domains.domain_verification_challenges (domain_id);

ALTER TABLE domains.tenant_domains ENABLE ROW LEVEL SECURITY;
ALTER TABLE domains.tenant_domains FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON domains.tenant_domains
    FOR SELECT
    USING (
        tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
        OR current_setting('app.bypass_rls', true) = 'true'
    );

CREATE POLICY tenant_insert ON domains.tenant_domains
    FOR INSERT
    WITH CHECK (
        tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
    );

CREATE POLICY tenant_update ON domains.tenant_domains
    FOR UPDATE
    USING (
        tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
        OR current_setting('app.bypass_rls', true) = 'true'
    );

ALTER TABLE domains.domain_verification_challenges ENABLE ROW LEVEL SECURITY;
ALTER TABLE domains.domain_verification_challenges FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_select ON domains.domain_verification_challenges
    FOR SELECT
    USING (
        domain_id IN (
            SELECT id FROM domains.tenant_domains
            WHERE tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
        )
        OR current_setting('app.bypass_rls', true) = 'true'
    );

CREATE POLICY tenant_insert ON domains.domain_verification_challenges
    FOR INSERT
    WITH CHECK (
        domain_id IN (
            SELECT id FROM domains.tenant_domains
            WHERE tenant_id = nullif(current_setting('app.tenant_id', true), '')::uuid
        )
        OR current_setting('app.bypass_rls', true) = 'true'
    );

GRANT USAGE ON SCHEMA domains TO app_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON domains.tenant_domains TO app_user;
GRANT SELECT, INSERT ON domains.domain_verification_challenges TO app_user;
